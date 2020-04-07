use chrono::{DateTime, Local};
use hashbrown::HashMap;
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

/// Convert vector of tuples, used in configuration, to hashmap of strings and
/// regular expressions
pub fn config_regex(pairs: Option<Pairs>) -> HashMap<String, Regex> {
    let mut h: HashMap<String, Regex> = HashMap::new();

    if let Some(pair) = pairs {
        for p in pair {
            h.insert(p.0, Regex::new(&p.1).unwrap());
        }
    }
    h
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

/// Middle value or average of the two middle values among a list of numbers
pub fn median(numbers: &mut [i32]) -> f32 {
    if numbers.len() == 1 {
        return numbers[0] as f32;
    }
    numbers.sort();
    let mid = (numbers.len() as f64 / 2.0).floor() as usize;

    if numbers.len() % 2 != 0 {
        numbers[mid] as f32
    } else {
        ((numbers[mid - 1] + numbers[mid]) as f32) / 2.0
    }
}

#[derive(Debug)]
pub struct Limits {
    min: f32,
    max: f32,
}

impl PartialEq for Limits {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl Eq for Limits {}

/// Calculate Tukey fence values for a range of numbers. Values outside the
/// range are considered outliers.
///
/// *Parameters*
///
/// `distance`: Constant used to calculate fence. Tukey proposed `1.5` for an
/// "outlier" and `3` for "far out". This method defaults to `3` if no value is
/// given.
pub fn boundary(numbers: &mut [i32], distance: i32) -> Option<Limits> {
    if numbers.is_empty() {
        return None;
    }

    numbers.sort();
    let len = numbers.len();
    let mid = len / 2;
    // first quartile
    let q1 = median(&mut numbers[0..mid]);
    // third quartile
    let q3 = median(&mut numbers[mid..len - 1]);
    // interquartile range: range of the middle 50% of the data
    let range = q3 - q1;

    Some(Limits {
        min: q1 - range * (distance as f32),
        max: q3 + range * (distance as f32),
    })
}

#[cfg(test)]
mod tests {
    use super::{boundary, median, slugify, Limits};
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

    #[test]
    fn median_test() {
        assert_eq!(median(&mut [1, 2, 3]), 2.0);
        assert_eq!(median(&mut [3]), 3.0);
        assert_eq!(median(&mut [4, 5, 6, 7]), 5.5);
    }

    #[test]
    fn boundary_test() {
        assert_eq!(
            boundary(&mut [1, 2, 3, 9, 27, 36, 22], 3),
            Some(Limits {
                min: -65.5,
                max: 92.0
            })
        );
    }

    // test('calculates median', () => {
    //     expect(median(1, 2, 3)).toBe(2)
    //     expect(median(3)).toBe(3)
    //     expect(median(4, 5, 6, 7)).toBe(5.5)
    //  })

    //  test('calculates Tukey fence boundaries', () => {
    //     expect(boundary([1, 2, 3, 9, 27, 36, 22])).toEqual({ min: -65.5, max: 92 })
    // })
}
