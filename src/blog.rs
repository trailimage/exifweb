use crate::category::Category;
use crate::post::Post;

pub struct Blog<'a> {
    pub posts: Vec<Post<'a>>,
    pub categories: Vec<Category<'a>>,
}
