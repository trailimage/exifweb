use crate::{min_date, ExifConfig, Pairs};
use chrono::{DateTime, Local};

/// Latitude and longitude in degrees.
#[derive(Debug, Default)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

impl Location {
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

// https://exiftool.org/TagNames/EXIF.html
#[derive(Debug, Default)]
pub struct EXIF {
    pub artist: String,
    pub camera: String,
    pub compensation: String,
    pub shutter_speed: String,
    pub mode: ExposureMode,
    pub aperture: String,
    pub focal_length: f64,
    pub iso: u32,
    pub lens: String,
    pub software: String,
    /// Whether raw values have been formatted.
    pub sanitized: bool,
}

impl EXIF {
    pub fn sanitize(&mut self, config: &ExifConfig) {
        if self.sanitized {
            return;
        }
        self.software = replace_pairs(self.software.clone(), &config.software);
        self.camera = replace_pairs(self.camera.clone(), &config.camera);
        self.lens = replace_pairs(self.lens.clone(), &config.lens);

        self.sanitized = true;
    }
}

fn replace_pairs(text: String, pairs: &Pairs) -> String {
    let mut clean = text;
    for (x, y) in pairs {
        if clean.starts_with(x) {
            clean = clean.replace(x, y);
        }
    }
    clean
}

#[derive(Debug)]
pub struct Photo {
    /// File name of the photo
    pub name: String,
    pub title: String,
    pub caption: String,
    pub exif: EXIF,
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
}

impl Default for Photo {
    fn default() -> Self {
        Photo {
            name: String::new(),
            title: String::new(),
            caption: String::new(),
            exif: EXIF::default(),
            location: Location::default(),
            index: 0,
            tags: Vec::new(),
            primary: false,
            date_taken: min_date(),
            outlier_date: false,
        }
    }
}
