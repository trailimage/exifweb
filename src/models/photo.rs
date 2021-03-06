use super::{Camera, Location, SizeCollection};
use crate::{config::ExifConfig, models::size, tools::replace_pairs};
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Unique path to any blog photo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoPath {
    pub post_path: String,
    pub photo_index: u8,
}

impl PhotoPath {
    pub fn post_url(&self) -> String {
        format!("{}#{:03}", self.post_path, self.photo_index)
    }

    pub fn thumb_url(&self, ext: &str) -> String {
        format!(
            "{}/{:03}_{}{}",
            self.post_path,
            self.photo_index,
            size::suffix::THUMB,
            ext
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct PhotoFile {
    /// File name of source image including extension
    pub name: String,
    /// Timestamp when the file was created
    pub created: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    /// Name of photographer recorded in EXIF
    #[serde(skip)]
    pub artist: Option<String>,

    /// Name of software used to process the photo
    #[serde(skip)]
    pub software: String,

    #[serde(skip)]
    pub title: Option<String>,

    #[serde(skip)]
    pub caption: Option<String>,

    /// Information about the camera used to make the photo
    #[serde(skip)]
    pub camera: Option<Camera>,

    /// Latitude and longitude where photo was taken
    #[serde(skip)]
    pub location: Option<Location>,

    /// One-based position of photo within post
    pub index: u8,

    // TODO: remove hash tags like #boisephotographer
    /// Tags applied to the photo
    #[serde(skip)]
    pub tags: Vec<String>,

    /// When the photograph was taken per camera EXIF
    #[serde(skip)]
    pub date_taken: Option<DateTime<FixedOffset>>,

    /// Whether taken date is an outlier (such an historic photo) compared to
    /// other photos in the same post. Outliers may be removed from mini-maps so
    /// the maps aren't overly zoomed-out.
    ///
    /// http://www.wikihow.com/Calculate-Outliers
    #[serde(skip)]
    pub outlier_date: bool,

    /// Sizes in which the photo is available
    pub size: SizeCollection,

    #[serde(skip)]
    pub file: PhotoFile,
}

impl Photo {
    /// Standardize EXIF data based on configuration and remove invalid values
    pub fn sanitize(&mut self, config: &ExifConfig) {
        self.software = replace_pairs(self.software.clone(), &config.software);

        if let Some(camera) = &mut self.camera {
            camera.name = replace_pairs(camera.name.clone(), &config.camera);

            if let Some(lens) = &camera.lens {
                camera.lens = Some(replace_pairs(lens.clone(), &config.lens));
            }
        }

        if let Some(l) = &self.location {
            if !l.is_valid() {
                // remove invalid location
                self.location = None;
            }
        }
    }

    pub fn json_ld(&self) -> serde_json::Value {
        let size = &self.size.medium;

        // TODO: needs full image path?
        serde_json::json!({
            "@type": "ImageObject",
            "url": size.name,
            "width": size.width,
            "height": size.height
        })
    }

    /// Whether photo is in portrait orientation (taller than wide)
    pub fn is_portrait(&self) -> bool {
        self.size.is_portrait()
    }

    /// Image width divided by height
    pub fn aspect_ratio(&self) -> f32 {
        self.size.aspect_ratio()
    }
}

impl PartialOrd for Photo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for Photo {
    fn cmp(&self, other: &Photo) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialEq for Photo {
    fn eq(&self, other: &Self) -> bool {
        self.file.name == other.file.name && self.date_taken == other.date_taken
    }
}

impl Eq for Photo {}

impl Default for Photo {
    fn default() -> Self {
        Photo {
            file: PhotoFile {
                name: String::new(),
                created: 0,
            },
            artist: None,
            software: String::new(),
            title: None,
            caption: None,
            camera: None,
            location: None,
            index: 0,
            tags: Vec::new(),
            date_taken: None,
            outlier_date: false,
            size: SizeCollection::default(),
        }
    }
}
