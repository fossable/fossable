use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Default)]
#[serde(rename = "svg")]
pub struct Svg {
    pub g: G,
    #[serde(rename = "@width")]
    pub width: String,
    #[serde(rename = "@height")]
    pub height: String,
}

impl Svg {
    pub fn write_to(&self, path: &str) -> Result<(), Box<dyn Error>> {
        std::fs::write(path, quick_xml::se::to_string(&self)?)?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        quick_xml::se::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Default)]
#[serde(rename = "g")]
pub struct G {
    pub rect: Vec<Rect>,
}

#[derive(Serialize, Default, Clone)]
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

impl Rect {
    pub fn to_svg(self) -> Svg {
        Svg {
            g: G {
                rect: vec![self.clone()],
            },
            width: self.width,
            height: self.height,
        }
    }
}
