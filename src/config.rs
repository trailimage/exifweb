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
pub struct SeriesConfig {
    /// Series title
    pub title: String,
    /// Number of parts in the series
    pub parts: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,
    pub categories: PostCategories,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostPhotos {
    /// Date of second photo in folder (skip first since it's often a contextual
    /// shot from another time)
    pub when: time::Date,
    /// When folder was last processed
    pub processed: time::Date,
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
    large: u16,
    regular: u16,
    small: u16,
    thumb: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoConfig {
    size: SizeConfig,
    exif: ExifConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogConfig {
    pub photo: PhotoConfig,
}
