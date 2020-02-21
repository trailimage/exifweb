use crate::config::Pairs;

pub struct EXIF {
    artist: String,
    compensation: String,
    time: String,
    f_number: u8,
    focal_length: u8,
    iso: u8,
    lens: String,
    software: String,
    sanitized: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lens() {
        assert_eq!(lens("FE 35mm whatever"), "bSony FE 35mm ƒ2.8")
    }
}
