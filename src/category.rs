use crate::post::Post;

pub struct Category<'a> {
    pub name: String,
    pub posts: Vec<&'a Post<'a>>,
}
