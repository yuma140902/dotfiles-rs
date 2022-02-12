use std::fs;

use crate::error::IntoIoError;
use crate::error::IoErr;
use crate::repo::RepoInfo;
use crate::AbsPath;
use crate::RelPath;

pub fn try_install(
    repository: &AbsPath,
    install_base: &AbsPath,
    info: &RepoInfo,
) -> Result<(), IoErr> {
    eprintln!("Installing files");
    let mut count = 0;
    for file in &info.files {
        let file = RelPath::new(file).map_err(IntoIoError::into_ioerr)?;
        eprintln!("Installing {}", file.as_ref().to_string_lossy());
        let status = install_file(repository, install_base, &file)?;
        if status == InstallStatus::Skipped {
            eprintln!("Skipped");
        } else {
            count += 1;
        }
    }
    eprintln!("Installed {} files", count);

    eprintln!("Installing directories");
    let mut count = 0;
    for dir in &info.dirs {
        let dir = RelPath::new(dir).map_err(IntoIoError::into_ioerr)?;
        eprintln!("Installing {}", dir.as_ref().to_string_lossy());
        let status = install_dir(repository, install_base, &dir)?;
        if status == InstallStatus::Skipped {
            eprintln!("Skipped");
        } else {
            count += 1;
        }
    }
    eprintln!("Installed {} directories", count);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum InstallStatus {
    Installed,
    Skipped,
}

fn install_file(
    repository: &AbsPath,
    install_base: &AbsPath,
    file: &RelPath,
) -> Result<InstallStatus, IoErr> {
    let in_repo =
        AbsPath::with_virtual_working_dir(file, repository).map_err(IntoIoError::into_ioerr)?;
    let in_home =
        AbsPath::with_virtual_working_dir(file, install_base).map_err(IntoIoError::into_ioerr)?;

    if in_home.as_ref().exists() {
        return Ok(InstallStatus::Skipped);
    }

    eprintln!(
        "Creating symlink {} -> {}",
        in_home.as_ref().to_string_lossy(),
        in_repo.as_ref().to_string_lossy()
    );
    if let Some(dir) = in_home.as_ref().parent() {
        fs::create_dir_all(dir)?;
    }
    crate::make_symlink(&in_home, &in_repo)?;

    Ok(InstallStatus::Installed)
}

fn install_dir(
    repository: &AbsPath,
    install_base: &AbsPath,
    dir: &RelPath,
) -> Result<InstallStatus, IoErr> {
    install_file(repository, install_base, dir)
}
