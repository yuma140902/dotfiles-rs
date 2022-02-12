use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::error::IntoIoError;
use crate::error::IoErr;
use crate::repo::RepoInfo;
use crate::AbsPath;
use crate::RelPath;

pub fn try_pick_files_and_dirs<'a>(
    repository: &AbsPath,
    install_base: &AbsPath,
    files_and_dirs: &'a Vec<PathBuf>,
    info: &mut RepoInfo,
) -> Result<(), IoErr> {
    let mut files: Vec<RelPath> = Vec::new();
    let mut dirs: Vec<RelPath> = Vec::new();
    for path_in_home in files_and_dirs {
        let path = RelPath::with_virtual_working_dir(path_in_home, install_base)
            .map_err(IntoIoError::into_ioerr)?;

        if path_in_home.is_file() {
            eprintln!("Picking up file {}", path_in_home.to_string_lossy());
            let result = pick_file(repository, install_base, &path)?;
            if result == PickStatus::Skipped {
                eprintln!("Skipped");
            }
            files.push(path);
        } else if path_in_home.is_dir() {
            eprintln!("Picking up directory {}", path_in_home.to_string_lossy());
            let result = pick_dir(repository, install_base, &path)?;
            if result == PickStatus::Skipped {
                eprintln!("Skipped");
            }
            dirs.push(path);
        }
    }

    let mut files: Vec<PathBuf> = files
        .iter()
        .map(|rel_path| rel_path.as_ref().to_path_buf())
        .collect();
    info.files.append(&mut files);
    info.files.sort();
    info.files.dedup();

    let mut dirs: Vec<PathBuf> = dirs
        .iter()
        .map(|rel_path| rel_path.as_ref().to_path_buf())
        .collect();
    info.dirs.append(&mut dirs);
    info.dirs.sort();
    info.dirs.dedup();

    Ok(())
}

#[derive(Debug, PartialEq)]
enum PickStatus {
    Picked,
    Skipped, // TODO
}

fn pick_file(
    repository: &AbsPath,
    install_base: &AbsPath,
    path: &RelPath,
) -> Result<PickStatus, IoErr> {
    let path_in_home =
        AbsPath::with_virtual_working_dir(&path, &install_base).map_err(IntoIoError::into_ioerr)?;
    let path_in_repo =
        AbsPath::with_virtual_working_dir(&path, &repository).map_err(IntoIoError::into_ioerr)?;

    eprintln!(
        "copying {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy()
    );
    if let Some(dir) = path_in_repo.as_ref().parent() {
        fs::create_dir_all(dir)?;
    }
    fs::copy(&path_in_home, &path_in_repo)?;

    eprintln!("removing {}", path_in_home.as_ref().to_string_lossy());
    fs::remove_file(&path_in_home)?;

    eprintln!(
        "creating symlink {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy()
    );
    crate::make_symlink(path_in_home, &path_in_repo)?;

    Ok(PickStatus::Picked)
}

fn pick_dir(
    repository: &AbsPath,
    install_base: &AbsPath,
    path: &RelPath,
) -> Result<PickStatus, IoErr> {
    let path_in_home =
        AbsPath::with_virtual_working_dir(&path, &install_base).map_err(IntoIoError::into_ioerr)?;
    let path_in_repo =
        AbsPath::with_virtual_working_dir(&path, &repository).map_err(IntoIoError::into_ioerr)?;

    eprintln!(
        "copying {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy()
    );
    if let Some(dir) = path_in_repo.as_ref().parent() {
        fs::create_dir_all(dir)?;
    }
    copy_dir_recursive(&path_in_home, &path_in_repo)?;

    eprintln!("removing {}", path_in_home.as_ref().to_string_lossy());
    fs::remove_dir_all(&path_in_home)?;

    eprintln!(
        "creating symlink {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy()
    );
    crate::make_symlink(path_in_home, &path_in_repo)?;

    Ok(PickStatus::Picked)
}

/// ディレクトリを再帰的にコピーする。幅優先である
fn copy_dir_recursive(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), IoErr> {
    fs::create_dir(&to)?;
    for entry in from.as_ref().read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let dest = to.as_ref().join(entry.file_name());
        if entry.file_type()?.is_file() {
            fs::copy(path, dest)?;
        } else if entry.file_type()?.is_dir() {
            copy_dir_recursive(path, dest)?;
        }
    }
    Ok(())
}
