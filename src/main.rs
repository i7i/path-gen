mod common;
mod error;

pub(crate) use crate::common::*;

fn main() {
    make_data();
}

fn make_data() {
    let data = SvgData::new()
        .move_to((120.0, 10.0))
        .line_to((120.0, 360.0));

    let path = SvgPath::new()
        .set("fill", "none")
        .set("stroke", "#BFEFF2")
        .set("stroke-width", 1)
        .set("opacity", 1)
        .set("d", data);

    let document = SvgDocument::new().add(path);

    write(&document, None).unwrap();
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

#[cfg(test)]
fn path_element(point0: (f32, f32), point1: (f32, f32)) -> String {
    format!(
        "<path fill=\"none\" d=\"M {:.2} {:.2} L {:.2} {:.2}\" stroke=\"#BFEFF2\" \
        stroke-width=\"1\" zIndex=\"1\" opacity=\"1\"></path>",
        point0.0, point0.1, point1.0, point1.1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_element_from_point() {
        let point0 = (120.0, 10.0);
        let point1 = (120.0, 360.0);
        let have = path_element(point0, point1);
        let want = "<path fill=\"none\" d=\"M 120.00 10.00 L 120.00 360.00\" stroke=\"#BFEFF2\" \
                    stroke-width=\"1\" zIndex=\"1\" opacity=\"1\"></path>";

        assert_eq!(want, have);
    }
}
