use more_path_types::{AbsolutePathError, RelativePathError};
use std::io::ErrorKind;

pub type IoErr = std::io::Error;

pub trait IntoIoError {
    fn into_ioerr(self) -> IoErr;
}

impl IntoIoError for RelativePathError {
    fn into_ioerr(self) -> IoErr {
        match self {
            RelativePathError::NoWorkingDirectory { io_error } => io_error,
            RelativePathError::PathDiff => IoErr::new(ErrorKind::Other, "failed path-diff"),
            RelativePathError::Absolutize { io_error } => io_error,
        }
    }
}

impl IntoIoError for AbsolutePathError {
    fn into_ioerr(self) -> IoErr {
        match self {
            AbsolutePathError::Absolutize { io_error } => io_error,
        }
    }
}
