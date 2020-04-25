use crate::{
    config::{BlogConfig, PhotoConfig, PostConfig, PostLog},
    json_ld,
    models::{Category, Photo, TagPhotos},
};
use chrono::{DateTime, FixedOffset, Utc};
use core::cmp::Ordering;
use hashbrown::HashMap;
use serde_json;
use std::time::SystemTime;

/// Additional details for posts that are part of series
#[derive(Debug)]
pub struct PostSeries {
    /// Portion of path that is common among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `series_path` is `brother-ride`.
    pub path: String,

    /// Portion of path that is unique among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `parth_path` is
    /// `2.trying-to-survive`.
    pub part_path: String,

    pub title: String,

    /// Position of post in series
    pub part: u8,
    /// Total number of posts in the series
    pub total_parts: u8,
    /// Whether next post is part of the same series
    pub next_is_part: bool,
    /// Whether previous post is part of the same series
    pub prev_is_part: bool,
}
impl PostSeries {
    /// Navigation label to show if adjacent post is part of the same series.
    /// This is a convenience method used in template rendering.
    fn navigation_label(&self, to_part: u8) -> Option<String> {
        if to_part > 0 && to_part <= self.total_parts {
            Some(format!("Part {}", self.part))
        } else {
            None
        }
    }

    pub fn next_label(&self) -> Option<String> {
        self.navigation_label(self.part + 1)
    }

    pub fn prev_label(&self) -> Option<String> {
        self.navigation_label(self.part - 1)
    }
}

#[derive(Debug)]
pub struct Post {
    /// File path to the post
    ///
    /// *example* `brother-ride/2.trying-to-survive`
    pub path: String,

    pub series: Option<PostSeries>,

    /// When the depicted events happened
    pub happened_on: Option<DateTime<FixedOffset>>,

    /// When the post was last updated
    pub updated_on: DateTime<Utc>,

    pub title: String,

    pub summary: String,

    /// Whether post pictures occurred sequentially in a specific time range as
    /// opposed to, for example, a themed set of images from various times
    pub chronological: bool,

    /// Whether post is featured in main navigation rather than being included
    /// in regular post sequence and categories
    pub featured: bool,

    /// Photos found for the post. If post data were loaded from a previous
    /// render log then this will be empty.
    pub photos: Vec<Photo>,

    /// Next chronological post path (newer)
    pub next_path: Option<String>,
    /// Previous chronological post path (older)
    pub prev_path: Option<String>,

    /// Whether GPX track was found for the post
    pub has_track: bool,
    /// Categories to which this post belongs
    pub categories: Vec<Category>,

    pub photo_count: usize,

    /// Photo longitude/latitude pairs used to generate MapBox static maps
    /// https://docs.mapbox.com/api/maps/#retrieve-a-static-map-from-a-style
    pub photo_locations: Vec<(f32, f32)>,

    /// If post photos or configuration have changed, or an adjacent post has
    /// changed, then the post should be re-rendered.
    ///
    /// Posts that haven't changed don't re-parse their photos so `photos` is
    /// an empty vector.
    pub needs_render: bool,

    /// Zero-based index of cover photo within vector of photos
    pub cover_photo_index: usize,

    pub tags: HashMap<String, TagPhotos<u8>>,

    /// Record of previous post photos and configuration
    pub history: PostLog,
}

impl Post {
    /// Photo at `copy_photo_index` position
    pub fn cover_photo(&self) -> Option<&Photo> {
        let photo = if self.photos.is_empty() {
            //self.history.cover_photo.map(|ref p| p)
            match self.history.cover_photo {
                Some(ref p) => Some(p),
                None => None,
            }
        } else {
            self.photos.get(self.cover_photo_index)
        };

        if photo.is_none() {
            panic!(
                "No photo at index {} for {} among {} photos",
                self.cover_photo_index,
                self.title,
                self.photos.len()
            );
        }

        photo
    }

    /// Label (not title) for next post in series or `default` if the next post
    /// is not part of the same series. This is a convenience method for
    /// template rendering.
    pub fn next_label(&self, default: &str) -> String {
        if let Some(series) = &self.series {
            if let Some(label) = series.next_label() {
                return label;
            }
        }
        String::from(default)
    }

    /// Label (not title) for previous post in series or `default` if the
    /// previous post is not part of the same series. This is a convenience
    /// method for template rendering.
    pub fn prev_label(&self, default: &str) -> String {
        if let Some(series) = &self.series {
            if let Some(label) = series.prev_label() {
                return label;
            }
        }
        String::from(default)
    }
}

impl Default for Post {
    fn default() -> Self {
        Post {
            path: String::new(),

            happened_on: None,
            updated_on: DateTime::from(SystemTime::now()),
            title: String::new(),
            summary: String::new(),

            chronological: true,
            featured: false,
            photos: Vec::new(),

            next_path: None,
            prev_path: None,

            has_track: false,
            categories: Vec::new(),

            photo_count: 0,
            cover_photo_index: 0,
            needs_render: true,

            tags: HashMap::new(),
            history: PostLog::empty(),
            series: None,

            photo_locations: Vec::new(),
        }
    }
}

impl Post {
    pub fn from_config(config: PostConfig, log: PostLog) -> Self {
        Post {
            categories: config.categories(),
            title: config.title,
            summary: config.summary,
            cover_photo_index: config.cover_photo_index,
            chronological: config.chronological,
            history: log,
            ..Self::default()
        }
    }

    pub fn json_ld(&self, config: &BlogConfig) -> serde_json::Value {
        let image = self.cover_photo().map(|p| p.json_ld());
        let categories: Vec<String> = self
            .categories
            .iter()
            .map(|c: &Category| c.name.clone())
            .collect();

        serde_json::json!({
            "@type": "BlogPosting",
            "@context": json_ld::CONTEXT,
            "author": json_ld::owner(config),
            "name": &self.title,
            "headline": &self.title,
            "description": &self.summary,
            "image": image,
            "publisher": json_ld::organization(config),
            "mainEntityOfPage": json_ld::web_page(config, "about"),
            "datePublished": &self.happened_on.map(|d| d.to_rfc3339()),
            "dateModified": &self.updated_on.to_rfc3339(),
            "articleSection": categories.join(",")
        })
    }

    /// Build root-relative URLs for all post photo sizes
    pub fn build_photo_urls(&mut self, config: &PhotoConfig) {
        for p in self.photos.iter_mut() {
            p.size.build_urls(&self.path, p.index, config);
        }
    }

    /// Whether post has changed since it was last loaded
    pub fn changed(&self) -> bool {
        self.history.differs(self)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.happened_on.partial_cmp(&other.happened_on)
    }
}

impl Ord for Post {
    fn cmp(&self, other: &Post) -> Ordering {
        self.happened_on.cmp(&other.happened_on)
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Post {}
