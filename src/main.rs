mod common;
mod error;
mod horizontal;
mod vertical;

pub(crate) use crate::common::*;

fn main() {
    let x = (120.0, 1500.0);
    let y = (10.0, 360.0);

    // Generate 5 horizontal lines
    Horizontal::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .lines(5)
        .tags()
        .write(None)
        .unwrap();

    // Generate 5 vertical lines
    Vertical::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .lines(5)
        .tags()
        .write(None)
        .unwrap();
}
