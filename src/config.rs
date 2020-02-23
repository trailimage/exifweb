use serde::{Deserialize, Serialize};

pub type Pairs = Vec<(String, String)>;

/// Categories to which the post has been assigned.
#[derive(Deserialize, Debug)]
pub struct PostCategories {
    pub who: String,
    pub when: String,
    pub r#where: Vec<String>,
    pub what: String,
}

#[derive(Deserialize, Debug)]
pub struct SeriesConfig {
    /// Series title. This becomes the main post title while the configured
    /// post title becomes the subtitle.
    pub title: String,
    /// Number of parts in the series
    pub parts: u8,
}

#[derive(Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,
    /// Categories to which the post has been assigned
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

#[derive(Deserialize, Debug)]
pub struct ExifConfig {
    pub camera: Pairs,
    pub software: Pairs,
    pub lens: Pairs,
}

/// Photo sizes to generate from original.
#[derive(Deserialize, Debug)]
pub struct SizeConfig {
    large: u16,
    regular: u16,
    small: u16,
    thumb: u16,
}

#[derive(Deserialize, Debug)]
pub struct PhotoConfig {
    /// Regex pattern to extract photo index and count from file name
    ///
    /// *Exmaple* `(\\d{3})-of-(\\d{3})\\.jpg$` for `neat_place_012-of-015.jpg`
    index_pattern: String,
    size: SizeConfig,
    /// EXIF normalization settings
    exif: ExifConfig,
}

#[derive(Deserialize, Debug)]
pub struct BlogConfig {
    /// Regex pattern to extract series part index from folder name
    ///
    /// *Example* `^(\\d) - ` for `3 - Cold Ride Home`
    pub series_index_pattern: String,
    pub photo: PhotoConfig,
}
