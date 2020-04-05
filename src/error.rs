pub(crate) use crate::common::*;

#[derive(Debug)]
pub enum Error {
    IoError {
        io_error: std::io::Error,
        path: PathBuf,
    },
    LineNumber,
    StdIoError {
        io_error: std::io::Error,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;

        match self {
            IoError { io_error, path } => {
                write!(f, "I/O error at `{}`: {}", path.display(), io_error)
            }
            LineNumber => write!(f, "Number of chart lines must be greater than 2.",),
            StdIoError { io_error } => write!(f, "Error when writing to `std::out`: {}", io_error),
        }
    }
}
