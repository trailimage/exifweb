use crate::Photo;
use std::path::Path;

struct Post<'a> {
    path: &'a Path,
    title: String,
    summary: String,
    photos: Vec<&'a Photo>,
    next: &'a Post<'a>,
    prev: &'a Post<'a>,
}
