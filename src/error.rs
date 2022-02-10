use more_path_types::{AbsolutePathError, RelativePathError};

pub trait IntoIoError {
    fn into_ioerr(self) -> std::io::Error;
}

impl IntoIoError for RelativePathError {
    fn into_ioerr(self) -> std::io::Error {
        match self {
            RelativePathError::NoWorkingDirectory { io_error } => io_error,
            RelativePathError::PathDiff => {
                std::io::Error::new(std::io::ErrorKind::Other, "failed path-diff")
            }
            RelativePathError::Absolutize { io_error } => io_error,
        }
    }
}

impl IntoIoError for AbsolutePathError {
    fn into_ioerr(self) -> std::io::Error {
        match self {
            AbsolutePathError::Absolutize { io_error } => io_error,
        }
    }
}
