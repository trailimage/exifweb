use crate::config::ExifConfig;
use crate::tools::slugify;
use crate::{Category, Post};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;

/// Ephemeral struct to compute and capture chronological post order
struct KeyTime {
    /// Post key
    key: String,
    /// Post `happened_on` date
    time: DateTime<FixedOffset>,
}

/// Unique path to any blog photo
pub struct PhotoPath {
    pub post_key: String,
    /// Photo file name without extension
    pub photo_name: String,
}

pub struct TagPhotos {
    /// Original tag name (not slugified)
    pub name: String,
    /// Photos that have the tag applied
    pub photos: Vec<PhotoPath>,
}

#[derive(Default)]
pub struct Blog<'a> {
    /// Posts keyed to their slug
    pub posts: HashMap<String, Post>,
    pub categories: Vec<Category<'a>>,
    /// Tag slugs mapped to the original tag names and photos with the tag
    pub tags: HashMap<String, TagPhotos>,
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

        for kt in self
            .posts
            .values()
            .filter(|p: &'_ &Post| p.chronological && p.happened_on.is_some())
            .map(|p: &Post| KeyTime {
                key: p.key.clone(),
                time: p.happened_on.unwrap(),
            })
        {
            ordered.push(kt);
        }

        // sort post keys oldest to newest
        ordered.sort_by(|a, b| a.time.cmp(&b.time));

        let len = ordered.len();

        for (k, p) in self.posts.iter_mut() {
            // sorted position of post
            if let Some(i) = ordered.iter().position(|kt| kt.key == *k) {
                if i > 0 {
                    p.prev_key = ordered.get(i - 1).unwrap().key.clone()
                }
                if i < len - 1 {
                    p.next_key = ordered.get(i + 1).unwrap().key.clone();
                }
            } else {
                println!("Post {} is not chronological", k);
            }
        }
    }

    /// Sanitize camera informaton in all post photos
    pub fn sanitize_exif(&mut self, config: &ExifConfig) {
        for (_, p) in self.posts.iter_mut() {
            for photo in p.photos.iter_mut() {
                photo.sanitize(config);
            }
        }
    }

    /// Collect unique photo tags as keys to the list of photos that applied
    /// those tags
    pub fn collate_tags(&mut self) {
        let mut tags: HashMap<String, TagPhotos> = HashMap::new();

        for (_, p) in self.posts.iter() {
            for photo in p.photos.iter() {
                for tag in photo.tags.iter() {
                    let tag_slug = slugify(tag);
                    let photo_path = PhotoPath {
                        post_key: p.key.clone(),
                        photo_name: photo.name.clone(),
                    };
                    match tags.get_mut(&tag_slug) {
                        Some(tag_photos) => {
                            // add new photo path to existing tag
                            tag_photos.photos.push(photo_path);
                        }
                        _ => {
                            // create new tag with photo path
                            tags.insert(
                                tag_slug,
                                TagPhotos {
                                    name: tag.clone(),
                                    photos: vec![photo_path],
                                },
                            );
                        }
                    }
                }
            }
        }

        self.tags = tags;
    }
}
