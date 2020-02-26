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
        let mut iter = self.posts.iter_mut();

        for i in 0..len {
            //let mut p = &self.posts[i];
            // if i > 0 {
            //     p.prev = Some(&self.posts[i - 1]);
            // }
            let p1: &mut Post = iter.nth(i).unwrap();
            let p2: &mut Post = iter.nth(i + 1).unwrap();

            p1.next = p2;
        }

        //self.posts[0].prev = Some(&self.posts[1]);

        // https://stackoverflow.com/questions/40875152/reference-to-element-in-vector
        // for (i, p) in self.posts.iter_mut().enumerate() {
        //     if i > 0 {
        //         let prev = &self.posts[i - 1];
        //         p.prev = Some(prev);
        //     }
        // }
    }
}

// https://www.reddit.com/r/rust/comments/7dep46/multiple_references_to_a_vectors_elements/
// fn get_two_mut<T>(slice: &mut Vec<T>, i: usize) -> (&mut T, &mut T) {
//     let mut iter = slice.iter_mut();
//     (iter.nth(i).unwrap(), iter.nth(i + 1).unwrap())
// }
