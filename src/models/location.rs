use serde::Deserialize;

/// Latitude and longitude in degrees
#[derive(Debug, Default, Deserialize, Clone)]
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
