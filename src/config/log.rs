//! TOML photo log

use super::load_toml;
use crate::models::{Post, TagPhotos};
use crate::tools::write_result;
use chrono::{DateTime, FixedOffset, Local};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// File that stores photo tag information and last process time
static LOG_FILE: &str = "log.toml";

/// Log processed photo information per post folder to determine when
/// re-processing is necessary. Re-rendering is triggered when
///
/// - the configuration file or a photo has a modified date newer than
///   the `processed` time
/// - the number of photos has changed
/// - adjacent post paths have changed
///
#[derive(Serialize, Deserialize, Debug)]
pub struct PostLog {
    pub next_path: String,
    pub prev_path: String,

    /// Date of first relevant (not an outlier) photo in folder
    pub happened_on: Option<DateTime<FixedOffset>>,
    /// When post data were last loaded
    pub as_of: DateTime<Local>,
    /// Number of photos in the post. If this changes then the post needs to be
    /// re-rendered.
    pub photo_count: usize,
    /// Photo tags keyed by their slug to the photos they were assigned to.
    /// These need to be logged so
    pub tags: HashMap<String, TagPhotos>,
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

        write_result(&path, || toml::to_string(&log));
    }

    pub fn load(path: &Path) -> Option<Self> {
        load_toml(path, LOG_FILE, false)
    }
}

impl Clone for PostLog {
    fn clone(&self) -> Self {
        let mut tags: HashMap<String, TagPhotos> = HashMap::new();

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
