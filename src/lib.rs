use std::fs;
use std::path::Path;
use std::path::PathBuf;

use more_path_types::Absolute;
use more_path_types::AbsolutePathError;
use more_path_types::Any;
use more_path_types::Relative;
use more_path_types::RelativePathError;
use once_cell::sync::Lazy;

type AbsPath = more_path_types::Path<Absolute, Any>;
type RelPath = more_path_types::Path<Relative, Any>;

pub static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| dirs::home_dir().expect("no home dir"));

pub fn pick(repository: &PathBuf, install_base: &Option<PathBuf>, files_and_dirs: &Vec<PathBuf>) {
    let repository = AbsPath::with_virtual_working_dir(repository, &*HOME_DIR)
        .expect("could not absolutize repository path");
    let install_base = install_base.as_ref().unwrap_or(&*HOME_DIR);
    let install_base = AbsPath::new(install_base).expect("could not absolutize install_base");

    let result = try_pick_files_and_dirs(&repository, &install_base, files_and_dirs);
    if let Err(err) = result {
        eprintln!("Error in pick subcommand");
        eprintln!("{:#?}", err);
    }
}

#[derive(Debug)]
enum PickErrorKind {
    IO(std::io::Error),
    AbsPath(AbsolutePathError),
    RelPath(RelativePathError),
}

#[derive(Debug)]
struct SinglePickError<'a> {
    path: &'a Path,
    kind: PickErrorKind,
}

#[derive(Debug)]
struct MultiplePickError<'a>(pub Vec<SinglePickError<'a>>);

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

fn try_pick_files_and_dirs<'a>(
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
        next_if_fail!(make_symlink(path_in_home, &path_in_repo) => io_err_cnv(path_in_home) => err_handler);
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

fn make_symlink(symlink_path: impl AsRef<Path>, target: &AbsPath) -> Result<(), std::io::Error> {
    let symlink_path = symlink_path.as_ref();
    let target = target.as_ref();
    if target.is_file() {
        symlink::symlink_file(target, symlink_path)?;
    } else if target.is_dir() {
        symlink::symlink_dir(target, symlink_path)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "could not make symlink",
        ));
    }

    Ok(())
}
