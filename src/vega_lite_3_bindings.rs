impl Into<vega_lite_3::RemovableValue<vega_lite_3::UrlData>> for crate::Data {
    fn into(self) -> vega_lite_3::RemovableValue<vega_lite_3::UrlData> {
        match self {
            crate::Data::Url(url) => vega_lite_3::RemovableValue::Specified(
                vega_lite_3::UrlDataBuilder::default()
                    .url(url)
                    .build()
                    .unwrap(),
            ),
        }
    }
}

impl Into<vega_lite_3::XClass> for crate::EncodingAxis {
    fn into(self) -> vega_lite_3::XClass {
        match self {
            crate::EncodingAxis::Field(field_name) => vega_lite_3::XClassBuilder::default()
                .field(field_name)
                .def_type(vega_lite_3::StandardType::Quantitative)
                .build()
                .unwrap(),
        }
    }
}
impl Into<vega_lite_3::YClass> for crate::EncodingAxis {
    fn into(self) -> vega_lite_3::YClass {
        match self {
            crate::EncodingAxis::Field(field_name) => vega_lite_3::YClassBuilder::default()
                .field(field_name)
                .def_type(vega_lite_3::StandardType::Quantitative)
                .build()
                .unwrap(),
        }
    }
}

impl Into<vega_lite_3::AnyMark> for crate::Mark {
    fn into(self) -> vega_lite_3::AnyMark {
        match self {
            crate::Mark::Line => vega_lite_3::Mark::Line.into(),
            crate::Mark::Point => vega_lite_3::Mark::Point.into(),
        }
    }
}

impl Into<vega_lite_3::DefWithConditionMarkPropFieldDefStringNull> for crate::EncodingCondition {
    fn into(self) -> vega_lite_3::DefWithConditionMarkPropFieldDefStringNull {
        match self {
            crate::EncodingCondition::Field(field_name) => {
                vega_lite_3::DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                    .field(vega_lite_3::Field::String(field_name))
                    .build()
                    .unwrap()
            }
        }
    }
}

impl crate::Procyon {
    /// Build the graph as a vega_lite_3 spec
    pub fn build_v3(&self) -> vega_lite_3::Vegalite {
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
