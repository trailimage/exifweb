use crate::config::ExifConfig;
use crate::num_traits::FromPrimitive;
use crate::tools::{boundary, min_date, replace_pairs};
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::marker::Copy;

/// Latitude and longitude in degrees
#[derive(Debug, Default)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}

impl Location {
    /// Whether latitude and longitude are within valid range
    pub fn is_valid(&self) -> bool {
        self.longitude <= 180.0
            && self.longitude >= -180.0
            && self.latitude <= 90.0
            && self.latitude >= -90.0
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.longitude == other.longitude && self.latitude == other.latitude
    }
}

impl Eq for Location {}

#[derive(Debug, Primitive, Copy)]
#[repr(u8)]
pub enum ExposureMode {
    Undefined = 0,
    Manual = 1,
    ProgramAE = 2,
    AperturePriority = 3,
    ShutterPriority = 4,
    Creative = 5,
    Action = 6,
    Portrait = 7,
    Landscape = 8,
    Bulb = 9,
}

impl Clone for ExposureMode {
    fn clone(&self) -> ExposureMode {
        *self
    }
}

impl Default for ExposureMode {
    fn default() -> Self {
        ExposureMode::Undefined
    }
}

impl<'de> Deserialize<'de> for ExposureMode {
    fn deserialize<D>(deserializer: D) -> Result<ExposureMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(ExposureModeVisitor)
    }
}

struct ExposureModeVisitor;

impl<'de> de::Visitor<'de> for ExposureModeVisitor {
    type Value = ExposureMode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 9")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ExposureMode::from_u64(value).unwrap())
    }
}

/// Information about the camera used to make the photo.
#[derive(Debug, Default)]
pub struct Camera {
    /// Make and model of the camera
    pub name: String,
    /// Exposure compensation expressed as a string to permit arbitrary
    /// fractions
    pub compensation: Option<String>,
    /// Shutter speed expressed as a string to permit arbitrary fractions
    pub shutter_speed: Option<String>,
    pub mode: ExposureMode,
    pub aperture: Option<f32>,
    pub focal_length: Option<u16>,
    pub iso: Option<u16>,
    /// Description of the lens used
    pub lens: Option<String>,
}

#[derive(Debug)]
pub struct Photo {
    /// File name of the photo without extension
    pub name: String,
    /// Name of photographer recorded in EXIF
    pub artist: String,
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
    /// Whether this is the post's main photo
    pub primary: bool,
    /// When the photograph was taken per camera EXIF
    pub date_taken: Option<DateTime<FixedOffset>>,

    /// Whether taken date is an outlier compared to other photos in the same
    /// post. Outliers may be removed from mini-maps so the maps aren't overly
    /// zoomed-out to accomodate contextual photos taken days before or after
    /// the main post.
    ///
    /// See http://www.wikihow.com/Calculate-Outliers
    pub outlier_date: bool,

    /// Whether values have been formatted based on configuration
    pub sanitized: bool,
}

impl Photo {
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

        self.sanitized = true;
    }
}

impl PartialOrd for Photo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Photo {
    fn cmp(&self, other: &Photo) -> Ordering {
        self.name.cmp(&other.name)
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
            artist: String::new(),
            software: String::new(),
            title: None,
            caption: None,
            camera: None,
            location: None,
            index: 0,
            tags: Vec::new(),
            primary: false,
            date_taken: None,
            outlier_date: false,
            sanitized: false,
        }
    }
}

/// Simplistic outlier calculation identifies photos that are likely not part of
/// the main sequence.
///
/// - https://en.wikipedia.org/wiki/Outlier
/// - http://www.wikihow.com/Calculate-Outliers
fn identify_outliers(photos: Vec<Photo>) {
    let mut times: Vec<i64> = photos
        .iter()
        .filter(|p| p.date_taken.is_some())
        .map(|p| p.date_taken.unwrap().timestamp())
        .collect();

    if let Some(fence) = boundary(&mut times[..], 3) {
        for mut p in photos {
            if p.date_taken.is_none() {
                continue;
            }
            let d = p.date_taken.unwrap().timestamp() as f64;
            if d > fence.max || d < fence.min {
                p.outlier_date = true;
            }
        }
    }
}
