use crate::photo::Photo;
use std::path::PathBuf;

pub struct Post<'a> {
    pub path: PathBuf,
    pub title: String,
    pub summary: String,
    pub photos: Vec<&'a Photo>,
    pub next: Option<&'a Post<'a>>,
    pub prev: Option<&'a Post<'a>>,
}
