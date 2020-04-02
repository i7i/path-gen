pub(crate) use crate::common::*;

#[derive(Debug)]
pub enum Error {
    IoError {
        io_error: std::io::Error,
        path: PathBuf,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;

        match self {
            IoError { io_error, path } => {
                write!(f, "I/O error at `{}`: {}", path.display(), io_error)
            }
        }
    }
}
