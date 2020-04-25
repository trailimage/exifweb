use super::load_config;
use crate::models::{Category, CategoryKind};
use serde::Deserialize;
use std::path::Path;

/// Categories to which the post has been assigned
#[derive(Deserialize, Debug)]
pub struct PostCategories {
    #[serde(default)]
    pub who: Option<String>,
    /// year
    #[serde(default)]
    pub when: Option<u16>,
    #[serde(default)]
    pub r#where: Vec<String>,
    #[serde(default)]
    pub what: Option<String>,
}
/// Configuration within each post folder
#[derive(Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,

    /// Categories to which the post has been assigned
    #[serde(default, rename = "categories")]
    pub category_list: Option<PostCategories>,

    /// One-based index of cover photo
    #[serde(default)]
    pub cover_photo_index: usize,

    // TODO: enable YouTube links
    /// YouTube ID used to embed video
    pub youtube_id: Option<String>,

    #[serde(default = "chronological_default")]
    pub chronological: bool,

    /// One-based series part or 0 if not in a series
    #[serde(default)]
    pub part: u8,
}

fn chronological_default() -> bool {
    true
}

impl PostConfig {
    /// Categories derived from configuration
    pub fn categories(&self) -> Vec<Category> {
        let mut categories: Vec<Category> = Vec::new();

        if let Some(list) = &self.category_list {
            if let Some(who) = &list.who {
                categories
                    .push(Category::new(&who.to_string(), CategoryKind::Who))
            }

            if let Some(when) = &list.when {
                categories
                    .push(Category::new(&when.to_string(), CategoryKind::When))
            }

            if let Some(what) = &list.what {
                categories
                    .push(Category::new(&what.to_string(), CategoryKind::What))
            }

            for w in list.r#where.iter() {
                categories.push(Category::new(w, CategoryKind::Where))
            }
        }

        categories
    }

    /// Load standard post configuration file from path
    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path)
    }
}
