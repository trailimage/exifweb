use chrono::{DateTime, Local};
use lazy_static::*;
use regex::Regex;
use std::time::SystemTime;

/// Unix epoch time (January 1, 1970)
pub fn min_date() -> DateTime<Local> {
    DateTime::from(SystemTime::UNIX_EPOCH)
}

pub fn slugify(s: &str) -> String {
    lazy_static! {
        static ref MIX_CASE: Regex = Regex::new(r"([a-z])([A-Z])").unwrap();
        static ref UNDERSCORE: Regex = Regex::new(r"[_\s/-]+").unwrap();
        static ref NON_LETTER: Regex = Regex::new(r"[^\-a-z0-9]").unwrap();
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
