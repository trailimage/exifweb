use super::load_config;
use serde::Deserialize;
use std::path::Path;
#[derive(Deserialize, Debug)]
pub struct SeriesConfig {
    /// Series title. This becomes the main post title while the configured
    /// post title becomes the subtitle.
    pub title: String,
    /// Number of parts in the series
    pub parts: u8,
}

impl SeriesConfig {
    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path)
    }
}
