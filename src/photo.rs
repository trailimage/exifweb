use crate::EXIF;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.longitude == other.longitude && self.latitude == other.latitude
    }
}

impl Eq for Location {}

#[derive(Debug)]
pub struct Photo {
    /// File name of the photo
    pub name: String,
    pub exif: EXIF,
    pub location: Location,
    /// One-based position of photo within post
    pub index: u8,
    /// Tags applied to the photo
    pub tags: Vec<String>,
    /// Whether this is the post's main photo
    pub primary: bool,
    /// When the photograph was taken per camera EXIF
    pub date_taken: SystemTime,

    /// Whether taken date is an outlier compared to other photos in the same
    /// post. Outliers may be removed from mini-maps so the maps aren't overly
    /// zoomed-out to accomodate contextual photos taken days before or after
    /// the main post.
    ///
    /// See http://www.wikihow.com/Calculate-Outliers
    pub outlier_date: bool,
}
