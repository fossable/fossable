use super::Project;

pub struct Sandpolis;

impl Project for Sandpolis {
    fn homepage() -> &'static str {
        todo!()
    }

    fn icon() -> &'static str {
        include_str!("../../icons/sandpolis.svg")
    }

    fn name() -> &'static str {
        "sandpolis"
    }

    fn primary_color() -> &'static str {
        todo!()
    }

    fn public_key() -> &'static str {
        todo!()
    }

    fn word() -> &'static str {
        concat!(
            "              ◼         ◼      ",
            "              ◼         ◼      ",
            " ◼◼  ◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼  ◼◼",
            " ◼  ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼  ◼ ",
            "◼◼   ◼◼ ◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼ ◼◼ ",
            "                ◼              ",
            "                ◼              ",
        )
    }
}
