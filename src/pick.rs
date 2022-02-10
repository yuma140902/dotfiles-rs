use std::fs;
use std::path::Path;
use std::path::PathBuf;

use more_path_types::AbsolutePathError;
use more_path_types::RelativePathError;

use crate::AbsPath;
use crate::RelPath;

#[derive(Debug)]
pub enum PickErrorKind {
    IO(std::io::Error),
    AbsPath(AbsolutePathError),
    RelPath(RelativePathError),
}

#[derive(Debug)]
pub struct SinglePickError<'a> {
    path: &'a Path,
    kind: PickErrorKind,
}

#[derive(Debug)]
pub struct MultiplePickError<'a>(pub Vec<SinglePickError<'a>>);

impl<'a> MultiplePickError<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, error: SinglePickError<'a>) {
        self.0.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

macro_rules! something_if_fail {
    ($e:expr => $something:stmt) => {
        something_if_fail!($e => |_err| {})
    };
    ($e:expr => $err_handler:expr => $something:stmt) => {
        something_if_fail!($e => |err| { err } => $err_handler => $something)
    };
    ($e:expr => $err_converter:expr => $err_handler:expr => $something:stmt) => {
        match $e {
            Ok(v) => v,
            Err(err) => {
                $err_handler($err_converter(err));
                $something
            }
        }
    };
}

// XXX: マクロの乱用
macro_rules! next_if_fail {
    ($e:expr) => {
        something_if_fail!($e => continue)
    };
    ($e:expr => $err_handler:expr) => {
        something_if_fail!($e => $err_handler => continue)
    };
    ($e:expr => $err_converter:expr => $err_handler:expr) => {
        something_if_fail!($e => $err_converter => $err_handler => continue)
    };
}

pub fn try_pick_files_and_dirs<'a>(
    repository: &AbsPath,
    install_base: &AbsPath,
    files_and_dirs: &'a Vec<PathBuf>,
) -> Result<(), MultiplePickError<'a>> {
    let mut errors = MultiplePickError::new();
    let mut err_handler = |err| {
        eprintln!("ERROR!");
        errors.push(err);
    };
    fn io_err_cnv<'a>(path: &'a PathBuf) -> impl Fn(std::io::Error) -> SinglePickError<'a> {
        move |err: std::io::Error| SinglePickError {
            path,
            kind: PickErrorKind::IO(err),
        }
    }
    for path_in_home in files_and_dirs {
        let path_rel = next_if_fail!(RelPath::with_virtual_working_dir(path_in_home, &install_base)
            => |err| SinglePickError { path: &path_in_home, kind: PickErrorKind::RelPath(err) } => err_handler);

        let path_in_repo = next_if_fail!(AbsPath::with_virtual_working_dir(path_rel, &repository)
            => |err| SinglePickError { path: &path_in_home, kind: PickErrorKind::AbsPath(err) } => err_handler);

        eprintln!(
            "copying {} -> {}",
            path_in_home.to_string_lossy(),
            path_in_repo.as_ref().to_string_lossy()
        );
        next_if_fail!(fs::copy(path_in_home, &path_in_repo) => io_err_cnv(path_in_home) => err_handler);

        eprintln!("removing {}", path_in_home.to_string_lossy());
        next_if_fail!(remove_file_or_directory(path_in_home) => io_err_cnv(path_in_home) => err_handler);

        eprintln!(
            "creating symlink {} -> {}",
            path_in_home.to_string_lossy(),
            path_in_repo.as_ref().to_string_lossy()
        );
        next_if_fail!(crate::make_symlink(path_in_home, &path_in_repo) => io_err_cnv(path_in_home) => err_handler);
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}

fn remove_file_or_directory(path: impl AsRef<Path>) -> Result<(), std::io::Error> {
    if path.as_ref().is_file() {
        fs::remove_file(path)?;
    } else if path.as_ref().is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "could not delete",
        ));
    }

    Ok(())
}
