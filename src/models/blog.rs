use crate::config::{BlogConfig, BlogLog, ExifConfig};
use crate::models::{Category, CategoryKind, PhotoPath, Post, TagPhotos};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use std::collections::BTreeMap;

/// Ephemeral struct to compute and capture chronological post order
struct KeyTime {
    /// Post path
    path: String,
    /// Post `happened_on` date
    time: DateTime<FixedOffset>,
}
#[derive(Default)]
pub struct Blog {
    /// Posts keyed to their path
    pub posts: BTreeMap<String, Post>,
    /// Match list of post paths to a list of categories keyed to their kind
    pub categories: HashMap<CategoryKind, Vec<Category>>,
    /// Tag slugs matched to the original tag names and photos with the tag. Use
    /// B-Tree so that keys are sorted.
    pub tags: BTreeMap<String, TagPhotos<PhotoPath>>,
    /// Record of previously loaded photo tags
    pub history: BlogLog,
}

impl Blog {
    pub fn add_post(&mut self, p: Post) {
        if self.posts.contains_key(&p.path) {
            panic!("Attempt to insert duplicate post {}", p.path)
        }
        self.add_post_categories(&p);
        self.posts.insert(p.path.clone(), p);
    }

    /// Calculate map image sizes based on cover image dimensions
    pub fn prepare_maps(&mut self, config: &BlogConfig) {
        for (_, p) in self.posts.iter_mut() {
            p.prepare_maps(config);
        }
    }

    /// Add post categories to the global category list
    fn add_post_categories(&mut self, p: &Post) {
        for post_category in &p.categories {
            let post_path = p.path.clone();

            match self.categories.get_mut(&post_category.kind) {
                Some(existing_kind) => {
                    // post category kind already exists
                    match existing_kind.iter_mut().find(
                        |c: &'_ &mut Category| c.name == post_category.name,
                    ) {
                        Some(existing) => {
                            // post category already exists
                            existing.post_paths.push(post_path);
                            existing.post_paths.sort();
                        }
                        None => {
                            // category kind exists but not the category itself
                            let mut copy = post_category.clone();
                            copy.post_paths.push(post_path);
                            existing_kind.push(copy);
                            existing_kind.sort();
                        }
                    }
                }
                None => {
                    // post contains a new category kind
                    let mut copy = post_category.clone();
                    // category in post had no post_paths since it was implicit
                    copy.post_paths.push(post_path);
                    self.categories.insert(post_category.kind, vec![copy]);
                }
            };
        }
    }

    /// Number of posts that need to be rendered
    pub fn needs_render_count(&self) -> usize {
        let mut total: usize = 0;
        for p in self.posts.values() {
            if p.files_changed() || p.sequence_changed() {
                total += 1;
            }
        }
        total
    }

    /// Retrieve post with path
    pub fn get(&self, path: &str) -> Option<&Post> {
        if path.is_empty() {
            None
        } else {
            self.posts.get(path)
        }
    }

    pub fn get_featured(&mut self, path: &str) -> Option<&Post> {
        if let Some(post) = self.posts.get_mut(path) {
            post.featured = true;
            Some(post)
        } else {
            None
        }
    }

    pub fn get_next(&self, post: &Post) -> Option<&Post> {
        post.next_path.as_ref().and_then(|p| self.get(&p))
    }

    pub fn get_prev(&self, post: &Post) -> Option<&Post> {
        post.prev_path.as_ref().and_then(|p| self.get(&p))
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
                    p.prev_path =
                        ordered.get(i - 1).map(|kt: &KeyTime| kt.path.clone());
                }
                if i < len - 1 {
                    p.next_path =
                        ordered.get(i + 1).map(|kt: &KeyTime| kt.path.clone());
                }
            } else {
                eprintln!("   Post {} is not chronological", k);
            }
        }
    }

    /// Sanitize camera informaton in all post photos. This will only affect
    /// posts that need to be rendered since unchanged posts will have an empty
    /// photo list.
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
        let mut tags: BTreeMap<String, TagPhotos<PhotoPath>> = BTreeMap::new();

        for (_, p) in self.posts.iter() {
            for (slug, post_tag) in p.tags.iter() {
                let mut photo_paths: Vec<PhotoPath> = post_tag
                    .photos
                    .iter()
                    .map(|i| PhotoPath {
                        post_path: p.path.clone(),
                        photo_index: *i,
                    })
                    .collect();

                match tags.get_mut(slug) {
                    Some(tag_photos) => {
                        // add post photo paths to existing tag
                        tag_photos.photos.append(photo_paths.as_mut());
                    }
                    _ => {
                        // create new tag with photo path
                        tags.insert(
                            slug.clone(),
                            TagPhotos {
                                name: post_tag.name.clone(),
                                photos: photo_paths,
                                changed: true,
                            },
                        );
                    }
                }
            }
        }

        if !self.history.tags.is_empty() {
            // if previous tags were logged then compare to new tags to identify
            // which haven't changed and so don't need to be rendered again
            for (slug, mut tag_photos) in &mut tags {
                if let Some(log_tag_photos) = self.history.tags.get(slug) {
                    if tag_photos.name == log_tag_photos.name
                        && tag_photos.photos.len()
                            == log_tag_photos.photos.len()
                    {
                        // naive check only considers number of matched photos
                        tag_photos.changed = false;
                    }
                }
            }
        }

        self.tags = tags;
    }
}
