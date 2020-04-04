pub(crate) use crate::common::*;

pub(crate) struct Horizontal {
    range: (Point, Point),
    num_lines: u8,
    pub(crate) tags: SvgDocument,
}

impl Horizontal {
    pub(crate) fn new(range: (Point, Point), num_lines: u8) -> Self {
        let tags = SvgDocument::new();
        Horizontal {
            range,
            num_lines,
            tags,
        }
    }

    pub fn get_tags(&mut self) -> Result<()> {
        let y_coordinates = self.coordinates().unwrap();
        let mut tmp_document = SvgDocument::new();
        for y in y_coordinates {
            let path = self.get_path(y).unwrap();
            tmp_document = tmp_document.add(path);
        }
        self.tags = tmp_document;
        Ok(())
    }

    fn coordinates(&self) -> Result<Vec<f32>, ()> {
        let segment = self.delta()?;

        let mut lines = Vec::new();
        for i in 0..self.num_lines - 1 {
            let tmp = self.range.0.y + (segment * i as f32);
            lines.push(tmp);
        }
        lines.push(self.range.1.y);
        Ok(lines)
    }

    fn delta(&self) -> Result<f32, ()> {
        Ok((self.range.1.y - self.range.0.y).abs() / (self.num_lines - 1) as f32)
    }

    fn get_path(&self, y: f32) -> Result<SvgPath> {
        let data = SvgData::new()
            .move_to((self.range.0.x, y))
            .line_to((self.range.1.x, y));
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
        let horiz = Horizontal::new(points, 3);
        let have = horiz.delta().unwrap();

        let want = 175.0;
        assert_eq!(want, have);
    }

    #[test]
    fn segment_width_btwn_5_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let horiz = Horizontal::new(points, 5);
        let have = horiz.delta().unwrap();

        let want = 87.5;
        assert_eq!(want, have);
    }

    #[test]
    fn y_coords_of_3_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let horiz = Horizontal::new(points, 3);
        let have = horiz.coordinates().unwrap();

        let want = vec![10.0, 185.0, 360.0];
        assert_eq!(want, have);
    }

    #[test]
    fn y_coords_of_5_lines() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let horiz = Horizontal::new(points, 5);
        let have = horiz.coordinates().unwrap();

        let want = vec![10.0, 97.5, 185.0, 272.5, 360.0];
        assert_eq!(want, have);
    }
    #[test]
    fn generates_svg_tags() {
        let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
        let mut horiz = Horizontal::new(points, 5);

        let tmp_document = SvgDocument::new();
        let path0 = horiz.get_path(10.0).unwrap();
        let tmp_document = tmp_document.add(path0);

        let path1 = horiz.get_path(97.5).unwrap();
        let tmp_document = tmp_document.add(path1);

        let path2 = horiz.get_path(185.0).unwrap();
        let _tmp_document = tmp_document.add(path2);
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
        horiz.get_tags().unwrap();
        let have = horiz.tags.to_string();
        assert_eq!(want, have);
    }
}
