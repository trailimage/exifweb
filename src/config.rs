//! TOML configuration models

use crate::tools::Pairs;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Categories to which the post has been assigned
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
    /// One-based index of cover photo
    pub cover_photo_index: u8,

    pub youtube_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostPhotos {
    /// Date of second photo in folder (skip first since it's often a contextual
    /// shot from another time)
    pub when: SystemTime,
    /// When folder was last processed
    pub processed: SystemTime,
    /// Photo tags
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExifConfig {
    pub camera: Pairs,
    pub software: Pairs,
    pub lens: Pairs,
}

/// Photo sizes to generate from original
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
    pub index_regex: String,
    pub size: SizeConfig,
    /// EXIF normalization settings
    pub exif: ExifConfig,
}

#[derive(Deserialize, Debug)]
pub struct CategoryConfig {
    pub icon: CategoryIcon,
    pub what_regex: Option<Pairs>,
}

/// Match category kind to material icon
/// https://material.io/icons/
#[derive(Deserialize, Debug)]
pub struct CategoryIcon {
    pub who: String,
    pub what: String,
    pub when: String,
    pub r#where: String,
    pub default: String,
}

#[derive(Deserialize, Debug)]
pub struct StyleConfig {
    /// Maximum pixel height of static maps displayed with post summaries
    pub pub_inline_map_height: i8,
    /// Pixel width of main content used to compute generated image widths
    pub content_width: i8,
}

#[derive(Deserialize, Debug)]
pub struct BlogConfig {
    /// Regex pattern to extract series part index from folder name
    ///
    /// *Examples*
    ///  - `^(\d) - ` for `3 - Cold Ride Home`
    ///  - `^(\d)\.` for `3.cold-ride-home`
    pub series_index_regex: String,
    /// Redirect source slug to target
    pub redirects: Option<Pairs>,
    pub style: StyleConfig,
    pub category: CategoryConfig,
    pub photo: PhotoConfig,
}
