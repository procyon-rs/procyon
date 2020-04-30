//! High level helpers to create beautiful graphs based on Vega-Lite

// warnings,
#![deny(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

pub use showata::Showable;
//
use base64;
use fantoccini::{Client, Locator};
use retry::{delay::Fixed, retry, OperationResult};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{thread, time};
#[cfg(feature = "vega_lite_4")]
mod vega_lite_4_bindings;

#[cfg(feature = "vega_lite_3")]
mod vega_lite_3_bindings;

/// Helpers to create a graph
#[derive(Debug)]
pub struct Procyon {
    data: Data,
    mark: Mark,
    encode_x: Option<EncodingAxis>,
    encode_y: Option<EncodingAxis>,
    encode_color: Option<EncodingCondition>,
}

// TODO: allow other sources of input
/// A Data source. This could be an URL to a csv file, a [`ndarray`](https://crates.io/crates/ndarray) array, ...
#[derive(Clone, Debug)]
pub enum Data {
    /// Data that is available at the given URL
    Url(String),
}
impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::Url(value.to_string())
    }
}

// TODO: allow encoding with field type (in altair: "field:Q")
// TODO: add explicit form for all options, with builder
/// Encoding information about how to take data for an axis of the graph
#[derive(Clone, Debug)]
pub enum EncodingAxis {
    /// Data comes from the named field for this axis
    Field(String),
}
impl From<&str> for EncodingAxis {
    fn from(value: &str) -> Self {
        EncodingAxis::Field(value.to_string())
    }
}

/// Encoding information about a condition
#[derive(Clone, Debug)]
pub enum EncodingCondition {
    /// Condition is based on the named field
    Field(String),
}
impl From<&str> for EncodingCondition {
    fn from(value: &str) -> Self {
        EncodingCondition::Field(value.to_string())
    }
}

#[derive(Clone, Debug)]
enum Mark {
    Line,
    Point,
}

impl Procyon {
    /// Create an basic chart based on the given data
    pub fn chart(data: impl Into<Data>) -> Self {
        Self {
            data: data.into(),
            mark: Mark::Line,
            encode_x: None,
            encode_y: None,
            encode_color: None,
        }
    }

    /// Create a scatterplot
    pub fn mark_point(&mut self) -> &mut Self {
        let mut new = self;
        new.mark = Mark::Point;

        new
    }

    /// Configure X/Y axis
    pub fn encode_axis<X: Into<EncodingAxis>, Y: Into<EncodingAxis>>(
        &mut self,
        x: X,
        y: Y,
    ) -> &mut Self {
        let new = self;
        new.encode_x = Some(x.into());
        new.encode_y = Some(y.into());

        new
    }

    /// Configure color for the graph
    pub fn encode_color<C: Into<EncodingCondition>>(&mut self, color: C) -> &mut Self {
        let new = self;
        new.encode_color = Some(color.into());

        new
    }
    /// save the image
    pub async fn save(&self, image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Spawn in background a webdriver
        // Currently geckodriver is hardcoded but should support
        // all webdriver compliant driver.
        // TODO:updated to all webdriver
        let mut webdriver_process = Command::new("geckodriver")
            .args(&["--port", "4444"])
            .spawn()
            .expect("webdriver failed to start");
        // This should also be TODO:updated to all webdriver
        let mut caps = serde_json::map::Map::new();
        let opts = serde_json::json!({ "args": ["--headless"]});
        caps.insert("moz:firefoxOptions".to_string(), opts);

        let mut connection_ok = false;
        while !connection_ok {
            thread::sleep(time::Duration::from_secs(5));
            // This should also be TODO:updated to all webdriver
            let mut caps = serde_json::map::Map::new();
            let opts = serde_json::json!({ "args": ["--headless"]});
            caps.insert("moz:firefoxOptions".to_string(), opts);
            let mut connection_ok = Client::with_capabilities("http://localhost:4444", caps)
                .await
                .is_err();
            if connection_ok == false {
                break;
            };
        }
        let mut c = Client::with_capabilities("http://localhost:4444", caps).await?;
        // let mut c = retry(Fixed::from_millis(100), || {
        //     match Some(c.as_ref().unwrap()) {
        //         Some(Client { tx, is_legacy }) => OperationResult::Ok(c),
        //         _ => OperationResult::Retry("not connected yet"),
        //     }
        // })
        // .unwrap()?;

        //.expect("failed to connect to WebDriver");

        // Current implem use the saved html by showata and imitate user clik to save the image
        // Another approach is to updated the embeded js like in altair:
        // https://github.com/altair-viz/altair_saver/blob/master/altair_saver/savers/_selenium.py
        dbg!(self.build().to_html_file()?.to_str().unwrap());
        c.goto(&format!(
            "file:///private{}",
            self.build().to_html_file()?.to_str().unwrap()
        ))
        .await?;
        // c.goto("file:///private/var/folders/32/h6lt_67s75g6jf3h4hx_myxc0000gn/T/showata/show-1587655243189631000.html").await?;
        let mut summary_button = c
            .wait_for_find(Locator::Css("summary"))
            .await?
            .click()
            .await?;
        let mut hidden_link = summary_button
            .find(Locator::LinkText("Save as PNG"))
            .await?
            .click()
            .await?;

        let link = hidden_link
            .find(Locator::LinkText("Save as PNG"))
            .await?
            .attr("href")
            .await?;

        let image_data_url = link.unwrap();
        //let data64: Vec<&str> = image_data_url.split(',').collect::<Vec<&str>>();
        //dbg!(data64[1]);
        //let bytes: Vec<u8> = base64::decode(data64[1]).unwrap();
        // "png;base64,iVB"
        let _format = &image_data_url[11..14];
        let bytes: Vec<u8> = base64::decode(&image_data_url[22..]).unwrap();
        let mut image_file = File::create(image_path).unwrap();
        image_file.write(&bytes).unwrap();
        hidden_link.close().await;
        webdriver_process.kill();
        Ok(())
    }
    /// Build the graph
    #[cfg(feature = "vega_lite_4")]
    pub fn build(&self) -> vega_lite_4::Vegalite {
        self.build_v4()
    }

    /// Build the graph
    #[cfg(all(not(feature = "vega_lite_4"), feature = "vega_lite_3"))]
    pub fn build(&self) -> vega_lite_3::Vegalite {
        self.build_v3()
    }
}
