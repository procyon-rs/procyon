//! High level helpers to create beautiful graphs based on Vega-Lite

#![deny(
    warnings,
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
