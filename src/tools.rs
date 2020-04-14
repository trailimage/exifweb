use crate::Photo;
use hashbrown::HashMap;
use lazy_static::*;
use regex::Regex;
use std::path::{Path, PathBuf};

/// Hash represented as vector of string tuples
pub type Pairs = Vec<(String, String)>;

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
pub fn median(numbers: &mut [i64]) -> f64 {
    if numbers.len() == 1 {
        return numbers[0] as f64;
    }
    numbers.sort();
    let mid = (numbers.len() as f64 / 2.0).floor() as usize;

    if numbers.len() % 2 != 0 {
        numbers[mid] as f64
    } else {
        ((numbers[mid - 1] + numbers[mid]) as f64) / 2.0
    }
}

// Minimum and maximum for a range of values
#[derive(Debug)]
pub struct Limits {
    pub min: f64,
    pub max: f64,
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
fn boundary(numbers: &mut [i64], distance: i64) -> Option<Limits> {
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
        min: q1 - range * (distance as f64),
        max: q3 + range * (distance as f64),
    })
}

/// Simplistic outlier calculation identifies photos that are likely not part of
/// the main sequence.
///
/// - https://en.wikipedia.org/wiki/Outlier
/// - http://www.wikihow.com/Calculate-Outliers
pub fn identify_outliers(photos: &mut Vec<Photo>) {
    let mut times: Vec<i64> = photos
        .iter()
        .filter(|p| p.date_taken.is_some())
        .map(|p| p.date_taken.unwrap().timestamp())
        .collect();

    if let Some(fence) = boundary(&mut times[..], 3) {
        for mut p in photos {
            if p.date_taken.is_none() {
                continue;
            }
            let d = p.date_taken.unwrap().timestamp() as f64;
            if d > fence.max || d < fence.min {
                p.outlier_date = true;
            }
        }
    }
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
}
