mod chart;
mod common;
mod error;
mod horizontal;
mod vertical;

pub(crate) use crate::common::*;

fn main() -> Result<()> {
    let x = (120.0, 1500.0);
    let y = (10.0, 360.0);

    // Generate 5 horizontal lines
    Horizontal::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .lines(5)
        .tags()?
        .write(None)?;

    // Generate 5 vertical lines
    Vertical::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .lines(5)
        .tags()?
        .write(None)?;

    Chart::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .h_lines(5)
        .v_lines(5)
        .write(None)?;

    Ok(())
}
