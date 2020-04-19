use crate::config::{ExifConfig, PostLog};
use crate::models::{
    Category, CategoryKind, Photo, PhotoPath, Post, TagPhotos,
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;

/// Ephemeral struct to compute and capture chronological post order
struct KeyTime {
    /// Post path
    path: String,
    /// Post `happened_on` date
    time: DateTime<FixedOffset>,
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
    pub tags: HashMap<String, TagPhotos<PhotoPath>>,
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

    pub fn add_post_photos(&mut self, path: &String, photos: &mut Vec<Photo>) {
        if photos.is_empty() {
            return;
        }
        if let Some(post) = self.posts.get_mut(path) {
            post.photos.append(photos);
        }
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

    /// Number of posts that need to be rendered
    pub fn needs_render_count(&self) -> usize {
        let mut total: usize = 0;
        for (_, p) in &self.posts {
            if p.needs_render {
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
    /// and return paths to all posts that
    ///
    /// - had different values the last time they were loaded
    /// - and are not already flagged to be rendered
    ///
    pub fn correlate_posts(&mut self) -> Vec<String> {
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

        self.sequence_changed_posts()
    }

    /// Return paths to all posts that have a different `prev_path` or
    /// `next_path` than the last time they were loaded
    fn sequence_changed_posts(&mut self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        for (path, p) in self.posts.iter_mut() {
            if p.needs_render || p.history.is_none() {
                continue;
            }
            let log: &PostLog = Option::as_ref(&p.history).unwrap();

            if log.prev_path != p.prev_path || log.next_path != p.next_path {
                p.needs_render = true;
                paths.push(path.clone());
            }
        }
        paths
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
        let mut tags: HashMap<String, TagPhotos<PhotoPath>> = HashMap::new();

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
                            },
                        );
                    }
                }
            }
        }

        self.tags = tags;
    }
}
