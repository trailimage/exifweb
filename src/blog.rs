use crate::Category;
use crate::Post;

pub struct Blog<'a> {
    pub posts: Vec<Post<'a>>,
    pub categories: Vec<Category<'a>>,
}
