use crate::config::ExifConfig;
use crate::{Category, Post};
use chrono::{DateTime, Local};
use hashbrown::HashMap;

/// Ephemeral struct to compute and capture chronological post order
struct KeyTime {
    key: String,
    time: DateTime<Local>,
}

pub struct PhotoTag {
    /// Original tag name (not slugified)
    pub name: String,
    /// List of photos that have the tag applied. The contents should be the
    /// post slug plus the photo name as a URL hash that together can be used
    /// as a direct link that scrolls to the photo in a post.
    ///
    /// *example* `brother-ride-2018/place-we-went#004`
    pub link: Vec<String>,
}

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: HashMap<String, Post>,
    pub categories: Vec<Category<'a>>,
    /// Tag slugs mapped to the original tag names and photos with the tag
    pub tags: HashMap<String, PhotoTag>,
}

impl<'a> Blog<'a> {
    pub fn add_post(&mut self, p: Post) {
        if let Some(dup) = self.posts.insert(p.key.clone(), p) {
            // if insert returns Post then same key was already present
            panic!("Attempt to insert duplicate post {}", dup.key)
        }
    }

    /// Update post `prev_key` and `next_key` based on chronological ordering
    pub fn correlate_posts(&mut self) {
        let mut ordered: Vec<KeyTime> = Vec::new();

        for kt in
            self.posts
                .values()
                .filter(|p| p.chronological)
                .map(|p| KeyTime {
                    key: p.key.clone(),
                    time: p.happened_on,
                })
        {
            ordered.push(kt);
        }

        // sort post keys oldest to newest
        ordered.sort_by(|a, b| a.time.cmp(&b.time));

        let len = ordered.len();

        for (k, p) in self.posts.iter_mut() {
            // sorted position of post
            let i = ordered.iter().position(|kt| kt.key == *k).unwrap();

            if i > 0 {
                p.prev_key = ordered.get(i - 1).unwrap().key.clone()
            }
            if i < len - 1 {
                p.next_key = ordered.get(i + 1).unwrap().key.clone();
            }
        }
    }

    /// Sanitize EXIF in all post photos
    pub fn sanitize_exif(&mut self, config: &ExifConfig) {
        for (_, p) in self.posts.iter_mut() {
            for photo in p.photos.iter_mut() {
                photo.sanitize(config);
            }
        }
    }

    ///
    pub fn collate_tags(&mut self) {
        let tags: HashMap<String, PhotoTag> = HashMap::new();

        self.tags = tags;
    }
}
