use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::error::IoErr;

use more_path_types::Absolute;
use more_path_types::Any;
use more_path_types::Relative;
use once_cell::sync::Lazy;
use repo::RepoInfo;

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

    let mut info = load_repo_info(&repository).expect("could not load repo info");

    let result =
        pick::try_pick_files_and_dirs(&repository, &install_base, files_and_dirs, &mut info);
    if let Err(err) = result {
        eprintln!("Error in pick subcommand");
        eprintln!("{:#?}", err);
    }
    save_repo_info(&info, &repository).expect("could not save repo info");
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

const REPO_INFO_FILE_NAME: &str = "dotfiles-rs.yml";

fn load_repo_info(repository: &AbsPath) -> Result<RepoInfo, Box<dyn Error>> {
    let path = repository.as_ref().join(REPO_INFO_FILE_NAME);
    if !path.exists() {
        return Ok(RepoInfo::default());
    }
    let yaml = fs::read_to_string(path)?;
    let info: RepoInfo = serde_yaml::from_str(&yaml)?;
    Ok(info)
}

fn save_repo_info(info: &RepoInfo, repository: &AbsPath) -> Result<(), Box<dyn Error>> {
    let path = repository.as_ref().join(REPO_INFO_FILE_NAME);
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)?;
    let yaml = serde_yaml::to_string(info)?;
    file.write_all(yaml.as_bytes())?;
    Ok(())
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
