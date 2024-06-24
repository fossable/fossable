use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "svg")]
pub struct Svg {
    pub g: Vec<G>,
    #[serde(rename = "@width")]
    pub width: String,
    #[serde(rename = "@height")]
    pub height: String,
    #[serde(rename = "@fill")]
    pub fill: Option<String>,
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,
    #[serde(rename = "@xmlns:xlink", skip_serializing_if = "Option::is_none")]
    pub xmlns_xlink: Option<String>,
}

impl Svg {
    pub fn write_to(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 2);

        self.serialize(ser)?;
        std::fs::write(path, &buffer)?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        quick_xml::se::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "g")]
pub struct G {
    #[serde(default)]
    pub rect: Vec<Rect>,
    #[serde(default)]
    pub path: Vec<Path>,
    #[serde(rename = "@transform")]
    pub transform: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "path")]
pub struct Path {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@style")]
    pub style: String,
    #[serde(rename = "@d")]
    pub d: String,
    #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
    pub stroke_linecap: Option<String>,
    #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
    pub stroke_linejoin: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "rect")]
pub struct Rect {
    #[serde(rename = "@style")]
    pub style: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@width")]
    pub width: String,
    #[serde(rename = "@height")]
    pub height: String,
    #[serde(rename = "@x")]
    pub x: String,
    #[serde(rename = "@y")]
    pub y: String,
    #[serde(rename = "@rx")]
    pub rx: String,
}
