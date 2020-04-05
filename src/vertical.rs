pub(crate) use crate::common::*;

pub(crate) struct Vertical {
    range: (Point, Point),
    num_lines: u8,
    pub(crate) tags: SvgDocument,
}

impl Vertical {
    pub(crate) fn new(range: (Point, Point), num_lines: u8) -> Self {
        let tags = SvgDocument::new();
        Vertical {
            range,
            num_lines,
            tags,
        }
    }

    pub fn get_tags(&mut self) -> Result<()> {
        let x_coordinates = self.coordinates().unwrap();
        let mut tmp_document = SvgDocument::new();
        for x in x_coordinates {
            let path = self.get_path(x).unwrap();
            tmp_document = tmp_document.add(path);
        }
        self.tags = tmp_document;
        Ok(())
    }

    fn coordinates(&self) -> Result<Vec<f32>, ()> {
        let segment = self.delta()?;

        let mut lines = Vec::new();
        for i in 0..self.num_lines - 1 {
            let tmp = self.range.0.x + (segment * i as f32);
            lines.push(tmp);
        }
        lines.push(self.range.1.x);
        Ok(lines)
    }

    fn delta(&self) -> Result<f32, ()> {
        Ok((self.range.1.x - self.range.0.x).abs() / (self.num_lines - 1) as f32)
    }

    fn get_path(&self, x: f32) -> Result<SvgPath> {
        let data = SvgData::new()
            .move_to((x, self.range.0.y))
            .line_to((x, self.range.1.y));
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
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let vertical = Vertical::new(points, 3);
        let have = vertical.delta().unwrap();

        let want = 690.0;
        assert_eq!(want, have);
    }

    #[test]
    fn segment_width_btwn_5_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let vertical = Vertical::new(points, 5);
        let have = vertical.delta().unwrap();

        let want = 345.0;
        assert_eq!(want, have);
    }

    #[test]
    fn x_coords_of_3_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let vertical = Vertical::new(points, 3);
        let have = vertical.coordinates().unwrap();

        let want = vec![120.0, 810.0, 1500.0];
        assert_eq!(want, have);
    }

    #[test]
    fn x_coords_of_5_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let vertical = Vertical::new(points, 5);
        let have = vertical.coordinates().unwrap();

        let want = vec![120.0, 465.0, 810.0, 1155.0, 1500.0];
        assert_eq!(want, have);
    }

    #[test]
    fn generates_svg_paths() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let mut vertical = Vertical::new(points, 5);

        let tmp_document = SvgDocument::new();
        let path0 = vertical.get_path(120.0).unwrap();
        let tmp_document = tmp_document.add(path0);

        let path1 = vertical.get_path(465.0).unwrap();
        let tmp_document = tmp_document.add(path1);

        let path2 = vertical.get_path(810.0).unwrap();
        let tmp_document = tmp_document.add(path2);

        let path3 = vertical.get_path(1155.0).unwrap();
        let tmp_document = tmp_document.add(path3);

        let path4 = vertical.get_path(1500.0).unwrap();
        let tmp_document = tmp_document.add(path4);

        vertical.tags = tmp_document;
        let have = vertical.tags.to_string();

        let want = format!("<svg xmlns=\"http://www.w3.org/2000/svg\">\n\
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

        assert_eq!(want, have);
    }

    #[test]
    fn generates_svg_tags() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let mut vertical = Vertical::new(points, 5);
        vertical.get_tags().unwrap();

        let want = format!("<svg xmlns=\"http://www.w3.org/2000/svg\">\n\
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

        let have = vertical.tags.to_string();
        assert_eq!(want, have);
    }
}
