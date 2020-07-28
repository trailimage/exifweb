use super::load_config;
use crate::tools::folder_name;
use serde::Deserialize;
use std::path::Path;

/// Configuration for posts that form a series
#[derive(Deserialize, Debug)]
pub struct SeriesConfig {
    /// Series title. This becomes the main post title while the configured
    /// post title becomes the subtitle.
    pub title: String,
    /// Number of parts in the series. The part number of each post is inferred
    /// from its folder name using `capture_series_index` configuration.
    pub parts: u8,

    /// Root-relative path to series
    #[serde(skip)]
    pub path: String,
}

impl SeriesConfig {
    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path).map(|mut c: SeriesConfig| {
            c.path = folder_name(path).to_string();
            c
        })
    }
}
