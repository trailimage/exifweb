use crate::photo::Photo;
use crate::tools::min_date;
use chrono::{DateTime, Local};
use core::cmp::Ordering;
use yarte::Template;

#[derive(Debug, Template)]
#[template(path = "post.html")]
pub struct Post {
    /// Unique identifer used as the URL slug. If post is part of a series then
    /// the key is compound.
    ///
    /// *example* `brother-ride/2.trying-to-survive`
    pub key: String,
    /// Portion of key that is common among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `seriesKey` is `brother-ride`.
    pub series_key: String,
    /// Portion of key that is unique among series members. For example, with
    /// `brother-ride/2.trying-to-survive` the `partKey` is
    /// `2.trying-to-survive`.
    pub part_key: String,

    /// When the depicted events happened
    pub happened_on: DateTime<Local>,
    /// When the post was created
    pub created_on: DateTime<Local>,
    /// When the post was last updated
    pub updated_on: DateTime<Local>,

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
    /// Whether post is featured in main navigation.
    pub featured: bool,
    pub photos: Vec<Photo>,

    /// Next chronological post (newer).
    pub next_key: String,
    /// Previous chronological post (older).
    pub prev_key: String,

    /// One-based position of this post in a series or 0 if it's not in a series.
    pub part: u8,
    /// Whether post is part of a series.
    pub is_partial: bool,
    /// Whether next post is part of the same series.
    pub next_is_part: bool,
    /// Whether previous post is part of the same series.
    pub prev_is_part: bool,
    /// Total number of posts in the series.
    pub total_parts: u8,
    /// Whether this post is the first in a series.
    pub is_series_start: bool,
    /// Whether GPX track was found for the post.
    pub has_track: bool,
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
            key: String::new(),
            series_key: String::new(),
            part_key: String::new(),

            happened_on: min_date(),
            created_on: min_date(),
            updated_on: min_date(),

            title: String::new(),
            sub_title: String::new(),
            //original_title: "",
            summary: String::new(),

            chronological: true,
            featured: false,
            photos: Vec::new(),

            next_key: String::new(),
            prev_key: String::new(),

            part: 0,
            total_parts: 0,
            is_partial: false,
            next_is_part: false,
            prev_is_part: false,
            is_series_start: false,
            has_track: false,
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
        self.key == other.key
    }
}

impl Eq for Post {}
