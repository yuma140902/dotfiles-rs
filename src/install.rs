use more_path_types::AbsolutePathError;
use more_path_types::RelativePathError;

use crate::AbsPath;
use crate::RelPath;

pub fn try_install(repository: &AbsPath, install_base: &AbsPath) -> Result<(), std::io::Error> {
    eprintln!("WARNING: ディレクトリのインストールは未実装");
    for entry in repository.as_ref().read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let file = entry.path();
            eprintln!("Installing {}", file.to_string_lossy());
            let path_in_repo = AbsPath::new(file).map_err(|err| match err {
                AbsolutePathError::Absolutize { io_error } => io_error,
            })?;
            let result = try_install_file(repository, install_base, &path_in_repo)?;
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

fn try_install_file(
    repository: &AbsPath,
    install_base: &AbsPath,
    path_in_repo: &AbsPath,
) -> Result<InstallStatus, std::io::Error> {
    let path_rel =
        RelPath::with_virtual_working_dir(path_in_repo, repository).map_err(|err| match err {
            RelativePathError::NoWorkingDirectory { io_error } => io_error,
            RelativePathError::Absolutize { io_error } => io_error,
            RelativePathError::PathDiff => {
                std::io::Error::new(std::io::ErrorKind::Other, "pathdiff error")
            }
        })?;

    let path_in_home =
        AbsPath::with_virtual_working_dir(&path_rel, install_base).map_err(|err| match err {
            AbsolutePathError::Absolutize { io_error } => io_error,
        })?;

    if path_in_home.as_ref().exists() {
        return Ok(InstallStatus::Skipped);
    }

    eprintln!(
        "creating symlink {} -> {}",
        path_in_home.as_ref().to_string_lossy(),
        path_in_repo.as_ref().to_string_lossy(),
    );
    crate::make_symlink(&path_in_home, &path_in_repo)?;

    Ok(InstallStatus::Installed)
}
