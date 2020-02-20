use crate::Post;

struct Blog<'a> {
    posts: Vec<&'a Post<'a>>,
}
