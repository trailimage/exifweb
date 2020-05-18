//! RON logs

use super::load_ron;
use crate::{
    models::{Blog, Photo, PhotoPath, Post, TagPhotos},
    tools::write_result,
};
use chrono::{DateTime, FixedOffset, Local};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

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

    /// Timestamp when post data were last loaded
    pub as_of: i64,

    /// Number of photos in the post. If this changes then the post needs to be
    /// re-rendered.
    pub photo_count: usize,

    pub photo_locations: Vec<(f32, f32)>,

    /// Even if post hasn't changed, its cover photo may be required to re-
    /// render category pages it's part of
    pub cover_photo: Option<Photo>,

    /// Photo tags keyed by their slug to the photos they were assigned to.
    /// These are logged so that photo tag pages can be regenerated without
    /// re-parsing every post photo.
    pub tags: BTreeMap<String, TagPhotos<u8>>,

    /// Whether post source files have changed since they were last read
    #[serde(skip)]
    pub files_changed: bool,
}

impl PostLog {
    /// Save information about loaded photos to avoid unecessary re-processing
    pub fn write(root: &Path, post: &Post) {
        let log = PostLog {
            prev_path: post.prev_path.clone(),
            next_path: post.next_path.clone(),
            happened_on: post.happened_on,
            photo_count: post.photo_count,
            photo_locations: post.photo_locations.clone(),
            as_of: Local::now().timestamp(),
            tags: post.tags.clone(),
            files_changed: false,
            cover_photo: post.cover_photo().map(|p| p.clone()),
        };
        let path = root.join(&post.path).join(LOG_FILE);
        let pretty = PrettyConfig {
            depth_limit: 4,
            ..PrettyConfig::default()
        };

        write_result(&path, || to_string_pretty(&log, pretty), false);
    }

    /// Load log file from path
    pub fn load(path: &Path) -> Option<Self> {
        load_ron(path, LOG_FILE, false)
    }

    pub fn empty() -> PostLog {
        PostLog {
            next_path: None,
            prev_path: None,
            happened_on: None,
            as_of: 0,
            photo_count: 0,
            photo_locations: Vec::new(),
            tags: BTreeMap::new(),
            files_changed: true,
            cover_photo: None,
        }
    }

    /// Whether logged values differ from current post values
    pub fn sequence_changed(&self, post: &Post) -> bool {
        self.prev_path != post.prev_path || self.next_path != post.next_path
    }

    /// Whether photo GPS coordinates have changed
    pub fn locations_changed(&self, post: &Post) -> bool {
        self.photo_locations != post.photo_locations
    }

    pub fn cover_photo_changed(&self, post: &Post) -> bool {
        if self.cover_photo.is_none() != post.cover_photo().is_none() {
            true
        } else {
            self.cover_photo
                .as_ref()
                .map_or(false, |p| p == post.cover_photo().unwrap())
        }
    }

    /// Map images only need to be regenerated if the aspect ratio of the cover
    /// photo changes since, on the category page, it is fitted beside the
    /// cover image to fill the content width
    pub fn cover_aspect_ratio_changed(&self, post: &Post) -> bool {
        if self.cover_photo.is_none() != post.cover_photo().is_none() {
            true
        } else {
            self.cover_photo.as_ref().map_or(false, |p| {
                p.aspect_ratio() != post.cover_photo().unwrap().aspect_ratio()
            })
        }
    }
}

impl Clone for PostLog {
    fn clone(&self) -> Self {
        // let mut tags: BTreeMap<String, TagPhotos<u8>> = HashMap::new();

        // for (slug, tag_photos) in self.tags.iter() {
        //     tags.insert(slug.to_string(), tag_photos.clone());
        // }

        PostLog {
            next_path: self.next_path.clone(),
            prev_path: self.prev_path.clone(),
            happened_on: self.happened_on,
            as_of: self.as_of,
            photo_count: self.photo_count,
            photo_locations: self.photo_locations.clone(),
            tags: self.tags.clone(),
            files_changed: self.files_changed,
            cover_photo: if let Some(p) = &self.cover_photo {
                Some(p.clone())
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlogLog {
    // use B-Tree so that keys are sorted
    pub tags: BTreeMap<String, TagPhotos<PhotoPath>>,
}

impl BlogLog {
    /// Save information about photo tags to avoid unecessary re-processing
    pub fn write(root: &Path, blog: &Blog) {
        let log = BlogLog {
            tags: blog.tags.clone(),
        };
        let path = root.join(LOG_FILE);
        let pretty = PrettyConfig {
            depth_limit: 4,
            ..PrettyConfig::default()
        };

        write_result(&path, || to_string_pretty(&log, pretty), false);
    }

    pub fn empty() -> BlogLog {
        BlogLog {
            tags: BTreeMap::new(),
        }
    }

    /// Load log file from path
    pub fn load(path: &Path) -> Option<Self> {
        load_ron(path, LOG_FILE, false)
    }
}
