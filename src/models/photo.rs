use super::{Camera, Location};
use crate::config::ExifConfig;
use crate::tools::replace_pairs;
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Unique path to any blog photo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoPath {
    pub post_path: String,
    pub photo_index: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Size {
    width: u16,
    height: u16,
}

#[derive(Debug)]
pub struct Photo {
    /// File name of the photo without extension
    pub name: String,
    /// Name of photographer recorded in EXIF
    pub artist: Option<String>,
    /// Name of software used to process the photo
    pub software: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    /// Information about the camera used to make the photo
    pub camera: Option<Camera>,
    /// Latitude and longitude where photo was taken
    pub location: Option<Location>,
    /// One-based position of photo within post
    pub index: u8,
    /// Tags applied to the photo
    pub tags: Vec<String>,
    /// When the photograph was taken per camera EXIF
    pub date_taken: Option<DateTime<FixedOffset>>,

    /// Whether taken date is an outlier (such an historic photo) compared to
    /// other photos in the same post. Outliers may be removed from mini-maps so
    /// the maps aren't overly zoomed-out.
    ///
    /// http://www.wikihow.com/Calculate-Outliers
    pub outlier_date: bool,

    /// Whether values have been formatted based on configuration
    pub sanitized: bool,

    pub width: u16,
    pub height: u16,
}

impl Photo {
    /// Standardize EXIF data based on configuration and remove invalid values
    pub fn sanitize(&mut self, config: &ExifConfig) {
        if self.sanitized {
            return;
        }
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

        self.sanitized = true;
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
        self.name == other.name && self.date_taken == other.date_taken
    }
}

impl Eq for Photo {}

impl Default for Photo {
    fn default() -> Self {
        Photo {
            name: String::new(),
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
            sanitized: false,
            width: 0,
            height: 0,
        }
    }
}
