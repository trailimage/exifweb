use crate::post::Post;

pub struct Blog<'a, 'b> {
    pub posts: Vec<&'a Post<'a, 'b>>,
}
