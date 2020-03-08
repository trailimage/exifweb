use chrono::{DateTime, Local};
use lazy_static::*;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{error, fmt};

/// Hash represented as vector of string tuples
pub type Pairs = Vec<(String, String)>;

/// Error loading configuration, posts or photos
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug, Clone)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn tab(n: usize) -> usize {
    n * 3
}

// This is important for other errors to wrap this one.
impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

/// Unix epoch time (January 1, 1970)
pub fn min_date() -> DateTime<Local> {
    DateTime::from(SystemTime::UNIX_EPOCH)
}

/// Whether path ends with an extension
pub fn has_ext(p: &PathBuf, ext: &str) -> bool {
    path_name(p).ends_with(ext)
}

/// Convert path name to printable string (ignores errors)
pub fn path_name(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}

/// Update text by replacing source with target values from a `Pairs` hash
pub fn replace_pairs(text: String, pairs: &[(String, String)]) -> String {
    let mut clean = text;
    for (x, y) in pairs {
        if clean.starts_with(x) {
            clean = clean.replace(x, y);
        }
    }
    clean
}

/// Use regex to capture position value from path (ignores errors)
pub fn pos_from_path(re: &Regex, path: &Path) -> Option<u8> {
    pos_from_name(re, path_name(&path))
}

/// Use regex to capture position value from file name (ignores errors)
pub fn pos_from_name(re: &Regex, name: &str) -> Option<u8> {
    re.captures(name).and_then(|caps| caps[1].parse().ok())
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
