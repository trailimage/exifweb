use serde::{Deserialize, Serialize};

pub type Pairs = Vec<(String, String)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCategories {
    pub who: String,
    pub when: String,
    pub r#where: Vec<String>,
    pub what: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,
    pub categories: PostCategories,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostFound {
    /// Date of second photo in folder (skip first since it's often a contextual
    /// shot from another time)
    pub when: String,
    /// Timestamp when folder was last processed
    pub processed: i32,
    /// Photo tags
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExifConfig {
    pub camera: Pairs,
    pub software: Pairs,
    pub lens: Pairs,
}

/// Photo sizes to create.
#[derive(Serialize, Deserialize, Debug)]
pub struct SizeConfig {
    large: i8,
    regular: i8,
    small: i8,
    thumb: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoConfig {
    size: SizeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogConfig {
    pub categories: Vec<String>,
    pub what: Vec<String>,
    pub exif: ExifConfig,
    pub photo: PhotoConfig,
}
