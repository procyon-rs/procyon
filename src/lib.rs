pub use showata::Showable;

#[cfg(feature = "vega_lite_4")]
mod vega_lite_4_bindings;

#[cfg(feature = "vega_lite_3")]
mod vega_lite_3_bindings;

pub struct Procyon {
    data: Data,
    mark: Mark,
    encode_x: Option<EncodingAxis>,
    encode_y: Option<EncodingAxis>,
    encode_color: Option<EncodingCondition>,
}

// TODO: allow other sources of input
#[derive(Clone)]
pub enum Data {
    Url(String),
}
impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::Url(value.to_string())
    }
}

// TODO: allow encoding with field type (in altair: "field:Q")
// TODO: add explicit form for all options, with builder
#[derive(Clone)]
pub enum EncodingAxis {
    Field(String),
}
impl From<&str> for EncodingAxis {
    fn from(value: &str) -> Self {
        EncodingAxis::Field(value.to_string())
    }
}

#[derive(Clone)]
pub enum EncodingCondition {
    Field(String),
}
impl From<&str> for EncodingCondition {
    fn from(value: &str) -> Self {
        EncodingCondition::Field(value.to_string())
    }
}

#[derive(Clone)]
enum Mark {
    Line,
    Point,
}

impl Procyon {
    pub fn chart(data: impl Into<Data>) -> Self {
        Self {
            data: data.into(),
            mark: Mark::Line,
            encode_x: None,
            encode_y: None,
            encode_color: None,
        }
    }

    pub fn mark_point(&mut self) -> &mut Self {
        let mut new = self;
        new.mark = Mark::Point;

        new
    }

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

    pub fn encode_color<C: Into<EncodingCondition>>(&mut self, color: C) -> &mut Self {
        let new = self;
        new.encode_color = Some(color.into());

        new
    }

    #[cfg(feature = "vega_lite_4")]
    pub fn build(&self) -> vega_lite_4::Vegalite {
        self.build_v4()
    }

    #[cfg(all(not(feature = "vega_lite_4"), feature = "vega_lite_3"))]
    pub fn build(&self) -> vega_lite_3::Vegalite {
        self.build_v3()
    }
}
