use vega_lite_3;
pub use vega_lite_3::Showable;

pub struct Procyon {
    data: Data,
    mark: vega_lite_3::Mark,
    encode_x: Option<EncodingAxis>,
    encode_y: Option<EncodingAxis>,
    encode_color: Option<EncodingCondition>,
}

// TODO: allow other sources of input
#[derive(Clone)]
pub enum Data {
    Url(String),
}
impl Into<vega_lite_3::Data> for Data {
    fn into(self) -> vega_lite_3::Data {
        match self {
            Data::Url(url) => vega_lite_3::DataBuilder::default()
                .url(url)
                .build()
                .unwrap(),
        }
    }
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
impl Into<vega_lite_3::XClass> for EncodingAxis {
    fn into(self) -> vega_lite_3::XClass {
        match self {
            EncodingAxis::Field(field_name) => vega_lite_3::XClassBuilder::default()
                .field(field_name)
                .def_type(vega_lite_3::StandardType::Quantitative)
                .build()
                .unwrap(),
        }
    }
}
impl Into<vega_lite_3::YClass> for EncodingAxis {
    fn into(self) -> vega_lite_3::YClass {
        match self {
            EncodingAxis::Field(field_name) => vega_lite_3::YClassBuilder::default()
                .field(field_name)
                .def_type(vega_lite_3::StandardType::Quantitative)
                .build()
                .unwrap(),
        }
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
impl Into<vega_lite_3::ValueDefWithConditionMarkPropFieldDefStringNull> for EncodingCondition {
    fn into(self) -> vega_lite_3::ValueDefWithConditionMarkPropFieldDefStringNull {
        match self {
            EncodingCondition::Field(field_name) => {
                vega_lite_3::ValueDefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .field(vega_lite_3::Field::String(field_name))
                    .build()
                    .unwrap()
            }
        }
    }
}

impl Procyon {
    pub fn chart(data: impl Into<Data>) -> Self {
        Self {
            data: data.into(),
            mark: vega_lite_3::Mark::Line,
            encode_x: None,
            encode_y: None,
            encode_color: None,
        }
    }

    pub fn mark_point(&mut self) -> &mut Self {
        let mut new = self;
        new.mark = vega_lite_3::Mark::Point;

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

    pub fn build(&self) -> vega_lite_3::Vegalite {
        let mut builder = &mut vega_lite_3::VegaliteBuilder::default();
        builder = builder.data(self.data.clone()).mark(self.mark.clone());
        let mut encoding_builder = &mut vega_lite_3::EncodingBuilder::default();
        if let Some(x) = self.encode_x.clone() {
            encoding_builder = encoding_builder.x(x);
        }
        if let Some(y) = self.encode_y.clone() {
            encoding_builder = encoding_builder.y(y);
        }
        if let Some(color) = self.encode_color.clone() {
            encoding_builder = encoding_builder.color(color);
        }
        builder = builder.encoding(encoding_builder.build().unwrap());

        builder.build().unwrap()
    }
}
