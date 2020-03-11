use lazy_static::*;
use regex::Regex;

/// Material icon tag
///
/// https://material.io/icons/
pub fn icon_tag(name: &str) -> String {
    format!("<i class=\"material-icons {}\">{}</i>", name, name)
}

pub fn fraction(f: &str) -> String {
    lazy_static! {
        static ref SLASH_NUMBERS: Regex = Regex::new(r"(\d+)/(\d+)").unwrap();
    }
    SLASH_NUMBERS
        .replace_all(f, "<sup>$1</sup>&frasl;<sub>$2</sub>")
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::fraction;

    #[test]
    fn fraction_test() {
        assert_eq!(fraction("1/2"), "<sup>1</sup>&frasl;<sub>2</sub>");
    }
}
