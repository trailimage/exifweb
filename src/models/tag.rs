use crate::models::{Photo, PhotoPath};
use crate::tools::slugify;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Photos to which a tag has been applied
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagPhotos {
    /// Original tag name (not slug)
    pub name: String,
    /// Photos that have the tag applied
    pub photos: Vec<PhotoPath>,
}

/// Collect unique photo tag slugs as keys to the list of photos that applied
/// those tags. These data are used to render tag search and results pages.
pub fn collate_tags(
    path: &str,
    photos: &Vec<Photo>,
) -> HashMap<String, TagPhotos> {
    let mut tags: HashMap<String, TagPhotos> = HashMap::new();

    for photo in photos.iter() {
        for tag in photo.tags.iter() {
            let tag_slug = slugify(tag);
            let photo_path = PhotoPath {
                post_path: path.to_owned(),
                photo_name: photo.name.clone(),
            };
            match tags.get_mut(&tag_slug) {
                Some(tag_photos) => {
                    // add new photo path to existing tag
                    tag_photos.photos.push(photo_path);
                }
                _ => {
                    // create new tag with photo path
                    tags.insert(
                        tag_slug,
                        TagPhotos {
                            name: tag.clone(),
                            photos: vec![photo_path],
                        },
                    );
                }
            }
        }
    }

    tags
}
