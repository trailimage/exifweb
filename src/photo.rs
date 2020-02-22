use crate::EXIF;

pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}
pub struct Photo {
    pub name: String,
    pub exif: EXIF,
    pub location: Location,
    /// Position of photo within post.
    pub index: i8,
    /// Tags applied to the photo.
    pub tags: Vec<String>,
    /// Whether this is the post's main photo.
    pub primary: bool,

    pub date_taken: i32,

    /// Whether taken date is an outlier compared to other photos in the same
    /// post. Outliers may be removed from mini-maps so the maps aren't overly
    /// zoomed-out to accomodate contextual photos taken days before or after
    /// the main post.
    ///
    /// See http://www.wikihow.com/Calculate-Outliers
    pub outlier_date: bool,
}
