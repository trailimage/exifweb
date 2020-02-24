use crate::Category;
use crate::Post;

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: Vec<Post<'a>>,
    pub categories: Vec<Category<'a>>,
}

impl Blog<'_> {
    pub fn correlate_posts(mut self) {
        self.posts.sort();
        let len = self.posts.len();

        // for i in 0..len {
        //     let mut p = &self.posts[i];
        //     if i > 0 {
        //         p.prev = Some(&self.posts[i - 1]);
        //     }
        // }

        for (i, p) in self.posts.iter_mut().enumerate() {
            if i > 0 {
                let prev = &mut self.posts[i - 1];
                p.prev = Some(prev);
            }
        }
    }
}
