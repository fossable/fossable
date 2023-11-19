use clap::{Parser, Subcommand};
use imagery::Logo;
use std::error::Error;

pub mod imagery;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate imagery
    Imagery { name: String },
}

pub fn fossable_word() -> [String; 7] {
    [
        "                                      ◼       ◼        ".into(),
        "◼ ◼                                   ◼       ◼        ".into(),
        "◼     ◼ ◼ ◼     ◼ ◼     ◼ ◼     ◼ ◼   ◼ ◼ ◼   ◼     ◼ ◼".into(),
        "◼ ◼   ◼   ◼     ◼       ◼     ◼   ◼   ◼   ◼   ◼   ◼ ◼  ".into(),
        "◼     ◼ ◼ ◼   ◼ ◼     ◼ ◼       ◼ ◼   ◼ ◼ ◼   ◼     ◼ ◼".into(),
        "                                                       ".into(),
        "                                                       ".into(),
    ]
}

pub fn goldboot_word() -> [String; 7] {
    [
        "                ◼       ◼   ◼                       ◼  ".into(),
        "                ◼       ◼   ◼                       ◼ ◼".into(),
        "◼ ◼ ◼   ◼ ◼ ◼   ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼  ".into(),
        "◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼  ".into(),
        "◼ ◼ ◼   ◼ ◼ ◼   ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼".into(),
        "    ◼                                                  ".into(),
        "◼ ◼ ◼                                                  ".into(),
    ]
}

pub fn gantry_word() -> [String; 7] {
    [
        "                        ◼                ".into(),
        "                        ◼ ◼              ".into(),
        "◼ ◼ ◼     ◼ ◼   ◼ ◼ ◼   ◼     ◼ ◼   ◼   ◼".into(),
        "◼   ◼   ◼   ◼   ◼   ◼   ◼     ◼     ◼   ◼".into(),
        "◼ ◼ ◼     ◼ ◼   ◼   ◼   ◼ ◼   ◼     ◼ ◼ ◼".into(),
        "    ◼                                   ◼".into(),
        "◼ ◼ ◼                               ◼ ◼ ◼".into(),
    ]
}

pub fn sandpolis_word() -> [String; 7] {
    [
        "                            ◼                   ◼            ".into(),
        "                            ◼                   ◼            ".into(),
        "  ◼ ◼     ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼   ◼     ◼ ◼".into(),
        "  ◼     ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼   ◼     ◼  ".into(),
        "◼ ◼       ◼ ◼   ◼   ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼ ◼ ◼   ◼   ◼   ◼ ◼  ".into(),
        "                                ◼                            ".into(),
        "                                ◼                            ".into(),
    ]
}

pub fn turbine_word() -> [String; 7] {
    [
        "◼                   ◼                        ".into(),
        "◼ ◼                 ◼                        ".into(),
        "◼     ◼   ◼   ◼ ◼   ◼ ◼ ◼   ◼   ◼ ◼ ◼     ◼ ◼".into(),
        "◼     ◼   ◼   ◼     ◼   ◼   ◼   ◼   ◼   ◼ ◼  ".into(),
        "◼ ◼   ◼ ◼ ◼   ◼     ◼ ◼ ◼   ◼   ◼   ◼     ◼ ◼".into(),
        "                                             ".into(),
        "                                             ".into(),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Imagery { name } => {
            let logo = match name.as_str() {
                "fossable" => Logo {
                    matrix: fossable_word(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#413577",
                    bg_style: Some("fill:#333333"),
                },
                "goldboot" => Logo {
                    matrix: goldboot_word(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#c8ab37",
                    bg_style: Some("fill:#333333"),
                },
                "gantry" => Logo {
                    matrix: gantry_word(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#378B2E",
                    bg_style: Some("fill:#333333"),
                },
                "sandpolis" => Logo {
                    matrix: sandpolis_word(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#AC4F40",
                    bg_style: Some("fill:#333333"),
                },
                "turbine" => Logo {
                    matrix: turbine_word(),
                    margin_px: 7,
                    rect_side_px: 7,
                    rect_gap_px: 1,
                    rect_style: "fill:#378B2E",
                    bg_style: Some("fill:#333333"),
                },
                _ => todo!(),
            };

            // Save SVG
            logo.to_svg().write_to(&format!("{}.svg", name))?;

            // Save PNGs
            // TODO
            // inkscape --export-type png -h ${height} -o output/icon-${height}.png output/icon.svg
        }
    };
    Ok(())
}
