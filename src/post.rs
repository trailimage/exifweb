use crate::Photo;
use std::path::PathBuf;

#[derive(Default)]
pub struct Post<'a> {
    /// Unique identifer used as the URL slug. If post is part of a series then
    /// the key is compound.
    ///
    /// *example* `brother-ride/day-10`
    pub key: String,
    /// Portion of key that is common among series members. For example, with
    /// `brother-ride/day-10` the `seriesKey` is `brother-ride`.
    pub series_key: String,
    /// Portion of key that is unique among series members. For example, with
    /// `brother-ride/day-10` the `partKey` is `day-10`.
    pub part_key: String,

    pub path: PathBuf,
    pub title: String,
    pub sub_title: String,
    pub original_title: String,
    pub summary: String,
    /// Whether post pictures occurred sequentially in a specific time range as
    /// opposed to, for example, a themed set of images from various times.
    pub chronological: bool,
    /// Whether post is featured in main navigation.
    pub featured: bool,
    pub cover_photo: Option<&'a Photo>,
    pub photos: Vec<&'a Photo>,

    /// Next chronological post (newer).
    pub next: Option<&'a Post<'a>>,
    /// Previous chronological post (older).
    pub prev: Option<&'a Post<'a>>,

    /// Position of this post in a series or 0 if it's not in a series.
    pub part: i8,
    /// Whether post is part of a series.
    pub is_partial: bool,
    /// Whether next post is part of the same series.
    pub next_is_part: bool,
    /// Whether previous post is part of the same series.
    pub previous_is_part: bool,
    /// Total number of posts in the series.
    pub total_parts: i8,
    /// Whether this post is the first in a series.
    pub is_series_start: bool,
    /// Whether GPX track was found for the post.
    pub has_track: bool,
}
