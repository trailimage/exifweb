use crate::photo::Photo;
use std::path::Path;

pub struct Post<'a, 'b> {
    pub path: &'b Path,
    pub title: String,
    pub summary: String,
    pub photos: Vec<&'a Photo>,
    pub next: Option<&'a Post<'a, 'b>>,
    pub prev: Option<&'a Post<'a, 'b>>,
}
