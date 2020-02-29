use crate::Photo;
use core::cmp::Ordering;
use lazy_static::*;
use regex::Regex;
use std::time::SystemTime;

#[derive(Debug)]
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

    /// When the depicted events happened
    pub happened_on: SystemTime,
    /// When the post was created
    pub created_on: SystemTime,
    /// When the post was last updated
    pub updated_on: SystemTime,

    pub title: String,
    pub sub_title: String,
    //pub original_title: String,
    pub summary: String,
    /// Whether post pictures occurred sequentially in a specific time range as
    /// opposed to, for example, a themed set of images from various times.
    pub chronological: bool,
    /// Whether post is featured in main navigation.
    pub featured: bool,
    pub cover_photo: Option<&'a Photo>,
    pub photos: Vec<&'a Photo>,

    /// Next chronological post (newer).
    pub next_key: String,
    /// Previous chronological post (older).
    pub prev_key: String,

    /// Position of this post in a series or 0 if it's not in a series.
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

impl Default for Post<'_> {
    fn default() -> Self {
        Post {
            key: String::new(),
            series_key: String::new(),
            part_key: String::new(),

            happened_on: SystemTime::UNIX_EPOCH,
            created_on: SystemTime::UNIX_EPOCH,
            updated_on: SystemTime::UNIX_EPOCH,

            title: String::new(),
            sub_title: String::new(),
            //original_title: "",
            summary: String::new(),

            chronological: true,
            featured: false,
            cover_photo: None,
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

impl PartialOrd for Post<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.happened_on.partial_cmp(&other.happened_on)
    }
}

impl Ord for Post<'_> {
    fn cmp(&self, other: &Post) -> Ordering {
        self.happened_on.cmp(&other.happened_on)
    }
}

impl PartialEq for Post<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Post<'_> {}

pub fn slugify(s: &str) -> String {
    lazy_static! {
        static ref MIX_CASE: Regex = Regex::new(r"([a-z])([A-Z])").unwrap();
        static ref UNDERSCORE: Regex = Regex::new(r"[_\s/-]+").unwrap();
        static ref NON_LETTER: Regex = Regex::new(r"[^\-a-z0-9]").unwrap();
        static ref MULTI_DASH: Regex = Regex::new(r"-{2,}").unwrap();
    }

    let mut text: String = MIX_CASE.replace_all(s, "$1-$2").to_lowercase();
    text = UNDERSCORE.replace_all(&text, "-").replace("-&-", "-and-");
    text = NON_LETTER.replace_all(&text, "").into_owned();

    MULTI_DASH.replace_all(&text, "-").into_owned()
}

#[cfg(test)]
mod tests {
    use super::slugify;
    use hashbrown::HashMap;

    #[test]
    fn slugify_test() {
        let expect: HashMap<&str, &str> = [
            ("Wiggle and Roll", "wiggle-and-roll"),
            ("Wiggle and    Sing", "wiggle-and-sing"),
            ("Too---dashing", "too-dashing"),
            ("powerful/oz", "powerful-oz"),
            ("three o' clock", "three-o-clock"),
            ("one_two_Three-48px", "one-two-three-48px"),
            ("camelCase", "camel-case"),
        ]
        .iter()
        .cloned()
        .collect();

        for (k, v) in expect.iter() {
            assert_eq!(slugify(k), *v);
        }
    }
}
