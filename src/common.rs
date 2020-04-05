pub(crate) use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, prelude::*},
    path::{Path, PathBuf},
};

pub(crate) use svg::{
    node::element::{path::Data as SvgData, Path as SvgPath},
    Document as SvgDocument,
};

pub(crate) use crate::{chart::Chart, error::Error, horizontal::Horizontal, vertical::Vertical};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
