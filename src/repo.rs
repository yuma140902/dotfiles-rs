use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RepoInfo {
    #[serde(default)]
    pub files: Vec<PathBuf>,

    #[serde(default)]
    pub dirs: Vec<PathBuf>,
}
