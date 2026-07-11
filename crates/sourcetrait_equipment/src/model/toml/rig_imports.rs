use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RigImportsToml {
    #[serde(alias = "libraries")]
    pub library: Vec<String>
}

