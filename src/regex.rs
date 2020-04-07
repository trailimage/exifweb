//! Common regular expressions
use ::regex::Regex;
use lazy_static::*;

lazy_static! {
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
