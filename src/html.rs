use crate::regex as re;
use lazy_static::*;
use regex::{NoExpand, Regex};

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

/// Replace UTF superscript with HTML superscript
pub fn footnotes(notes: &str) -> String {
    lazy_static! {
        static ref ASTERISK: Regex = Regex::new(r"^\s*\*").unwrap();
        static ref SUPERSCRIPT: Regex =
            Regex::new(r"[⁰¹²³⁴⁵⁶⁷⁸⁹]+\s*").unwrap();
        // trailing empty item
        static ref EMPTY_ITEM: Regex =
            Regex::new(r"</span></li><li><span>\s*$").unwrap();
        static ref ASTERISK_ITEM: Regex =
            Regex::new(r"<li><span>\s*\*\s*").unwrap();
    }

    let has_asterisk: bool = ASTERISK.is_match(notes);
    // photo credit asterisk becomes note 0
    let li_start = if has_asterisk { " start=\"0\"" } else { "" };

    let html = SUPERSCRIPT.replace_all(notes, "");
    let html = re::LINE_BREAK.replace_all(&*html, "</span></li><li><span>");
    let html = EMPTY_ITEM.replace_all(&*html, "");
    let html = format!(
        "<ol class=\"footnotes\"{}><li><span>{}</span></li></ol>",
        li_start, html
    );

    if has_asterisk {
        let replacement =
            format!("<li class=\"credit\">{}<span>", icon_tag("star"));

        return ASTERISK_ITEM
            .replace_all(&html, NoExpand(&replacement))
            .into_owned();
    }

    html
}

/// Linked list of photo tags
pub fn photo_tag_list(list: &mut Vec<&str>) -> String {
    let mut tag_list: String = String::new();

    list.sort();

    for t in list.iter() {
        let slug = re::NON_WORD.replace_all(&t.to_lowercase(), "").into_owned();
        let tag =
            format!("<a href=\"/photo-tag/{}\" rel=\"tag\">{}</a> ", slug, t);

        tag_list.push_str(&tag);
    }

    tag_list
}

/// If link text is a web address, replace with just domain and page
// pub fn shorten_link_text(text: &str) -> String {

// }

/// Format poetry text within a blockquote
pub fn poem(text: &str) -> String {
    lazy_static! {
        static ref OPEN_QUOTE: Regex = Regex::new(r"^\s*“").unwrap();
        static ref CLOSE_QUOTE: Regex =
            Regex::new(r"”\s*[⁰¹²³⁴⁵⁶⁷⁸⁹]?").unwrap();
        static ref POEM_START: Regex = Regex::new(r"(^|[\r\n]) *“").unwrap();
        static ref POEM_END: Regex =
            Regex::new(r"”([⁰¹²³⁴⁵⁶⁷⁸⁹])? *([\r\n]|$)").unwrap();
        // TODO: this was needed because Flickr collapsed spaces -- validate
        static ref INDENT: Regex = Regex::new(r"· · ").unwrap();

        static ref MULTI_BREAK: Regex = Regex::new(r"(<br/>){2,}").unwrap();
    }

    let mut poem: String = String::from(text);

    if OPEN_QUOTE.is_match(&poem) && CLOSE_QUOTE.is_match(&poem) {
        // Assume poem is block quoted. A false positive is possible if the poem
        // just happens to begin and end with internal quotes (note the
        // dependence on curly quotes).
        poem = POEM_START.replace_all(&poem, "$1").into_owned();
        poem = POEM_END.replace_all(&poem, "$1").into_owned();
    }

    poem = re::TRAILING_SPACE.replace(&poem, "").into_owned();
    poem = re::LINE_BREAK.replace(&poem, "<br/>").into_owned();
    poem = MULTI_BREAK.replace(&poem, "</p><p>").into_owned();
    poem = INDENT
        .replace(&poem, "<span class=\"tab\"></span>")
        .into_owned();
    poem = re::FOOTNOTE_NUMBER
        .replace(&poem, "$1<sup>$2</sup>")
        .into_owned();

    format!("<blockquote class=\"poem\"><p>{}</p></blockquote>", poem)
}

#[cfg(test)]
mod tests {
    use super::{footnotes, fraction, icon_tag, photo_tag_list};

    const NL: &str = "\r\n";
    //const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[test]
    fn icon_tag_test() {
        assert_eq!(
            icon_tag("star"),
            "<i class=\"material-icons star\">star</i>"
        );
    }

    #[test]
    fn fraction_test() {
        assert_eq!(fraction("1/2"), "<sup>1</sup>&frasl;<sub>2</sub>");
    }

    #[test]
    fn footnotes_test() {
        let source = format!(
            "* Note about photo credit{nl}¹ Some other note{nl}² Last note",
            nl = NL
        );
        let target = "<ol class=\"footnotes\" start=\"0\"><li class=\"credit\"><i class=\"material-icons star\">star</i><span>Note about photo credit</span></li><li><span>Some other note</span></li><li><span>Last note</span></li></ol>";

        assert_eq!(footnotes(&source), target);
    }

    #[test]
    fn photo_tag_list_test() {
        let mut tags = vec!["Second", "First", "Third and Last"];
        let target = "<a href=\"/photo-tag/first\" rel=\"tag\">First</a> <a href=\"/photo-tag/second\" rel=\"tag\">Second</a> <a href=\"/photo-tag/thirdandlast\" rel=\"tag\">Third and Last</a> ";

        assert_eq!(photo_tag_list(&mut tags), target);
    }
}
