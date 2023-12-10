use crate::svg::*;

/// Each project has a unique emblem which is an icon plus block text.
pub struct Emblem {
    pub matrix: [&'static str; 7],
    pub margin_px: usize,
    pub rect_side_px: usize,
    pub rect_gap_px: usize,
    pub rect_style: &'static str,
    pub bg_style: Option<&'static str>,
    pub icon_width: Option<usize>,
    pub icon: String,
}

impl Emblem {
    pub fn to_svg(&self) -> Svg {
        let mut svg = Svg::default();

        // Add background
        if let Some(bg_style) = self.bg_style {
            svg.g.rect.push(Rect {
                style: String::from(bg_style),
                id: format!("background"),
                width: format!("100%"),
                height: format!("100%"),
                x: format!(""),
                y: format!(""),
                rx: String::from("3%"),
            });
        }

        // Add letters
        for rect in self.generate_rects() {
            svg.g.rect.push(rect);
        }

        // Set image dimensions
        svg.width = format!(
            "{}",
            (self.margin_px * 2)
                + ((self.matrix.first().unwrap().len() - 1)
                    * (self.rect_side_px + self.rect_gap_px))
                + self.rect_side_px
                - self.total_horizontal_adjust()
        );
        svg.height = format!(
            "{}",
            (self.margin_px * 2)
                + ((self.matrix.len() - 1) * (self.rect_side_px + self.rect_gap_px))
                + self.rect_side_px
        );

        return svg;
    }

    fn generate_rects(&self) -> Vec<Rect> {
        let mut rects = Vec::new();

        // Makes the horizontal spacing a little bit nicer
        let mut horizontal_adjust = 0;

        for c in 0..self.matrix.first().unwrap().chars().count() {
            // Track empty columns
            let mut empty = true;
            for r in 0..self.matrix.len() {
                if self.matrix[r].chars().nth(c).unwrap() != ' ' {
                    empty = false;
                    rects.push(Rect {
                        style: String::from(self.rect_style),
                        id: format!("{r}-{c}"),
                        width: format!("{}", self.rect_side_px),
                        height: format!("{}", self.rect_side_px),
                        x: format!(
                            "{}",
                            self.margin_px + (c * self.rect_side_px + c * self.rect_gap_px)
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

        for c in 0..self.matrix.first().unwrap().chars().count() {
            // Track empty columns
            let mut empty = true;
            for r in 0..self.matrix.len() {
                if self.matrix[r].chars().nth(c).unwrap() != ' ' {
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
