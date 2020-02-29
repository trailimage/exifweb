use crate::ExifConfig;
use crate::Pairs;

#[derive(Debug)]
pub struct EXIF {
    pub artist: String,
    pub compensation: String,
    pub time: String,
    pub f_number: u8,
    pub focal_length: u16,
    pub iso: u16,
    pub camera: String,
    pub lens: String,
    pub software: String,
    /// Whether raw values have been formatted.
    pub sanitized: bool,
}

impl EXIF {
    pub fn sanitize(&mut self, config: ExifConfig) {
        if self.sanitized {
            return;
        }
        self.software = replace_pairs(self.software.clone(), config.software);
        self.camera = replace_pairs(self.camera.clone(), config.camera);
        self.lens = replace_pairs(self.lens.clone(), config.lens);

        self.sanitized = true;
    }
}

fn replace_pairs(text: String, pairs: Pairs) -> String {
    let mut clean = text;
    for (x, y) in pairs {
        if clean.starts_with(&x) {
            clean = clean.replace(&x, &y);
        }
    }
    clean
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_lens() {
//         assert_eq!(lens("FE 35mm whatever"), "bSony FE 35mm ƒ2.8")
//     }
// }
