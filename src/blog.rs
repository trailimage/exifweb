use crate::{slugify, Category, Post};
use hashbrown::HashMap;
use std::time::SystemTime;

struct KeyTime {
    key: String,
    time: SystemTime,
}

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: HashMap<String, Post<'a>>,
    pub categories: Vec<Category<'a>>,
}

impl<'a> Blog<'a> {
    pub fn add_post(&mut self, p: Post<'a>) {
        let key = slugify(&p.title);

        if let Some(_p) = self.posts.insert(key, p) {
            // if insert returns Post then same key was already present
            panic!("Attempt to insert duplicate post")
        }
    }

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

        ordered.sort_by(|a, b| a.time.cmp(&b.time));

        let len = ordered.len();

        for (k, p) in self.posts.iter_mut() {
            let i = ordered.iter().position(|kt| kt.key == *k).unwrap();

            if i > 0 {
                p.prev_key = ordered.get(i - 1).unwrap().key.clone()
            }
            if i < len {
                p.next_key = ordered.get(i + 1).unwrap().key.clone();
            }
        }
    }
}
