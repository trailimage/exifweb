//! TOML configuration models

use crate::deserialize::regex_string;
use crate::models::Location;
use crate::tools::Pairs;
use chrono::{DateTime, FixedOffset, Local};
use regex::Regex;
use serde::{Deserialize, Serialize};

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
    pub access_token: String,
    /// Maximum number of photo markers to show on static map
    pub max_static_markers: u16,
    pub style: MapBoxStyles,
}

#[derive(Deserialize, Debug)]
pub struct GoogleConfig {
    pub api_key: String,
    pub project_id: String,
    /// Shown as `UA-<analytics_id>-1`
    pub analytics_id: String,
    pub search_engine_id: String,
    pub blog_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostPhotos {
    /// Date of first relevant photo in folder
    pub when: Option<DateTime<FixedOffset>>,
    /// When folder was last processed
    pub processed: DateTime<Local>,
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
