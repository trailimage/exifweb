use crate::Category;
use crate::Post;

#[derive(Default)]
pub struct Blog<'a> {
    pub posts: Vec<Post<'a>>,
    pub categories: Vec<Category<'a>>,
}
