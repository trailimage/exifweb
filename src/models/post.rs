use crate::{
    config::{BlogConfig, PostConfig, PostLog},
    json_ld,
    models::{collate_tags, Category, Photo, TagPhotos},
    tools::earliest_photo_date,
};
use chrono::{DateTime, FixedOffset, Utc};
use core::cmp::Ordering;
use serde_json;
use std::{collections::BTreeMap, time::SystemTime};

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

    /// Zero-based index of cover photo within vector of photos
    pub cover_photo_index: usize,

    pub tags: BTreeMap<String, TagPhotos<u8>>,

    /// Record of previous post photos and configuration
    pub history: PostLog,

    /// Width/height of cover map
    pub cover_map_size: (u16, u16),
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

    pub fn add_photos(&mut self, photos: Vec<Photo>) {
        let mut locations: Vec<(f32, f32)> = Vec::new();

        for p in &photos {
            if let Some(l) = &p.location {
                locations.push(l.as_tuple());
            }
        }

        locations.sort_by(|a, b| {
            if a.0 > b.0 {
                Ordering::Greater
            } else if a.0 < b.0 {
                Ordering::Less
            } else {
                if a.1 > b.1 {
                    Ordering::Greater
                } else if a.1 < b.1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });

        if self.chronological {
            self.happened_on = earliest_photo_date(&photos)
        }

        self.tags = collate_tags(&photos);
        self.photo_locations = locations;
        self.photo_count = photos.len();
        self.photos = photos;
    }

    pub fn has_video(&self) -> bool {
        false
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
            cover_map_size: (0, 0),

            tags: BTreeMap::new(),
            history: PostLog::empty(),
            series: None,

            photo_locations: Vec::new(),
        }
    }
}

impl Post {
    pub fn from_config(config: PostConfig, log: PostLog) -> Self {
        // one-based index of cover photo
        let i = config.cover_photo_index;

        Post {
            categories: config.categories(),
            title: config.title,
            summary: config.summary,
            // convert to zero-based index
            cover_photo_index: if i > 0 { i - 1 } else { 0 },
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

    /// Build root-relative URLs for all post photo sizes and compute cover map
    /// dimensions to fit next to small image within content width
    pub fn prepare_maps(&mut self, config: &BlogConfig) {
        let max_height = config.style.inline_map_height;
        let max_width = config.style.content_width;

        if let Some(p) = self.cover_photo() {
            let height = if p.is_portrait() {
                // limit height next to portrait images
                max_height
            } else {
                p.size.small.height
            };
            let width = max_width - p.size.small.width;

            self.cover_map_size = (width, height);
        } else {
            self.cover_map_size = (max_width, max_height);
        }
    }

    /// Whether next or previous posts have changed
    pub fn sequence_changed(&self) -> bool {
        self.history.sequence_changed(self)
    }

    /// Whether photo GPS locations have changed
    pub fn locations_changed(&self) -> bool {
        self.history.locations_changed(self)
    }

    /// Whether photos or configuration have changed
    pub fn files_changed(&self) -> bool {
        self.history.files_changed
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
