//! TOML blog configuration

use super::{env_or_empty, load_config, ReadsEnv};
use crate::deserialize::regex_string;
use crate::models::Location;
use crate::tools::Pairs;
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

// https://developers.facebook.com/docs/reference/plugins/like/
// https://developers.facebook.com/apps/110860435668134/summary
#[derive(Deserialize, Debug)]
pub struct FacebookConfig {
    pub app_id: String,
    pub admin_id: String,
    pub page_id: String,
    pub site_id: String,
    pub author_url: String,
}

#[derive(Deserialize, Debug)]
pub struct MapBoxStyles {
    pub dynamic: String,
    pub r#static: String,
}

#[derive(Deserialize, Debug)]
pub struct MapBoxConfig {
    #[serde(skip)]
    pub access_token: String, // env::var("MAPBOX_ACCESS_TOKEN")
    /// Maximum number of photo markers to show on static map
    pub max_static_markers: u16,
    pub style: MapBoxStyles,
}

impl ReadsEnv for MapBoxConfig {
    fn from_env(&mut self) {
        self.access_token = env_or_empty("MAPBOX_ACCESS_TOKEN")
    }
}

#[derive(Deserialize, Debug)]
pub struct GoogleConfig {
    #[serde(skip)]
    pub api_key: String, // env::var("GOOGLE_KEY")
    pub project_id: String,
    /// Shown as `UA-<analytics_id>-1`
    pub analytics_id: String,
    #[serde(skip)]
    pub search_engine_id: String, // env::var("GOOGLE_SEARCH_ID")
    pub blog_id: String,
}

impl ReadsEnv for GoogleConfig {
    fn from_env(&mut self) {
        self.api_key = env_or_empty("GOOGLE_KEY");
        self.search_engine_id = env_or_empty("GOOGLE_SEARCH_ID");
    }
}

/// Replacement camera, lens and software text
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
    #[serde(deserialize_with = "regex_string")]
    pub capture_index: Regex,
    pub size: SizeConfig,
    /// EXIF normalization settings
    pub exif: ExifConfig,
    /// Tags to exclude from rendered views
    pub remove_tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CategoryConfig {
    /// Match name of "what" category to transportation mode that may in turn
    /// match an icon
    pub what_regex: Option<Pairs>,
    pub icon: CategoryIcon,
}

#[derive(Deserialize, Debug)]
pub struct GpsPrivacy {
    /// Erase tracks around given latitude and longitude
    pub center: Location,
    /// Radius around privacyCenter to exclude from GeoJSON
    pub miles: usize,
    pub verify: bool,
}

#[derive(Deserialize, Debug)]
pub struct GpsTrackConfig {
    pub min_track_points: usize,

    /// Distance a track point must deviate from others to avoid Douglas-Peucker
    /// simplification
    pub max_point_deviation_feet: f32,

    /// Manually adjusted tracks may have infinite speeds between points so
    /// throw out anything over a threshold
    pub max_possible_speed_mph: f32,

    pub privacy: Option<GpsPrivacy>,

    /// Whether track GPX files can be downloaded
    pub allow_download: bool,
    // Link patterns to external maps with `lat`, `lon`, `zoom` and `altitude`
    // tokens
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
    pub inline_map_height: u16,
    /// Pixel width of main content used to compute generated image widths
    pub content_width: u16,
}

#[derive(Deserialize, Debug)]
pub struct SiteConfig {
    pub url: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct BlogConfig {
    pub author_name: String,
    pub repo_url: String,
    /// Folders known not to contain posts
    pub ignore_folders: Vec<String>,

    /// Whether to render post pages even if there have been no changes since
    /// the last render. This is set with a `--force` argument rather than
    /// configuration.
    #[serde(skip)]
    pub force_rerender: bool,

    /// Regex pattern to extract series part index from folder name
    ///
    /// *Examples*
    ///  - `^(\d) - ` for `3 - Cold Ride Home`
    ///  - `^(\d)\.` for `3.cold-ride-home`
    #[serde(deserialize_with = "regex_string")]
    pub capture_series_index: Regex,

    /// Redirect source slug to target
    pub redirects: Option<Pairs>,
    pub site: SiteConfig,
    pub style: StyleConfig,
    pub category: CategoryConfig,
    pub photo: PhotoConfig,
    pub facebook: FacebookConfig,
    pub mapbox: MapBoxConfig,
    pub google: GoogleConfig,
}

impl BlogConfig {
    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path)
    }
}
