pub(crate) use crate::common::*;

pub(crate) struct Chart {
    x_range: [f32; 2],
    y_range: [f32; 2],
    h_lines: u8,
    v_lines: u8,
}

impl Chart {
    pub(crate) fn new() -> Chart {
        let x_range = [0.0; 2];
        let y_range = [0.0; 2];
        let h_lines = 0;
        let v_lines = 0;

        Chart {
            x_range,
            y_range,
            h_lines,
            v_lines,
        }
    }

    pub(crate) fn x_range<'a>(&'a mut self, x0: f32, x1: f32) -> &'a mut Chart {
        self.x_range[0] = x0;
        self.x_range[1] = x1;
        self
    }

    pub(crate) fn y_range<'a>(&'a mut self, y0: f32, y1: f32) -> &'a mut Chart {
        self.y_range[0] = y0;
        self.y_range[1] = y1;
        self
    }

    pub(crate) fn h_lines<'a>(&'a mut self, n: u8) -> &'a mut Chart {
        self.h_lines = n;
        self
    }

    pub(crate) fn v_lines<'a>(&'a mut self, n: u8) -> &'a mut Chart {
        self.v_lines = n;
        self
    }

    #[cfg(test)]
    pub(crate) fn to_string(&self) -> String {
        let mut svg_string = String::new();

        let horizontal = Horizontal::new()
            .x_range(self.x_range[0], self.x_range[1])
            .y_range(self.y_range[0], self.y_range[1])
            .lines(self.h_lines)
            .tags()
            .to_string();

        let vertical = Vertical::new()
            .x_range(self.x_range[0], self.x_range[1])
            .y_range(self.y_range[0], self.y_range[1])
            .lines(self.h_lines)
            .tags()
            .to_string();

        svg_string.push_str(&horizontal);
        svg_string.push_str("\n");
        svg_string.push_str(&vertical);

        svg_string
    }

    pub(crate) fn write(&self, out: Option<&Path>) -> Result<()> {
        Horizontal::new()
            .x_range(self.x_range[0], self.x_range[1])
            .y_range(self.y_range[0], self.y_range[1])
            .lines(self.h_lines)
            .tags()
            .write(out)
            .unwrap();

        Vertical::new()
            .x_range(self.x_range[0], self.x_range[1])
            .y_range(self.y_range[0], self.y_range[1])
            .lines(self.h_lines)
            .tags()
            .write(out)
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_chart() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Chart::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .h_lines(5)
            .v_lines(5)
            .to_string();

        let horizontal_tags = format!("<svg xmlns=\"http://www.w3.org/2000/svg\">\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            </svg>",
            120, 10, 1500, 10,
            120, 97.5, 1500, 97.5,
            120, 185, 1500, 185,
            120, 272.5, 1500, 272.5,
            120, 360, 1500, 360);

        let vertical_tags = format!("<svg xmlns=\"http://www.w3.org/2000/svg\">\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            <path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>\n\
            </svg>",
            120,  10, 120,  360,
            465,  10, 465,  360,
            810,  10, 810,  360,
            1155, 10, 1155, 360,
            1500, 10, 1500, 360);

        let mut want = String::new();
        want.push_str(&horizontal_tags);
        want.push_str("\n");
        want.push_str(&vertical_tags);

        assert_eq!(want, have);
    }
}
