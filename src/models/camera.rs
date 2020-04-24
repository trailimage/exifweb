use super::ExposureMode;

/// Information about the camera used to make the photo.
#[derive(Debug, Default, Clone)]
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
