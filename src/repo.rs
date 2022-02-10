use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RepoInfo {
    #[serde(default)]
    pub dirs: Vec<PathBuf>,
}
