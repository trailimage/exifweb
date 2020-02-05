use crate::Photo;

struct Post {
    title: str,
    photos: Vec<&Photo>,
    next: &Post,
    prev: &Post,
}
