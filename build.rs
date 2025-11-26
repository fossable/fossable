fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "build-graphics")]
    build_graphics();
}

#[cfg(feature = "build-graphics")]
fn build_graphics() {
    use svg::emblem::Emblem;

    // Rerun if any icon files change
    for entry in std::fs::read_dir("icons").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    for bg_style in [true, false] {
        for (project, emblem) in [
            #[cfg(feature = "project-goldboot")]
            (
                "goldboot",
                Emblem {
                    word: [
                        "        ◼   ◼ ◼           ◼ ",
                        "        ◼   ◼ ◼           ◼◼",
                        "◼◼◼ ◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ",
                        "◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ",
                        "◼◼◼ ◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼",
                        "  ◼                         ",
                        "◼◼◼                         ",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#c8ab37",
                    icon_style: "fill:#c8ab37",
                    icon: include_str!("icons/goldboot.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
            #[cfg(feature = "project-gantry")]
            (
                "gantry",
                Emblem {
                    word: [
                        "            ◼        ",
                        "            ◼◼       ",
                        "◼◼◼  ◼◼ ◼◼◼ ◼  ◼◼ ◼ ◼",
                        "◼ ◼ ◼ ◼ ◼ ◼ ◼  ◼  ◼ ◼",
                        "◼◼◼  ◼◼ ◼ ◼ ◼◼ ◼  ◼◼◼",
                        "  ◼                 ◼",
                        "◼◼◼               ◼◼◼",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#248467",
                    icon_style: "fill:#248467",
                    icon: include_str!("icons/gantry.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
            #[cfg(feature = "project-sandpolis")]
            (
                "sandpolis",
                Emblem {
                    word: [
                        "              ◼         ◼      ",
                        "              ◼         ◼      ",
                        " ◼◼  ◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼  ◼◼",
                        " ◼  ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼  ◼ ",
                        "◼◼   ◼◼ ◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼ ◼◼ ",
                        "                ◼              ",
                        "                ◼              ",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#c89437",
                    icon_style: "fill:#c89437",
                    icon: include_str!("icons/sandpolis.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
            #[cfg(feature = "project-turbine")]
            (
                "turbine",
                Emblem {
                    word: [
                        "◼         ◼            ",
                        "◼◼        ◼            ",
                        "◼  ◼ ◼ ◼◼ ◼◼◼ ◼ ◼◼◼  ◼◼",
                        "◼  ◼ ◼ ◼  ◼ ◼ ◼ ◼ ◼ ◼◼ ",
                        "◼◼ ◼◼◼ ◼  ◼◼◼ ◼ ◼ ◼  ◼◼",
                        "                       ",
                        "                       ",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#378B2E",
                    icon_style: "stroke:#378B2E",
                    icon: include_str!("icons/turbine.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
            #[cfg(feature = "project-outpost")]
            (
                "outpost",
                Emblem {
                    word: [
                        "        ◼              ◼ ",
                        "        ◼◼             ◼◼",
                        "◼◼◼ ◼ ◼ ◼  ◼◼◼ ◼◼◼  ◼◼ ◼ ",
                        "◼ ◼ ◼ ◼ ◼  ◼ ◼ ◼ ◼  ◼  ◼ ",
                        "◼◼◼ ◼◼◼ ◼◼ ◼◼◼ ◼◼◼ ◼◼  ◼◼",
                        "           ◼             ",
                        "           ◼             ",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#c85037",
                    icon_style: "stroke:#c85037",
                    icon: include_str!("icons/outpost.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
            #[cfg(feature = "project-workset")]
            (
                "workset",
                Emblem {
                    word: [
                        "                         ◼ ",
                        "                         ◼◼",
                        "◼   ◼ ◼◼◼ ◼◼ ◼ ◼  ◼◼  ◼◼ ◼ ",
                        "◼ ◼ ◼ ◼ ◼ ◼  ◼◼   ◼  ◼◼  ◼ ",
                        "◼◼ ◼◼ ◼◼◼ ◼  ◼ ◼ ◼◼   ◼◼ ◼◼",
                        "                           ",
                        "                           ",
                    ],
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#3776c8",
                    icon_style: "fill:#3776c8;stroke:#3776c8",
                    icon: include_str!("icons/workset.svg").to_string(),
                    icon_width: Some(50),
                    bg_style: if bg_style { Some("fill:#333333") } else { None },
                },
            ),
        ] {
            let emblem_svg = emblem.to_svg().unwrap();
            let path = format!("{manifest_dir}/emblems/{project}.svg");

            // Write svg
            emblem_svg.write_to(&path).unwrap();

            // Write emblem rasters in varying sizes
            for (width, height) in [(256, 64), (512, 128), (1024, 256), (2048, 512)] {
                let path = format!("{manifest_dir}/emblems/{project}-{width}.png");
                emblem_svg.rasterize(&path, width, height).unwrap();
            }

            // Write icon rasters in varying sizes
            // for height in [64, 128, 256, 512] {
            //     let path = format!("{manifest_dir}/{project}.svg");
            //     if !std::fs::exists(&path).unwrap() {
            //         icon_svg.rasterize(&path, height, height).unwrap();
            //     }
            // }
        }
    }
}

#[cfg(feature = "build-graphics")]
pub mod svg {
    use quick_xml::se::Serializer;
    use resvg::tiny_skia::{Pixmap, Transform};
    use serde::{Deserialize, Serialize};
    use std::{error::Error, path::Path};

    /// Implements a small subset of the SVG spec for our needs.
    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename = "svg")]
    pub struct Svg {
        pub g: Vec<SvgGroup>,
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

        /// Convert into PNG and write to the given path.
        pub fn rasterize<P>(&self, path: P, width: u32, height: u32) -> Result<(), Box<dyn Error>>
        where
            P: AsRef<Path>,
        {
            let svg = usvg::Tree::from_str(&self.to_string(), &usvg::Options::default())?;
            let mut pixmap = Pixmap::new(width, height).expect("invalid size");

            // Calculate scale to fit the SVG into the target dimensions
            let svg_size = svg.size();
            let scale = width as f32 / svg_size.width();

            let transform = Transform::from_scale(scale, scale);
            resvg::render(&svg, transform, &mut pixmap.as_mut());
            pixmap.save_png(path.as_ref())?;
            Ok(())
        }
    }

    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename = "g")]
    pub struct SvgGroup {
        #[serde(default)]
        pub path: Vec<SvgPath>,
        #[serde(default)]
        pub rect: Vec<SvgRect>,
        #[serde(default)]
        pub ellipse: Vec<SvgEllipse>,
        #[serde(default)]
        pub circle: Vec<SvgCircle>,
        #[serde(rename = "@transform")]
        pub transform: Option<String>,
    }

    #[derive(Serialize, Deserialize, Default, Clone)]
    #[serde(rename = "path")]
    pub struct SvgPath {
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
    pub struct SvgRect {
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

    #[derive(Serialize, Deserialize, Default, Clone)]
    #[serde(rename = "ellipse")]
    pub struct SvgEllipse {
        #[serde(rename = "@cx")]
        pub cx: String,
        #[serde(rename = "@cy")]
        pub cy: String,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@rx")]
        pub rx: String,
        #[serde(rename = "@ry")]
        pub ry: String,
        #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
        pub stroke: Option<String>,
        #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
        pub stroke_linecap: Option<String>,
        #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
        pub stroke_linejoin: Option<String>,
        #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
        pub stroke_width: Option<String>,
        #[serde(rename = "@style")]
        pub style: Option<String>,
    }

    #[derive(Serialize, Deserialize, Default, Clone)]
    #[serde(rename = "circle")]
    pub struct SvgCircle {
        #[serde(rename = "@cx")]
        pub cx: String,
        #[serde(rename = "@cy")]
        pub cy: String,
        #[serde(rename = "@id")]
        pub id: String,
        #[serde(rename = "@r")]
        pub rx: String,
        #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
        pub stroke: Option<String>,
        #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
        pub stroke_linecap: Option<String>,
        #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
        pub stroke_linejoin: Option<String>,
        #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
        pub stroke_width: Option<String>,
        #[serde(rename = "@style")]
        pub style: Option<String>,
    }

    pub mod emblem {
        use super::*;

        /// Each project has a unique emblem which is an icon plus block text.
        pub struct Emblem {
            pub word: [&'static str; 7],
            pub margin_px: usize,
            pub rect_side_px: usize,
            pub rect_gap_px: usize,
            pub rect_style: &'static str,
            pub icon_style: &'static str,
            pub icon: String,
            pub icon_width: Option<usize>,
            pub bg_style: Option<&'static str>,
        }

        impl Emblem {
            pub fn to_svg(&self) -> Result<Svg, Box<dyn Error>> {
                let mut svg = Svg::default();
                svg.fill = Some("none".into());

                // Add SVG namespaces
                svg.xmlns = Some("http://www.w3.org/2000/svg".to_string());
                svg.xmlns_xlink = Some("http://www.w3.org/1999/xlink".to_string());

                // Add background
                if let Some(bg_style) = self.bg_style {
                    svg.g.push(SvgGroup {
                        rect: vec![SvgRect {
                            style: String::from(bg_style),
                            id: format!("background"),
                            width: format!("100%"),
                            height: format!("100%"),
                            x: format!(""),
                            y: format!(""),
                            rx: String::from("3%"),
                        }],
                        ..Default::default()
                    });
                }

                // Add icon
                if self.icon_width.is_some() {
                    let icon: Svg = quick_xml::de::from_str(self.icon.as_str())?;

                    let icon_group = icon.g.first().unwrap();

                    // Position the icon
                    svg.g.push(SvgGroup {
                        path: icon_group
                            .path
                            .iter()
                            .map(|p| {
                                // Replace style
                                let mut p = p.clone();
                                p.style = self.icon_style.to_string();
                                p.clone()
                            })
                            .collect(),
                        rect: icon_group
                            .rect
                            .iter()
                            .map(|r| {
                                // Replace style
                                let mut r = r.clone();
                                r.style = self.icon_style.to_string();
                                r.clone()
                            })
                            .collect(),
                        ellipse: icon_group
                            .ellipse
                            .iter()
                            .map(|e| {
                                // Replace style
                                let mut e = e.clone();
                                e.style = Some(self.icon_style.to_string());
                                e.clone()
                            })
                            .collect(),
                        circle: icon_group
                            .circle
                            .iter()
                            .map(|c| {
                                // Replace style
                                let mut c = c.clone();
                                c.style = Some(self.icon_style.to_string());
                                c.clone()
                            })
                            .collect(),
                        transform: Some(format!(
                            "translate({},{})",
                            self.margin_px / 2,
                            self.margin_px / 2
                        )),
                        ..Default::default()
                    });
                }

                // Add letters
                svg.g.push(SvgGroup {
                    rect: self.generate_rects(),
                    ..Default::default()
                });

                // Set image dimensions
                svg.width = format!(
                    "{}",
                    (self.margin_px * 2)
                        + self.icon_width.unwrap_or(0)
                        + ((self.word.first().unwrap().chars().count() - 1)
                            * (self.rect_side_px + self.rect_gap_px))
                        + self.rect_side_px
                        - self.total_horizontal_adjust()
                );
                svg.height = format!(
                    "{}",
                    (self.margin_px * 2)
                        + ((self.word.len() - 1) * (self.rect_side_px + self.rect_gap_px))
                        + self.rect_side_px
                );

                return Ok(svg);
            }

            fn generate_rects(&self) -> Vec<SvgRect> {
                let mut rects = Vec::new();

                // Makes the horizontal spacing a little bit nicer
                let mut horizontal_adjust = 0;

                for c in 0..self.word.first().unwrap().chars().count() {
                    // Track empty columns
                    let mut empty = true;
                    for r in 0..self.word.len() {
                        if self.word[r].chars().nth(c).unwrap() != ' ' {
                            empty = false;
                            rects.push(SvgRect {
                                style: String::from(self.rect_style),
                                id: format!("{r}-{c}"),
                                width: format!("{}", self.rect_side_px),
                                height: format!("{}", self.rect_side_px),
                                x: format!(
                                    "{}",
                                    // Leave some space for the margin
                                    self.margin_px
                                // Leave some space for the icon if present
                                + self.icon_width.unwrap_or(0)
                                + (c * self.rect_side_px + c * self.rect_gap_px)
                                // Remove some space between each letter
                                - horizontal_adjust
                                ),
                                y: format!(
                                    "{}",
                                    self.margin_px + (r * self.rect_side_px + r * self.rect_gap_px)
                                ),
                                rx: String::from("1"),
                            });
                        }
                    }

                    if empty {
                        horizontal_adjust += 3;
                    }
                }

                return rects;
            }

            fn total_horizontal_adjust(&self) -> usize {
                // Makes the horizontal spacing a little bit nicer
                let mut horizontal_adjust = 0;

                for c in 0..self.word.first().unwrap().chars().count() {
                    // Track empty columns
                    let mut empty = true;
                    for r in 0..self.word.len() {
                        if self.word[r].chars().nth(c).unwrap() != ' ' {
                            empty = false;
                        }
                    }

                    if empty {
                        horizontal_adjust += 3;
                    }
                }
                return horizontal_adjust;
            }
        }
    }
}
