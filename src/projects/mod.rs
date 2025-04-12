pub mod sandpolis;

pub trait Project {
    /// Project homepage URL.
    fn homepage() -> &'static str;

    /// Project icon as an SVG.
    fn icon() -> &'static str;

    /// Project name (lowercase).
    fn name() -> &'static str;

    /// Project color (hex).
    fn primary_color() -> &'static str;

    /// Project public key for verifying artifact signatures.
    fn public_key() -> &'static str;

    /// Project name in block letters.
    fn word() -> &'static str;
}
