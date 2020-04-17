use super::load_config;
use crate::models::{Category, CategoryKind};
use serde::Deserialize;
use std::path::Path;

/// Categories to which the post has been assigned
#[derive(Deserialize, Debug)]
pub struct PostCategories {
    pub who: String,
    /// year
    pub when: u16,
    pub r#where: Vec<String>,
    pub what: String,
}
/// Configuration within each post folder
#[derive(Deserialize, Debug)]
pub struct PostConfig {
    pub title: String,
    pub summary: String,
    /// Categories to which the post has been assigned
    #[serde(rename = "categories")]
    pub category_list: PostCategories,
    /// One-based index of cover photo
    pub cover_photo_index: u8,
    /// YouTube ID used to embed video
    pub youtube_id: Option<String>,
}

impl PostConfig {
    /// Categories from configured category names
    pub fn categories(&self) -> Vec<Category> {
        let mut categories: Vec<Category> = vec![
            Category::new(
                &self.category_list.when.to_string(),
                CategoryKind::When,
            ),
            Category::new(&self.category_list.what, CategoryKind::What),
            Category::new(&self.category_list.who, CategoryKind::Who),
        ];

        for w in self.category_list.r#where.iter() {
            categories.push(Category::new(w, CategoryKind::Where))
        }
        categories
    }

    pub fn load(path: &Path) -> Option<Self> {
        load_config::<Self>(path)
    }
}

// Convert configured categories to a vector
// fn parse_categories(config: &PostConfig) -> Vec<Category> {
//     let mut categories: Vec<Category> = vec![
//         Category {
//             name: config.categories.when.clone(),
//             kind: CategoryKind::When,
//         },
//         Category {
//             name: config.categories.what.clone(),
//             kind: CategoryKind::What,
//         },
//         Category {
//             name: config.categories.who.clone(),
//             kind: CategoryKind::Who,
//         },
//     ];

//     for w in config.categories.r#where.iter() {
//         categories.push(Category {
//             name: w.clone(),
//             kind: CategoryKind::Where,
//         });
//     }
//     categories
// }
