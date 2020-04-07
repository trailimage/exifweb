//! Common regular expressions
use ::regex::Regex;
use lazy_static::*;

lazy_static! {
    pub static ref CURLY_QUOTE: Regex = Regex::new("[“”]").unwrap();
    pub static ref OPEN_QUOTE: Regex = Regex::new(r"^\s*“").unwrap();
    pub static ref NON_WORD: Regex = Regex::new(r"\W").unwrap();

    pub static ref LINE_BREAK: Regex = Regex::new(r"\r*\n").unwrap();

    pub static ref NEW_LINE: Regex = Regex::new(r"(\r\n|\n|\r)").unwrap();

    /// Capture footnoted word and superscript. Match superscripts but don't
    /// match atomic numbers.
    pub static ref FOOTNOTE_NUMBER: Regex =
        Regex::new(r"([^/\s])([⁰¹²³⁴⁵⁶⁷⁸⁹]+)(?!\w)").unwrap();

    /// Footnote text preceded by three underscores
    pub static ref FOOTNOTE_TEXT: Regex =
        Regex::new(r"(^|[\r\n]+)_{3}[\r\n]*([\s\S]+)$").unwrap();

    /// Long quote followed by line break or end of text
    pub static ref BLOCK_QUOTE: Regex =
        Regex::new(r"(\r\n|\r|\n|^)(“[^”]{200,}”[⁰¹²³⁴⁵⁶⁷⁸⁹]*)\s*(\r\n|\r|\n|$)")
            .unwrap();

    pub static ref TRAILING_SPACE: Regex = Regex::new(r"[\r\n\s]*$").unwrap();

    pub static ref EMPTY_P_TAG: Regex = Regex::new(r"<p[^>]*></p>").unwrap();

    /// Capture URL and link text
    pub static ref ANCHOR_TAG: Regex = Regex::new(r#"<a href=["']([^"']+)['"][^>]*>([^<]+)</a>"#).unwrap();

    /// Whether text contains a poem. Exclude dialog by negating comma or
    /// question mark before closing quote unless it's footnoted. Capture
    /// leading space and poem body.
    ///
    /// Match any character but new lines:
    /// ```
    /// [^\r\n]
    /// ```
    ///
    /// Do not match punctuation followed by closing quote (negative look-
    /// ahead) unless the quote mark is followed by a superscript number:
    /// ```
    /// (?![\.,!?]”[^⁰¹²³⁴⁵⁶⁷⁸⁹])
    /// ```
    ///
    /// Match stops at end of text (`$`) or when there are one or more
    /// new-lines (`\r\n`):
    /// ```
    /// ([\r\n]+|$)
    /// ```
    pub static ref POETRY: Regex = Regex::new(r"(^|[\r\n]+)((([^\r\n](?![\.,!?]”[^⁰¹²³⁴⁵⁶⁷⁸⁹])){4,80}([\r\n]+|$)){3,})").unwrap();

    /// Match the first HTML paragraph if it's short and contains a quote
    pub static ref QUIP: Regex = Regex::new(r"(<p>)(“(?=[^<]*”)[^<]{4,80}</p>)").unwrap();
}

#[cfg(test)]
mod tests {
    use super::BLOCK_QUOTE;

    const NL: &str = "\r\n";
    const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[test]
    fn block_quote_test() {
        let quote = format!("“{l}{n}“{l}{n}”{n}", l = LIPSUM, n = NL);
        assert!(BLOCK_QUOTE.is_match(&quote));

        // interrupted block quote should not match
        let quote = format!("“{l},” he said, “{l}{n}", l = LIPSUM, n = NL);
        assert!(!BLOCK_QUOTE.is_match(&quote));
    }
}
