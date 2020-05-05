//! High level helpers to create beautiful graphs based on Vega-Lite

#![deny(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    warnings,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

pub use showata::Showable;

use base64;
use data_url::DataUrl;
use fantoccini::{Client, Locator};
use retry::{delay::Exponential, retry_with_index, OperationResult};
//use futures_retry::{RetryPolicy, StreamRetryExt};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Spawn in background a webdriver, currently support is limited to
/// geckodriver. Please see [geckodriver doc](https://github.com/mozilla/geckodriver) and install it.
/// TODO: allow [chromium webdriver](https://chromedriver.chromium.org/) and update documentation
pub fn spawn_webdriver(
    webdriver_name: &str,
    port: Option<u64>,
) -> Result<(std::process::Child, u64), Box<dyn std::error::Error>> {
    let mut try_port = match port {
        Some(n) => n,
        None => 4444,
    };
    let webdriver_process = retry_with_index(Exponential::from_millis(100), |current_try| {
        if current_try > 3 {
            return OperationResult::Err("did not succeed within 3 tries");
        }
        try_port += current_try;
        let try_command = Command::new(webdriver_name)
            .args(&["--port", &try_port.to_string()])
            .spawn();
        match try_command {
            Ok(cmd) => OperationResult::Ok(cmd),
            Err(_) => OperationResult::Retry("Trying with another port"),
        }
    })
    .unwrap();
    Ok((webdriver_process, try_port))
}

/// Create a headless browser instance.
/// Code from : https://github.com/jonhoo/fantoccini/blob/master/tests/common.rs
/// The chrome case will be commented for now and will be tested later.str
/// It also need the port from the webdriver.
pub async fn create_headless_client(
    client_type: &str,
    port: u64,
) -> Result<Client, fantoccini::error::NewSessionError> {
    //    let mut client = retry_with_index(Exponential::from_millis(100),
    // |current_try| {        if current_try > 5 {
    //            return OperationResult::Err("did not succeed within 3 tries");
    //        }
    //        let mut try_client = match client_type {
    //            "firefox" => {
    //                let mut caps = serde_json::map::Map::new();
    //                let opts = serde_json::json!({ "args": ["--headless"] });
    //                caps.insert("moz:firefoxOptions".to_string(), opts.clone());
    //                Client::with_capabilities(&format!("http://localhost:{}", port.to_string()), caps)
    //                    .await?
    //            }
    //            browser => unimplemented!("unsupported browser backend {}",
    // browser),        };
    //        match try_client {
    //            Ok(try_client) => OperationResult::Ok(try_client),
    //            Err(e) => OperationResult::Retry("Trying to establish connection
    // between client and webdriver"),        }
    //    }).unwrap();
    //  Ok(client)
    unimplemented!(
        "Currently trying to find out how retry_futures works {} {}",
        client_type,
        port
    )
}

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

    /// Current implem use the saved html by showata and imitate user clik to
    /// save the image Another approach is to updated the embeded js like in
    /// altair: https://github.com/altair-viz/altair_saver/blob/master/altair_saver/savers/_selenium.py
    pub async fn save(&self, image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let (mut webdriver_process, webdriver_port) =
            spawn_webdriver("geckodriver", Some(4444)).unwrap();
        let mut client = create_headless_client("firefox", webdriver_port).await?;
        client
            .goto(&format!(
                "file:///private{}",
                self.build().to_html_file()?.to_str().unwrap()
            ))
            .await?;

        let mut summary_button = client
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
            .await?
            .unwrap();

        let image_data_url = DataUrl::process(&link).unwrap();
        let (body, _) = image_data_url.decode_to_vec().unwrap();
        let bytes: Vec<u8> = base64::decode(&body).unwrap();
        let mut image_file = File::create(image_path).unwrap();
        image_file.write(&bytes).unwrap();
        hidden_link.close().await?;
        webdriver_process.kill()?;
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
