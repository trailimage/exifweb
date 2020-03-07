use crate::{min_date, replace_pairs, ExifConfig};
use chrono::{DateTime, Local};

/// Latitude and longitude in degrees
#[derive(Debug, Default)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
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

#[derive(Debug)]
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
impl Default for ExposureMode {
    fn default() -> Self {
        ExposureMode::Undefined
    }
}

/// Information about the camera used to make the photo.
// https://exiftool.org/TagNames/EXIF.html
#[derive(Debug, Default)]
pub struct Camera {
    /// Make and model of the camera
    pub name: String,
    /// Exposure compensation
    pub compensation: String,
    pub shutter_speed: String,
    pub mode: ExposureMode,
    pub aperture: String,
    pub focal_length: f64,
    pub iso: u32,
    pub lens: String,
}

#[derive(Debug)]
pub struct Photo {
    /// File name of the photo
    pub name: String,
    /// Name of photographer recorded in EXIF
    pub artist: String,
    /// Name of software used to process the photo
    pub software: String,
    pub title: String,
    pub caption: String,
    pub camera: Camera,
    pub location: Location,
    /// One-based position of photo within post
    pub index: u8,
    /// Tags applied to the photo
    pub tags: Vec<String>,
    /// Whether this is the post's main photo
    pub primary: bool,
    /// When the photograph was taken per camera EXIF
    pub date_taken: DateTime<Local>,

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
        self.camera.name =
            replace_pairs(self.camera.name.clone(), &config.camera);
        self.camera.lens =
            replace_pairs(self.camera.lens.clone(), &config.lens);

        self.sanitized = true;
    }
}

impl Default for Photo {
    fn default() -> Self {
        Photo {
            name: String::new(),
            artist: String::new(),
            software: String::new(),
            title: String::new(),
            caption: String::new(),
            camera: Camera::default(),
            location: Location::default(),
            index: 0,
            tags: Vec::new(),
            primary: false,
            date_taken: min_date(),
            outlier_date: false,
            sanitized: false,
        }
    }
}
