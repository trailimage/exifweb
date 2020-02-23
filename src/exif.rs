use crate::Pairs;

#[derive(Debug)]
pub struct EXIF {
    pub artist: String,
    pub compensation: String,
    pub time: String,
    pub f_number: u8,
    pub focal_length: u16,
    pub iso: u16,
    pub lens: String,
    pub software: String,
    /// Whether raw values have been formatted.
    pub sanitized: bool,
}

fn replace_pairs(text: String, pairs: Pairs) -> String {
    let mut clean = text;
    for (x, y) in pairs {
        if clean.starts_with(&x) {
            clean = clean.replace(&x, &y);
        }
    }

    return clean;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_lens() {
//         assert_eq!(lens("FE 35mm whatever"), "bSony FE 35mm ƒ2.8")
//     }
// }
