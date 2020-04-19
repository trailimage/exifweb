use crate::config::ExifConfig;
use crate::num_traits::FromPrimitive;
use crate::tools::replace_pairs;
use chrono::{DateTime, FixedOffset};
use core::cmp::Ordering;
use fmt::Display;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, marker::Copy};

/// Latitude and longitude in degrees
#[derive(Debug, Default, Deserialize)]
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

#[derive(Debug, Primitive, Copy, PartialEq, Eq)]
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

impl Display for ExposureMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExposureMode::Undefined => f.write_str("Unknown"),
            ExposureMode::Manual => f.write_str("Manual"),
            ExposureMode::ProgramAE => f.write_str("Program AE"),
            ExposureMode::AperturePriority => f.write_str("Aperture Priority"),
            ExposureMode::ShutterPriority => f.write_str("Shutter Priority"),
            ExposureMode::Creative => f.write_str("Creative Mode"),
            ExposureMode::Action => f.write_str("Action Mode"),
            ExposureMode::Portrait => f.write_str("Portrait Mode"),
            ExposureMode::Landscape => f.write_str("Landscape Mode"),
            ExposureMode::Bulb => f.write_str("Bulb Flash"),
        }
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

/// Unique path to any blog photo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoPath {
    pub post_path: String,
    pub photo_index: u8,
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
    pub focal_length: Option<f32>,
    pub iso: Option<u16>,
    /// Description of the lens used
    pub lens: Option<String>,
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
