use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}
