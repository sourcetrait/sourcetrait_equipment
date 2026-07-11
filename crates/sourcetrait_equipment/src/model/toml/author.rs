use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthorToml {
    pub key: String,
    pub title: String,
    pub email: String,
}
