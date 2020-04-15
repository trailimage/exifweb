use crate::models::{Category, Photo};
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use yarte::Template;

#[derive(Debug, Template)]
#[template(path = "post.html")]
pub struct Post {
    /// File path to the post
    ///
    /// *example* `brother-ride/2.trying-to-survive`
    pub path: String,
    /// Portion of path that is common among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `series_path` is `brother-ride`.
    pub series_path: String,
    /// Portion of path that is unique among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `parth_path` is
    /// `2.trying-to-survive`.
    pub part_path: String,

    /// When the depicted events happened
    pub happened_on: Option<DateTime<FixedOffset>>,
    /// When the post was created
    //pub created_on: DateTime<FixedOffset>,
    /// When the post was last updated
    //pub updated_on: DateTime<FixedOffset>,

    /// Title of the post. For series, this will be the series title and the
    /// configured post title will become the `sub_title`.
    pub title: String,
    /// Subtitle of the post. For series, this will be the title the post was
    /// configured with while the post's `title` will be series title.
    pub sub_title: String,
    //pub original_title: String,
    pub summary: String,

    /// Whether post pictures occurred sequentially in a specific time range as
    /// opposed to, for example, a themed set of images from various times.
    pub chronological: bool,
    /// Whether post is featured in main navigation
    pub featured: bool,
    pub photos: Vec<Photo>,

    /// Next chronological post path (newer)
    pub next_path: String,
    /// Previous chronological post path (older)
    pub prev_path: String,

    /// One-based position of this post in a series or 0 if it's not in a series
    pub part: u8,
    /// Whether post is part of a series
    pub is_partial: bool,
    /// Whether next post is part of the same series
    pub next_is_part: bool,
    /// Whether previous post is part of the same series
    pub prev_is_part: bool,
    /// Total number of posts in the series
    pub total_parts: u8,
    /// Whether this post is the first in a series
    pub is_series_start: bool,
    /// Whether GPX track was found for the post
    pub has_track: bool,
    /// Categories to which this post belongs
    pub categories: Vec<Category>,
}

impl Post {
    /// First photo flagged as `primary`
    pub fn cover_photo(&self) -> Option<&Photo> {
        self.photos.iter().find(|p| p.primary)
    }
}

impl Default for Post {
    fn default() -> Self {
        Post {
            path: String::new(),
            series_path: String::new(),
            part_path: String::new(),

            happened_on: None,
            //created_on: min_date(),
            //updated_on: min_date(),
            title: String::new(),
            sub_title: String::new(),
            //original_title: "",
            summary: String::new(),

            chronological: true,
            featured: false,
            photos: Vec::new(),

            next_path: String::new(),
            prev_path: String::new(),

            part: 0,
            total_parts: 0,
            is_partial: false,
            next_is_part: false,
            prev_is_part: false,
            is_series_start: false,
            has_track: false,

            categories: Vec::new(),
        }
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
