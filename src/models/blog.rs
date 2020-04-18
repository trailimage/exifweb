use crate::config::ExifConfig;
use crate::tools::slugify;
use crate::{Category, CategoryKind, Post};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;

/// Ephemeral struct to compute and capture chronological post order
struct KeyTime {
    /// Post path
    path: String,
    /// Post `happened_on` date
    time: DateTime<FixedOffset>,
}

/// Unique path to any blog photo
pub struct PhotoPath {
    pub post_path: String,
    /// Photo file name without extension
    pub photo_name: String,
}

pub struct TagPhotos {
    /// Original tag name (not slugified)
    pub name: String,
    /// Photos that have the tag applied
    pub photos: Vec<PhotoPath>,
}

pub struct CategoryPosts {
    pub name: String,
    pub post_paths: Vec<String>,
    pub kind: CategoryKind,
}

#[derive(Default)]
pub struct Blog {
    /// Posts keyed to their slug
    pub posts: HashMap<String, Post>,
    pub categories: Vec<CategoryPosts>,
    /// Tag slugs mapped to the original tag names and photos with the tag
    pub tags: HashMap<String, TagPhotos>,
}

impl Blog {
    pub fn add_post(&mut self, p: Post) {
        if self.posts.contains_key(&p.path) {
            panic!("Attempt to insert duplicate post {}", p.path)
        }
        for c in &p.categories {
            self.add_category_post(c, &p)
        }

        self.posts.insert(p.path.clone(), p);
    }

    /// Get matching category or create and return the missing category
    fn add_category_post(&mut self, c: &Category, p: &Post) {
        let path = p.path.clone();

        if let Some(category) = self
            .categories
            .iter_mut()
            .find(|cp| cp.name == c.name && cp.kind == c.kind)
        {
            category.post_paths.push(path)
        } else {
            self.categories.push(CategoryPosts {
                name: c.name.clone(),
                kind: c.kind,
                post_paths: vec![path],
            });
        }
    }

    /// Number of changed posts
    pub fn changed_count(&self) -> usize {
        let mut total: usize = 0;
        for (_, p) in &self.posts {
            if p.changed {
                total += 1;
            }
        }
        total
    }

    /// Post with path
    pub fn get(&self, path: &str) -> Option<&Post> {
        if path.is_empty() {
            None
        } else {
            self.posts.get(path)
        }
    }

    /// Whether blog has any posts
    pub fn is_empty(&self) -> bool {
        self.posts.is_empty()
    }

    pub fn post_count(&self) -> usize {
        self.posts.len()
    }

    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }

    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Update post `prev_key` and `next_key` based on chronological ordering
    pub fn correlate_posts(&mut self) {
        let mut ordered: Vec<KeyTime> = Vec::new();

        for kt in self
            .posts
            .values()
            .filter(|p: &'_ &Post| p.chronological && p.happened_on.is_some())
            .map(|p: &Post| KeyTime {
                path: p.path.clone(),
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
            if let Some(i) = ordered.iter().position(|kt| kt.path == *k) {
                if i > 0 {
                    p.prev_path = ordered.get(i - 1).unwrap().path.clone()
                }
                if i < len - 1 {
                    p.next_path = ordered.get(i + 1).unwrap().path.clone();
                }
            } else {
                eprintln!("Post {} is not chronological", k);
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
                        post_path: p.path.clone(),
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
