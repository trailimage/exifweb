use serde::{Deserialize, Serialize};

pub type Pairs = Vec<(String, String)>;
#[derive(Serialize, Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,
    pub when: String,
    pub location: Vec<String>,
    pub what: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostFound {
    pub when: String,
    /// Photo tags
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExifConfig {
    pub camera: Pairs,
    pub software: Pairs,
    pub lens: Pairs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogConfig {
    pub categories: Vec<String>,
    pub what: Vec<String>,
    pub exif: ExifConfig,
}
