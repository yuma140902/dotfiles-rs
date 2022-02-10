use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(default)]
    pub dirs: Vec<PathBuf>,
}
