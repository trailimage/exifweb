use crate::Category;
use crate::Post;

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: Vec<Box<Post<'a>>>,
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

        // https://stackoverflow.com/questions/40875152/reference-to-element-in-vector
        for (i, p) in self.posts.iter_mut().enumerate() {
            if i > 0 {
                //let prev = &self.posts[i - 1];
                p.prev = Some(&self.posts[i - 1]);
            }
        }
    }
}
