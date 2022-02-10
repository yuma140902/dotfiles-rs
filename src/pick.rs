use std::fs;
use std::path::PathBuf;

use crate::error::IntoIoError;
use crate::AbsPath;
use crate::RelPath;

type IoErr = std::io::Error;

pub fn try_pick_files_and_dirs<'a>(
    repository: &AbsPath,
    install_base: &AbsPath,
    files_and_dirs: &'a Vec<PathBuf>,
) -> Result<(), IoErr> {
    for path_in_home in files_and_dirs {
        let path_in_home = AbsPath::new(path_in_home).map_err(IntoIoError::into_ioerr)?;
        if path_in_home.as_ref().is_file() {
            eprintln!(
                "Picking up file {}",
                path_in_home.as_ref().to_string_lossy()
            );
            let result = pick_file(repository, install_base, &path_in_home)?;
            if result == PickStatus::Skipped {
                eprintln!("Skipped");
            }
        } else if path_in_home.as_ref().is_file() {
            eprintln!(
                "Picking up directory {}",
                path_in_home.as_ref().to_string_lossy()
            );
            let result = pick_dir(repository, install_base, &path_in_home)?;
            if result == PickStatus::Skipped {
                eprintln!("Skipped");
            }
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum PickStatus {
    Picked,
    Skipped,
}

fn pick_file(
    repository: &AbsPath,
    install_base: &AbsPath,
    path_in_home: &AbsPath,
) -> Result<PickStatus, std::io::Error> {
    let path_rel = RelPath::with_virtual_working_dir(path_in_home, &install_base)
        .map_err(IntoIoError::into_ioerr)?;

    let path_in_repo = AbsPath::with_virtual_working_dir(path_rel, &repository)
        .map_err(IntoIoError::into_ioerr)?;

    eprintln!(
        "copying {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy()
    );
    fs::copy(path_in_home, &path_in_repo)?;

    eprintln!("removing {}", path_in_home.as_ref().to_string_lossy());
    fs::remove_file(path_in_home)?;

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
    path_in_home: &AbsPath,
) -> Result<PickStatus, std::io::Error> {
    todo!()
}
