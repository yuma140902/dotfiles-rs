use std::fmt::Display;
use std::fs;

use crate::error::IntoIoError;
use crate::error::IoErr;
use crate::repo::RepoInfo;
use crate::AbsPath;
use crate::RelPath;

pub struct InstallResult {
    files_installed: u32,
    files_skipped: u32,
    files_errored: u32,
    dirs_installed: u32,
    dirs_skipped: u32,
    dirs_errored: u32,
}

impl Display for InstallResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Installed {} files, {} skipped, {} errored\n",
            self.files_installed, self.files_skipped, self.files_errored
        ))?;
        f.write_fmt(format_args!(
            "Installed {} directories, {} skipped, {} errored\n",
            self.dirs_installed, self.dirs_skipped, self.dirs_errored
        ))?;
        Ok(())
    }
}

pub fn try_install(
    repository: &AbsPath,
    install_base: &AbsPath,
    info: &RepoInfo,
) -> Result<InstallResult, IoErr> {
    eprintln!("Installing files");
    let mut files_installed = 0;
    let mut files_skipped = 0;
    let mut files_errored = 0;
    for file in &info.files {
        let file = RelPath::new(file).map_err(IntoIoError::into_ioerr)?;
        eprintln!("Installing file {}", file.as_ref().to_string_lossy());
        let status = install_file(repository, install_base, &file)?;
        eprintln!("{}", status);
        match status {
            InstallStatus::Installed => files_installed += 1,
            InstallStatus::Skipped(_) => files_skipped += 1,
            InstallStatus::Error(_) => files_errored += 1,
        }
    }

    eprintln!("Installing directories");
    let mut dirs_installed = 0;
    let mut dirs_skipped = 0;
    let mut dirs_errored = 0;
    for dir in &info.dirs {
        let dir = RelPath::new(dir).map_err(IntoIoError::into_ioerr)?;
        eprintln!("Installing directory {}", dir.as_ref().to_string_lossy());
        let status = install_dir(repository, install_base, &dir)?;
        eprintln!("{}", status);
        match status {
            InstallStatus::Installed => dirs_installed += 1,
            InstallStatus::Skipped(_) => dirs_skipped += 1,
            InstallStatus::Error(_) => dirs_errored += 1,
        }
    }

    Ok(InstallResult {
        files_installed,
        files_skipped,
        files_errored,
        dirs_installed,
        dirs_skipped,
        dirs_errored,
    })
}

#[derive(Debug, PartialEq)]
enum InstallStatus {
    Installed,
    Skipped(SkipReason),
    Error(ErrorKind),
}

#[derive(Debug, PartialEq)]
enum SkipReason {
    AlreadyInstalled,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {}

impl Display for InstallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallStatus::Installed => f.write_str("Done")?,
            InstallStatus::Skipped(reason) => {
                f.write_fmt(format_args!("Skipped. reason: {}", reason))?
            }
            InstallStatus::Error(err) => f.write_fmt(format_args!("Error: {}", err))?,
        }
        Ok(())
    }
}

impl Display for SkipReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self))?;
        Ok(())
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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
        return Ok(InstallStatus::Skipped(SkipReason::AlreadyInstalled));
    }

    eprintln!(
        "creating symlink {} -> {}",
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
