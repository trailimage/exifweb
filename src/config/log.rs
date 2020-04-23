//! TOML photo log

use super::load_ron;
use crate::models::{Post, TagPhotos};
use crate::tools::write_result;
use chrono::{DateTime, FixedOffset, Local};
use hashbrown::HashMap;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// File that stores photo tag information and last process time
static LOG_FILE: &str = "log.ron";

/// Log processed photo information per post folder to determine when
/// re-processing is necessary.
///
/// Re-rendering is triggered when
///
/// - the configuration file or a photo has a modified date newer than
///   the `as_of` time
/// - the number of photos has changed
/// - adjacent post paths have changed
///
/// Re-generating an image is triggered when
///
/// - its modified date is newer than the `as_of` time
///
#[derive(Serialize, Deserialize, Debug)]
pub struct PostLog {
    #[serde(default)]
    pub next_path: Option<String>,
    #[serde(default)]
    pub prev_path: Option<String>,

    /// Date of first relevant (not an outlier) photo in folder
    pub happened_on: Option<DateTime<FixedOffset>>,

    /// When post data were last loaded
    pub as_of: DateTime<Local>,

    /// Number of photos in the post. If this changes then the post needs to be
    /// re-rendered.
    pub photo_count: usize,

    /// Photo tags keyed by their slug to the photos they were assigned to.
    /// These are logged so that photo tag pages can be regenerated.
    pub tags: HashMap<String, TagPhotos<u8>>,
}

impl PostLog {
    /// Save information about loaded photos to avoid unecessary re-processing
    pub fn write(root: &Path, post: &Post) {
        let log = PostLog {
            prev_path: post.prev_path.clone(),
            next_path: post.next_path.clone(),
            happened_on: post.happened_on,
            photo_count: post.photo_count,
            as_of: Local::now(),
            tags: post.tags.clone(),
        };
        let path = root.join(&post.path).join(LOG_FILE);
        let pretty = PrettyConfig {
            depth_limit: 4,
            ..PrettyConfig::default()
        };

        write_result(&path, || to_string_pretty(&log, pretty));
    }

    pub fn load(path: &Path) -> Option<Self> {
        load_ron(path, LOG_FILE, false)
    }
}

impl Clone for PostLog {
    fn clone(&self) -> Self {
        let mut tags: HashMap<String, TagPhotos<u8>> = HashMap::new();

        for (slug, tag_photos) in self.tags.iter() {
            tags.insert(slug.to_string(), tag_photos.clone());
        }

        PostLog {
            next_path: self.next_path.clone(),
            prev_path: self.prev_path.clone(),
            happened_on: self.happened_on,
            as_of: self.as_of,
            photo_count: self.photo_count,
            tags,
        }
    }
}
