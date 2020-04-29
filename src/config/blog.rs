//! TOML blog configuration

use super::{
    env_or_empty, load_config,
    vendors::{FacebookConfig, GoogleConfig, MapBoxConfig},
    ReadsEnv,
};
use crate::{deserialize::regex_string, models::Location, tools::Pairs};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

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
    pub large: u16,
    pub medium: u16,
    pub small: u16,
    pub thumb: u16,
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
    /// Extension (*with* leading period) of source files from which published
    /// web files are rendered
    pub source_ext: String,
    /// Extension (*with* leading period) applied to resized photos
    pub output_ext: String,
    /// Maximum edge size of source image. This may be used to determine if a
    /// resize is required for the largest photo.
    pub source_size: u16,
}

#[derive(Deserialize, Debug)]
pub struct CategoryConfig {
    /// Match name of "what" category to transportation mode that may in turn
    /// match an icon
    pub what_regex: Option<Pairs>,
    pub icon: CategoryIcon,
    /// Which category kinds to display and in what order
    pub display: Vec<String>,
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
pub struct ImageConfig {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Debug)]
pub struct SiteConfig {
    pub url: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub logo: ImageConfig,
    pub company_logo: Option<ImageConfig>,

    /// Generic name for a post (usually just "post") that can be used in a
    /// category page subtitle, e.g. "27 posts" and pluralized with just an `s`
    #[serde(default = "default_post_alias")]
    pub post_alias: String,
}

#[derive(Deserialize, Debug)]
pub struct OwnerConfig {
    pub name: String,
    #[serde(skip)]
    pub email: Option<String>,
    pub urls: Option<Vec<String>>,
    pub image: Option<ImageConfig>,
}
impl ReadsEnv for OwnerConfig {
    fn from_env(&mut self) {
        self.email = Some(env_or_empty("EMAIL_CONTACT"))
    }
}

#[derive(Deserialize, Debug)]
pub struct FeaturedPost {
    pub path: String,
    // title will be retrieved from actual post
    #[serde(skip)]
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct BlogConfig {
    pub author_name: String,
    pub repo_url: String,

    pub featured_post: Option<FeaturedPost>,

    /// Folders known not to contain posts
    pub ignore_folders: Vec<String>,

    /// Whether to render post pages even if there have been no changes since
    /// the last render. This is set with a `--force` argument rather than
    /// configuration.
    #[serde(skip)]
    pub force_rerender: bool,

    /// Redirect source slug to target
    pub redirects: Option<Pairs>,
    pub site: SiteConfig,
    pub owner: OwnerConfig,
    pub style: StyleConfig,
    pub category: CategoryConfig,
    pub photo: PhotoConfig,
    pub facebook: FacebookConfig,
    pub mapbox: MapBoxConfig,
    pub google: GoogleConfig,
}

impl BlogConfig {
    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path).and_then(|mut c| {
            c.from_env();
            Some(c)
        })
    }
}

impl ReadsEnv for BlogConfig {
    fn from_env(&mut self) {
        self.mapbox.from_env();
        self.google.from_env();
        self.owner.from_env();
    }
}

fn default_post_alias() -> String {
    String::from("Post")
}
