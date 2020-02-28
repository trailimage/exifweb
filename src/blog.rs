use crate::{slugify, Category, Post};
use hashbrown::HashMap;
use time::Date;

struct KeyTime<'b> {
    key: &'b str,
    time: Date,
}

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: HashMap<String, Post<'a>>,
    pub categories: Vec<Category<'a>>,
}

impl<'a> Blog<'a> {
    pub fn add_post(&mut self, p: Post<'a>) {
        let key = slugify(&p.title);
        // TODO: error if post is already present
        self.posts.insert(key, p);
    }

    pub fn correlate_posts(&mut self) {
        let mut ordered: Vec<KeyTime> = Vec::new();

        for kt in
            self.posts
                .values()
                .filter(|p| p.chronological)
                .map(|p| KeyTime {
                    key: &p.key,
                    time: p.happened_on,
                })
        {
            ordered.push(kt);
        }

        ordered.sort_by(|a, b| a.time.cmp(&b.time));

        for (k, p) in self.posts.iter_mut() {
            let i = ordered.iter().position(|kt| kt.key == k);
            p.next_key = ordered.get(1).unwrap().key;
        }
    }
}

// impl Blog<'_> {
//     pub fn correlate_posts(mut self) {
//         self.posts.sort();
//         let len = self.posts.len();
//         let mut iter = self.posts.iter_mut();

//         for i in 0..len {
//             //let mut p = &self.posts[i];
//             // if i > 0 {
//             //     p.prev = Some(&self.posts[i - 1]);
//             // }
//             let p1: &mut Post = iter.nth(i).unwrap();
//             let p2 = self.posts.get(i + 1).unwrap();

//             p1.next = Some(p2);
//         }

//self.posts[0].prev = Some(&self.posts[1]);

// https://stackoverflow.com/questions/40875152/reference-to-element-in-vector
// for (i, p) in self.posts.iter_mut().enumerate() {
//     if i > 0 {
//         let prev = &self.posts[i - 1];
//         p.prev = Some(prev);
//     }
// }
//     }
// }

// https://www.reddit.com/r/rust/comments/7dep46/multiple_references_to_a_vectors_elements/
// fn get_two_mut<T>(slice: &mut Vec<T>, i: usize) -> (&mut T, &mut T) {
//     let mut iter = slice.iter_mut();
//     (iter.nth(i).unwrap(), iter.nth(i + 1).unwrap())
// }
