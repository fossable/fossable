/// Each project has a unique emblem which is an icon plus block text.
pub struct Emblem {
    pub name: &'static str,
    pub word: [&'static str; 7],
    pub margin_px: usize,
    pub rect_side_px: usize,
    pub rect_gap_px: usize,
    pub color: &'static str,
    pub icon: &'static str,
    pub icon_width: Option<usize>,
}

#[cfg(feature = "project-goldboot")]
pub const GOLDBOOT: Emblem = Emblem {
    name: "goldboot",
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
    color: "#c8ab37",
    icon: include_str!("../icons/goldboot.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-gantry")]
pub const GANTRY: Emblem = Emblem {
    name: "gantry",
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
    color: "#248467",
    icon: include_str!("../icons/gantry.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-sandpolis")]
pub const SANDPOLIS: Emblem = Emblem {
    name: "sandpolis",
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
    color: "#c89437",
    icon: include_str!("../icons/sandpolis.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-turbine")]
pub const TURBINE: Emblem = Emblem {
    name: "turbine",
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
    color: "#378B2E",
    icon: include_str!("../icons/turbine.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-outpost")]
pub const OUTPOST: Emblem = Emblem {
    name: "outpost",
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
    color: "#c85037",
    icon: include_str!("../icons/outpost.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-workset")]
pub const WORKSET: Emblem = Emblem {
    name: "workset",
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
    color: "#3776c8",
    icon: include_str!("../icons/workset.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-common-ci")]
pub const COMMON_CI: Emblem = Emblem {
    name: "common-ci",
    word: [
        "                                    ",
        "                                    ",
        "◼◼◼ ◼◼◼ ◼◼◼◼◼ ◼◼◼◼◼ ◼◼◼ ◼◼◼   ◼◼◼ ◼ ",
        "◼   ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼   ◼   ◼ ",
        "◼◼◼ ◼◼◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼◼◼ ◼ ◼   ◼◼◼ ◼ ",
        "                                    ",
        "                                    ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#37c88b",
    icon: include_str!("../icons/common-ci.svg"),
    icon_width: Some(50),
};

#[cfg(feature = "project-fossdb")]
pub const FOSSDB: Emblem = Emblem {
    name: "fossdb",
    word: [
        " ◼◼              ◼ ◼   ",
        " ◼               ◼ ◼   ",
        "◼◼◼ ◼◼◼  ◼◼ ◼◼ ◼◼◼ ◼◼◼ ",
        " ◼  ◼ ◼  ◼  ◼  ◼ ◼ ◼ ◼ ",
        " ◼  ◼◼◼ ◼◼ ◼◼  ◼◼◼ ◼◼◼ ",
        "                       ",
        "                       ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#37c88b",
    icon: include_str!("../icons/fossdb.svg"),
    icon_width: Some(50),
};
