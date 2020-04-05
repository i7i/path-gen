pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}
