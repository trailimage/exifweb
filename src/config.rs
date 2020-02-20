use serde::{Deserialize, Serialize};

pub type Pairs = Vec<(String, String)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExifConfig {
    pub camera: Pairs,
    pub software: Pairs,
    pub lens: Pairs,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub categories: Vec<String>,
    pub what: Vec<String>,
    pub exif: ExifConfig,
}
