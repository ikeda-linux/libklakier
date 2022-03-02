use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub tracked_files: Vec<String>,
    pub dependencies: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub reccomendations: Option<Vec<String>>,
    pub arch: String,
}

pub struct Reccomendation {
    pub requirements: Vec<String>,
    pub reccomendations: Vec<String>,
}
