use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
    install_dir(repository, install_base, repository, &info.dirs)
}

#[derive(Debug, PartialEq)]
enum InstallStatus {
    Installed,
    Skipped,
}

fn install_dir(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    repository: &AbsPath,
    dirs: &Vec<PathBuf>,
) -> Result<(), IoErr> {
    for entry in from.as_ref().read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let status = install_file(&from, &to, &entry.file_name())?;
            if status == InstallStatus::Skipped {
                eprintln!("Skipped");
            }
        } else if file_type.is_dir() {
            let from = from.as_ref().join(entry.file_name());
            let to = to.as_ref().join(entry.file_name());
            let rel_src = RelPath::with_virtual_working_dir(&from, repository)
                .map_err(IntoIoError::into_ioerr)?;
            if dirs.contains(&rel_src.as_ref().to_path_buf()) {
                let status = install_dir_itself(&from, &to)?;
                if status == InstallStatus::Skipped {
                    eprintln!("Skipped");
                }
            } else {
                install_dir(&from, &to, repository, dirs)?;
            }
        } else {
            continue;
        }
    }
    Ok(())
}

fn install_file(
    src_dir: impl AsRef<Path>,
    dst_dir: impl AsRef<Path>,
    file_name: &OsStr,
) -> Result<InstallStatus, IoErr> {
    let from = AbsPath::new(src_dir.as_ref().join(file_name)).map_err(IntoIoError::into_ioerr)?;
    let to = dst_dir.as_ref().join(file_name);

    if to.exists() {
        return Ok(InstallStatus::Skipped);
    }

    eprintln!(
        "creating symlink {} -> {}",
        to.to_string_lossy(),
        from.as_ref().to_string_lossy(),
    );
    fs::create_dir_all(&dst_dir)?;
    crate::make_symlink(&to, &from)?;

    Ok(InstallStatus::Installed)
}

fn install_dir_itself(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<InstallStatus, IoErr> {
    let from = AbsPath::new(&from).map_err(IntoIoError::into_ioerr)?;

    if to.as_ref().exists() {
        return Ok(InstallStatus::Skipped);
    }

    eprintln!(
        "creating dir symlink {} -> {}",
        to.as_ref().to_string_lossy(),
        from.as_ref().to_string_lossy(),
    );
    if let Some(dir) = to.as_ref().parent() {
        fs::create_dir_all(dir)?;
    }
    crate::make_symlink(&to, &from)?;

    Ok(InstallStatus::Installed)
}
