mod common;
mod error;
mod horizontal;
mod point;

pub(crate) use crate::common::*;

fn main() {
    // Generates 5 horizontal lines
    let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
    let mut horiz = Horizontal::new(points, 5);
    horiz.get_tags().unwrap();
    write(&horiz.tags, None).unwrap();
}

fn write(document: &SvgDocument, out: Option<&Path>) -> Result<()> {
    if let Some(path) = out {
        let buffer = File::create(path).map_err(|io_error| Error::IoError {
            io_error,
            path: path.into(),
        })?;
        svg::write(buffer, document).map_err(|io_error| Error::IoError {
            io_error,
            path: path.into(),
        })?;
    } else {
        writeln!(io::stdout(), "{:?}", &document.to_string()).unwrap();
    }

    Ok(())
}
