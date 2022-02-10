use std::fs;

use crate::error::IntoIoError;
use crate::error::IoErr;
use crate::AbsPath;
use crate::RelPath;

pub fn try_install(repository: &AbsPath, install_base: &AbsPath) -> Result<(), IoErr> {
    eprintln!("WARNING: ディレクトリのインストールは未実装");
    for entry in repository.as_ref().read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let file = entry.path();
            eprintln!("Installing {}", file.to_string_lossy());
            let path_in_repo = AbsPath::new(file).map_err(IntoIoError::into_ioerr)?;
            let result = install_file(repository, install_base, &path_in_repo)?;
            if result == InstallStatus::Skipped {
                eprintln!("Skipped");
            }
        }
    }
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
    path_in_repo: &AbsPath,
) -> Result<InstallStatus, IoErr> {
    let path_rel = RelPath::with_virtual_working_dir(path_in_repo, repository)
        .map_err(IntoIoError::into_ioerr)?;

    let path_in_home = AbsPath::with_virtual_working_dir(&path_rel, install_base)
        .map_err(IntoIoError::into_ioerr)?;

    if path_in_home.as_ref().exists() {
        return Ok(InstallStatus::Skipped);
    }

    eprintln!(
        "creating symlink {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy(),
    );
    if let Some(dir) = path_in_home.as_ref().parent() {
        fs::create_dir_all(dir)?;
    }
    crate::make_symlink(&path_in_home, &path_in_repo)?;

    Ok(InstallStatus::Installed)
}
