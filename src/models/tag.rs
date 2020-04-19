use crate::models::Photo;
use crate::tools::slugify;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Photos to which a tag has been applied
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagPhotos<T> {
    /// Original tag name (not slug)
    pub name: String,
    /// Photos that have the tag applied
    pub photos: Vec<T>,
}

/// Collect unique photo tag slugs as keys to the list of photos that applied
/// those tags. These data are used to render tag search and results pages.
pub fn collate_tags(photos: &Vec<Photo>) -> HashMap<String, TagPhotos<u8>> {
    let mut tags: HashMap<String, TagPhotos<u8>> = HashMap::new();

    for photo in photos.iter() {
        for tag in photo.tags.iter() {
            let tag_slug = slugify(tag);
            match tags.get_mut(&tag_slug) {
                Some(tag_photos) => {
                    // add new photo path to existing tag
                    tag_photos.photos.push(photo.index);
                }
                _ => {
                    // create new tag with photo path
                    tags.insert(
                        tag_slug,
                        TagPhotos {
                            name: tag.clone(),
                            photos: vec![photo.index],
                        },
                    );
                }
            }
        }
    }

    tags
}
