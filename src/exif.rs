use toml::Value;
use regex::Regex;
use hashbrown::HashMap;

struct EXIF {
    artist: str,
    compensation: str,
    time: str,
    f_number: u8,
    focal_length: u8,
    iso: u8,
    lens: str,
    software: str,
    sanitized: bool,
}

/// Normalize lens name.
/// https://docs.rs/regex/1.3.4/regex/
fn lens(text: String) -> String {
    lazy_static! {
        static ref re_35 = Regex::new(r"FE 35mm.*").unwrap();
        static ref re_55 = Regex::new(r"FE 55mm.*").unwrap();
        static ref re_90 = Regex::new(r"FE 90mm.*").unwrap();
    }
    let after = re_35.replace_all(text, "Sony FE 35mm ƒ2.8");

    return after;
}

/// Normalize software name.
fn software(text: String) -> String {}

/// Normalize compensation value.
fn compensation(text: String) -> String {}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn test_lens() {
        assert_eq!(lens("FE 35mm whatever"), "bSony FE 35mm ƒ2.8")
    }
}
