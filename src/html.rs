use lazy_static::*;
use regex::Regex;

/// Material icon tag
///
/// https://material.io/icons/
pub fn icon_tag(name: &str) -> String {
    format!("<i class=\"material-icons {}\">{}</i>", name, name)
}

/// Stylize punctuation
pub fn typography(s: &str) -> String {
    lazy_static! {
        static ref RIGHT_SINGLE_QUOTE: Regex = Regex::new(r"(\w)'").unwrap();
        static ref LEFT_SINGLE_QUOTE: Regex = Regex::new(r"\b'(\w)").unwrap();
        static ref RIGHT_DOUBLE_QUOTE: Regex =
            Regex::new(r#"([\w,])("|&quot;)"#).unwrap();
        static ref LEFT_DOUBLE_QUOTE: Regex =
            Regex::new(r#"("|&quot;)(\w)"#).unwrap();
        //static ref HTML_QUOTES: Regex = Regex::new("(&ldquo;|&rdquo;)").unwrap();
        // Link with HTML encoded attribute quotes.
        // Capture opening link tag and link text.
        // static ref ENCODED_LINK: Regex =
        //     Regex::new("(<a [^>]+>)([^<]+)</a>").unwrap();
    }
    if s.is_empty() {
        return s.to_owned();
    }

    let text = RIGHT_SINGLE_QUOTE.replace_all(s, "$1&rsquo;");
    let text = LEFT_SINGLE_QUOTE.replace_all(&text, "&lsquo;$1");
    let text = RIGHT_DOUBLE_QUOTE.replace_all(&text, "$1&rdquo;");
    let text = LEFT_DOUBLE_QUOTE.replace_all(&text, "&ldquo;$2");

    text.into_owned()
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
    use super::{fraction, typography};
    use hashbrown::HashMap;

    #[test]
    fn typography_test() {
        let expect: HashMap<&str, &str> = [
            (r#""He said," she said"#, "&ldquo;He said,&rdquo; she said"),
            // This should no longer be necessary since Flickr isn't encoding
            // links
            // (
            //     r#"<a href="/page">so you "say"</a>"#,
            //     r#"<a href="/page">so you &ldquo;say&rdquo;</a>"#,
            // ),
        ]
        .iter()
        .cloned()
        .collect();

        for (k, v) in expect.iter() {
            assert_eq!(typography(k), *v);
        }
    }

    #[test]
    fn fraction_test() {
        assert_eq!(fraction("1/2"), "<sup>1</sup>&frasl;<sub>2</sub>");
    }
}
