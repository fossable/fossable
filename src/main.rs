use clap::{Parser, Subcommand};
use emblem::Emblem;
use std::error::Error;

pub mod emblem;
pub mod svg;
pub mod words;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate project emblems
    Emblem {
        name: String,
        #[clap(long, action)]
        bg: bool,
        #[clap(long, action)]
        icon: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Emblem { name, bg, icon } => {
            let logo = match name.as_str() {
                "fossable" => Emblem {
                    matrix: words::fossable(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#413577",
                    bg_style: if *bg { Some("fill:#333333") } else { None },
                    icon_width: if *icon { Some(50) } else { None },
                    icon: include_str!("../icons/fossable.svg").to_string(),
                },
                "goldboot" => Emblem {
                    matrix: words::goldboot(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#c8ab37",
                    bg_style: if *bg { Some("fill:#333333") } else { None },
                    icon_width: if *icon { Some(40) } else { None },
                    icon: include_str!("../icons/goldboot.svg").to_string(),
                },
                "gantry" => Emblem {
                    matrix: words::gantry(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#378B2E",
                    bg_style: if *bg { Some("fill:#333333") } else { None },
                    icon_width: if *icon { Some(50) } else { None },
                    icon: include_str!("../icons/gantry.svg").to_string(),
                },
                "sandpolis" => Emblem {
                    matrix: words::sandpolis(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#AC4F40",
                    bg_style: if *bg { Some("fill:#333333") } else { None },
                    icon_width: if *icon { Some(50) } else { None },
                    icon: include_str!("../icons/sandpolis.svg").to_string(),
                },
                "turbine" => Emblem {
                    matrix: words::turbine(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#378B2E",
                    bg_style: if *bg { Some("fill:#333333") } else { None },
                    icon_width: if *icon { Some(50) } else { None },
                    icon: include_str!("../icons/turbine.svg").to_string(),
                },
                _ => todo!(),
            };

            // Save SVG
            logo.to_svg()?.write_to(&format!("{}.svg", name))?;

            // Save PNGs
            // TODO
            // inkscape --export-type png -h ${height} -o output/icon-${height}.png output/icon.svg
        }
    };
    Ok(())
}
