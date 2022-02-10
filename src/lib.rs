use std::path::Path;
use std::path::PathBuf;

use crate::error::IoErr;

use more_path_types::Absolute;
use more_path_types::Any;
use more_path_types::Relative;
use once_cell::sync::Lazy;

pub type AbsPath = more_path_types::Path<Absolute, Any>;
pub type RelPath = more_path_types::Path<Relative, Any>;

mod error;
mod install;
mod pick;
mod repo;

pub static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| dirs::home_dir().expect("no home dir"));

pub fn pick(repository: &PathBuf, install_base: &Option<PathBuf>, files_and_dirs: &Vec<PathBuf>) {
    let repository = AbsPath::with_virtual_working_dir(repository, &*HOME_DIR)
        .expect("could not absolutize repository path");
    let install_base = install_base.as_ref().unwrap_or(&*HOME_DIR);
    let install_base = AbsPath::new(install_base).expect("could not absolutize install_base");

    let result = pick::try_pick_files_and_dirs(&repository, &install_base, files_and_dirs);
    if let Err(err) = result {
        eprintln!("Error in pick subcommand");
        eprintln!("{:#?}", err);
    }
}

pub fn install(repository: &PathBuf, install_base: &Option<PathBuf>) {
    let repository = AbsPath::with_virtual_working_dir(repository, &*HOME_DIR)
        .expect("could not absolutize repository path");
    let install_base = install_base.as_ref().unwrap_or(&*HOME_DIR);
    let install_base = AbsPath::new(install_base).expect("could not absolutize install_base");

    let result = install::try_install(&repository, &install_base);
    if let Err(err) = result {
        eprintln!("Error in install subcommand");
        eprintln!("{:#?}", err);
    }
}

fn make_symlink(symlink_path: impl AsRef<Path>, target: &AbsPath) -> Result<(), IoErr> {
    let symlink_path = symlink_path.as_ref();
    let target = target.as_ref();
    if target.is_file() {
        symlink::symlink_file(target, symlink_path)?;
    } else if target.is_dir() {
        symlink::symlink_dir(target, symlink_path)?;
    } else {
        return Err(IoErr::new(
            std::io::ErrorKind::Other,
            "could not make symlink",
        ));
    }

    Ok(())
}
