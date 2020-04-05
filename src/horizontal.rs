pub(crate) use crate::common::*;

pub(crate) struct Horizontal {
    x_range: [f32; 2],
    y_range: [f32; 2],
    lines: u8,
    pub(crate) tags: SvgDocument,
}

impl Horizontal {
    pub(crate) fn new() -> Self {
        let tags = SvgDocument::new();
        let lines = 0;
        let x_range = [0.0; 2];
        let y_range = [0.0; 2];
        Horizontal {
            x_range,
            y_range,
            lines,
            tags,
        }
    }

    pub(crate) fn x_range<'a>(&'a mut self, x0: f32, x1: f32) -> &'a mut Horizontal {
        self.x_range[0] = x0;
        self.x_range[1] = x1;
        self
    }

    pub(crate) fn y_range<'a>(&'a mut self, y0: f32, y1: f32) -> &'a mut Horizontal {
        self.y_range[0] = y0;
        self.y_range[1] = y1;
        self
    }

    pub(crate) fn lines<'a>(&'a mut self, n: u8) -> &'a mut Horizontal {
        self.lines = n;
        self
    }

    pub(crate) fn tags<'a>(&'a mut self) -> &'a mut Horizontal {
        let y_coordinates = self.coordinates().unwrap();
        let mut tmp_document = SvgDocument::new();
        for y in y_coordinates {
            let path = self.get_path(y).unwrap();
            tmp_document = tmp_document.add(path);
        }
        self.tags = tmp_document;
        self
    }

    pub(crate) fn to_string(&self) -> String {
        self.tags.to_string()
    }

    pub(crate) fn write(&self, out: Option<&Path>) -> Result<()> {
        if let Some(path) = out {
            let buffer = File::create(path).map_err(|io_error| Error::IoError {
                io_error,
                path: path.into(),
            })?;
            svg::write(buffer, &self.tags).map_err(|io_error| Error::IoError {
                io_error,
                path: path.into(),
            })?;
        } else {
            writeln!(io::stdout(), "{}", self.to_string()).unwrap();
        }

        Ok(())
    }

    fn coordinates(&self) -> Result<Vec<f32>, ()> {
        let segment = self.delta()?;

        let mut lines = Vec::new();
        for i in 0..self.lines - 1 {
            let tmp = self.y_range[0] + (segment * i as f32);
            lines.push(tmp);
        }
        lines.push(self.y_range[1]);
        Ok(lines)
    }

    fn delta(&self) -> Result<f32, ()> {
        Ok((self.y_range[1] - self.y_range[0]).abs() / (self.lines - 1) as f32)
    }

    fn get_path(&self, y: f32) -> Result<SvgPath> {
        let data = SvgData::new()
            .move_to((self.x_range[0], y))
            .line_to((self.x_range[1], y));
        Ok(SvgPath::new()
            .set("stroke", "#BFEFF2")
            .set("stroke-width", 1)
            .set("opacity", 1)
            .set("zIndex", 1)
            .set("d", data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_width_btwn_3_lines() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(3)
            .delta()
            .unwrap();

        let want = 175.0;

        assert_eq!(want, have);
    }

    #[test]
    fn segment_width_btwn_5_lines() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(5)
            .delta()
            .unwrap();

        let want = 87.5;

        assert_eq!(want, have);
    }

    #[test]
    fn y_coords_of_3_lines() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(3)
            .coordinates()
            .unwrap();

        let want = vec![10.0, 185.0, 360.0];

        assert_eq!(want, have);
    }

    #[test]
    fn y_coords_of_5_lines() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(5)
            .coordinates()
            .unwrap();

        let want = vec![10.0, 97.5, 185.0, 272.5, 360.0];
        assert_eq!(want, have);
    }

    #[test]
    fn generates_svg_path() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);

        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(5)
            .get_path(10.0)
            .unwrap()
            .to_string();

        let want = format!("<path d=\"M{},{} L{},{}\" opacity=\"1\" stroke=\"#BFEFF2\" stroke-width=\"1\" zIndex=\"1\"/>",
            120, 10, 1500, 10);

        assert_eq!(want, have);
    }

    #[test]
    fn generates_svg_tags() {
        let x = (120.0, 1500.0);
        let y = (10.0, 360.0);
        let have = Horizontal::new()
            .x_range(x.0, x.1)
            .y_range(y.0, y.1)
            .lines(5)
            .tags()
            .to_string();

        let want = format!("<svg xmlns=\"http://www.w3.org/2000/svg\">\n\
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

        assert_eq!(want, have);
    }
}
