use chrono::{DateTime, Local};
use lazy_static::*;
use regex::Regex;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::SystemTime;

/// Hash represented as vector of string tuples
pub type Pairs = Vec<(String, String)>;

/// Unix epoch time (January 1, 1970)
pub fn min_date() -> DateTime<Local> {
    DateTime::from(SystemTime::UNIX_EPOCH)
}

/// Whether path ends with an extension
pub fn has_ext(p: &PathBuf, ext: &str) -> bool {
    os_text(p.file_name()).ends_with(ext)
}

/// Convert OS string to printable string, ignoring errors
pub fn os_text(t: Option<&OsStr>) -> &str {
    t.unwrap().to_str().unwrap()
}

/// Update text by replacing source with target values from a `Pairs` hash
pub fn replace_pairs(text: String, pairs: &Pairs) -> String {
    let mut clean = text;
    for (x, y) in pairs {
        if clean.starts_with(x) {
            clean = clean.replace(x, y);
        }
    }
    clean
}

/// Convert text to slug (snake-case) format
pub fn slugify(s: &str) -> String {
    lazy_static! {
        // replace camelCase
        static ref MIX_CASE: Regex = Regex::new(r"([a-z])([A-Z])").unwrap();
        // replace underscores with dashes
        static ref UNDERSCORE: Regex = Regex::new(r"[_\s/-]+").unwrap();
        // replace non-alpha-numerics
        static ref NON_LETTER: Regex = Regex::new(r"[^\-a-z0-9]").unwrap();
        // replace multiple dashes with single
        static ref MULTI_DASH: Regex = Regex::new(r"-{2,}").unwrap();
    }

    let mut text: String = MIX_CASE.replace_all(s, "$1-$2").to_lowercase();
    text = UNDERSCORE.replace_all(&text, "-").replace("-&-", "-and-");
    text = NON_LETTER.replace_all(&text, "").into_owned();

    MULTI_DASH.replace_all(&text, "-").into_owned()
}

#[cfg(test)]
mod tests {
    use super::slugify;
    use hashbrown::HashMap;

    #[test]
    fn slugify_test() {
        let expect: HashMap<&str, &str> = [
            ("Wiggle and Roll", "wiggle-and-roll"),
            ("Wiggle and    Sing", "wiggle-and-sing"),
            ("Too---dashing", "too-dashing"),
            ("powerful/oz", "powerful-oz"),
            ("three o' clock", "three-o-clock"),
            ("one_two_Three-48px", "one-two-three-48px"),
            ("camelCase", "camel-case"),
        ]
        .iter()
        .cloned()
        .collect();

        for (k, v) in expect.iter() {
            assert_eq!(slugify(k), *v);
        }
    }
}
