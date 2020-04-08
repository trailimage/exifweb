use crate::config::CategoryConfig;
use crate::regex as re;
use hashbrown::HashMap;
use lazy_static::*;
use regex::{Captures, NoExpand, Regex};

/// Material icon tag
///
/// https://material.io/icons/
pub fn icon_tag(name: &str) -> String {
    format!("<i class=\"material-icons {}\">{}</i>", name, name)
}

/// HTML tag for post category icon
pub fn category_icon(kind: &str, config: &CategoryConfig) -> String {
    let icon = match kind.to_lowercase().as_str() {
        "who" => &config.icon.who,
        "what" => &config.icon.what,
        "when" => &config.icon.when,
        "where" => &config.icon.r#where,
        _ => &config.icon.default,
    };
    icon_tag(icon)
}

/// HTML tag for mode of travel category icon
pub fn travel_mode_icon(
    what_name: &str,
    mode_icons: HashMap<String, Regex>,
) -> Option<String> {
    for (k, v) in mode_icons.iter() {
        if v.is_match(&what_name) {
            return Some(k.to_owned());
        }
    }

    None
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
fn format_notes(notes: &str) -> String {
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

/// Remove block quotes and wrap in fake tags that won't match subsequent
/// operations
fn unformat_quote(html: &str) -> String {
    re::BLOCK_QUOTE
        .replace_all(&html, |c: &Captures| {
            let quote =
                re::CURLY_QUOTE.replace_all(&c["quote"], "").into_owned();
            format!("[Q]{}[/Q]", quote)
        })
        .into_owned()
}

fn format_quote(text: &str) -> String {
    lazy_static! {
        static ref AFTER_BLOCK_QUOTE: Regex =
            Regex::new(r"\[/Q\][\r\n\s]*([^<]+)").unwrap();
        static ref START_BLOCK_QUOTE: Regex =
            Regex::new(r"(<p>)?\[Q]").unwrap();
        static ref END_BLOCK_QUOTE: Regex =
            Regex::new(r"\[/Q\](</p>)?").unwrap();
    }
    let mut html: String = text.to_string();

    // restore block quotes
    html = AFTER_BLOCK_QUOTE
        .replace_all(&html, "[/Q]<p class=\"first\">$1")
        .into_owned();
    html = START_BLOCK_QUOTE
        .replace_all(&html, "<blockquote><p>")
        .into_owned();
    END_BLOCK_QUOTE
        .replace_all(&html, "</p></blockquote>")
        .into_owned()
}

/// Convert new lines to HTML paragraphs and normalize links
pub fn caption(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    const POEM: &'static str = "[POEM]";
    let mut footnotes: String = String::new();
    let mut poem: String = String::new();
    let mut html: String = text.to_string();

    // format footnotes separately
    html = re::FOOTNOTE_TEXT
        .replace_all(&html, |c: &Captures| {
            footnotes = format_notes(&c["notes"]);
            ""
        })
        .into_owned();

    // set poetry aside and replace with placeholder
    html = re::POETRY
        .replace_all(&html, |c: &Captures| {
            poem = format_poem(&c[2]);
            POEM
        })
        .into_owned();

    html = unformat_quote(&html);
    html = format!("<p>{}</p>", html);
    html = re::NEW_LINE.replace_all(&html, "</p><p>").into_owned();
    html = re::EMPTY_P_TAG.replace_all(&html, "").into_owned();
    html = re::QUIP
        .replace_all(&html, |c: &Captures| {
            format!("<p class=\"quip\">{}", &c[2])
        })
        .into_owned();

    html = re::FOOTNOTE_NUMBER
        .replace_all(&html, "$1<sup>$2</sup>")
        .into_owned();

    html = format_quote(&html);

    if !poem.is_empty() {
        html = html.replace(POEM, &format!("</p>{}<p class=\"first\">", poem));
        html = re::EMPTY_P_TAG.replace_all(&html, "").into_owned();
    }

    format!("{}{}", html, footnotes)
}

/// Format poetry text within a blockquote
fn format_poem(text: &str) -> String {
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

    poem = re::TRAILING_SPACE.replace_all(&poem, "").into_owned();
    poem = re::LINE_BREAK.replace_all(&poem, "<br/>").into_owned();
    poem = MULTI_BREAK.replace_all(&poem, "</p><p>").into_owned();
    poem = INDENT
        .replace_all(&poem, "<span class=\"tab\"></span>")
        .into_owned();
    poem = re::FOOTNOTE_NUMBER
        .replace_all(&poem, "$1<sup>$2</sup>")
        .into_owned();

    format!("<blockquote class=\"poem\"><p>{}</p></blockquote>", poem)
}

#[cfg(test)]
mod tests {
    use super::{
        caption, category_icon, format_notes, format_quote, fraction, icon_tag,
        photo_tag_list, travel_mode_icon, unformat_quote,
    };
    use crate::config::{CategoryConfig, CategoryIcon};
    use crate::tools::config_regex;

    const NEW_LINE: &str = "\r\n";
    const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    const QUOTE:&str = "Firefighters are working to get a handle on several wildfires that sparked during a lightning storm on Thursday night. Strong winds and poor visibility created challenges for firefighters working the blazes on Saturday ...";

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
            "* Note about photo credit{cr}¹ Some other note{cr}² Last note",
            cr = NEW_LINE
        );
        let target = "<ol class=\"footnotes\" start=\"0\"><li class=\"credit\"><i class=\"material-icons star\">star</i><span>Note about photo credit</span></li><li><span>Some other note</span></li><li><span>Last note</span></li></ol>";

        assert_eq!(format_notes(&source), target);
    }

    #[test]
    fn photo_tag_list_test() {
        let mut tags = vec!["Second", "First", "Third and Last"];
        let target = "<a href=\"/photo-tag/first\" rel=\"tag\">First</a> <a href=\"/photo-tag/second\" rel=\"tag\">Second</a> <a href=\"/photo-tag/thirdandlast\" rel=\"tag\">Third and Last</a> ";

        assert_eq!(photo_tag_list(&mut tags), target);
    }

    #[test]
    fn category_icon_test() {
        let config = CategoryConfig {
            icon: CategoryIcon {
                who: "person".to_owned(),
                what: "directions".to_owned(),
                when: "date_range".to_owned(),
                r#where: "map".to_owned(),
                default: "local_offer".to_owned(),
            },
            what_regex: None,
        };

        assert_eq!(category_icon(&"who", &config), icon_tag("person"));
        assert_eq!(category_icon(&"what", &config), icon_tag("directions"));
        assert_eq!(category_icon(&"nope", &config), icon_tag("local_offer"));
    }

    #[test]
    fn travel_mode_test() {
        let config = CategoryConfig {
            icon: CategoryIcon {
                who: "person".to_owned(),
                what: "directions".to_owned(),
                when: "date_range".to_owned(),
                r#where: "map".to_owned(),
                default: "local_offer".to_owned(),
            },
            what_regex: Some(vec![
                ("motorcycle".to_owned(), "(KTM|BMW|Honda)".to_owned()),
                ("bicycle".to_owned(), "bicycle".to_owned()),
            ]),
        };

        assert_eq!(
            travel_mode_icon(&"KTM", config_regex(config.what_regex)),
            Some("motorcycle".to_owned())
        );
    }

    #[test]
    fn caption_ending_with_quote_test() {
        let source =
            format!("{txt}{cr}{cr}“{txt}”", txt = LIPSUM, cr = NEW_LINE);
        let target = format!(
            "<p>{txt}</p><blockquote><p>{txt}</p></blockquote>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_quoted_paragraph_test() {
        let source = format!(
            "{txt}{cr}{cr}“{txt}{cr}{cr}“{txt}{cr}{cr}“{txt}”",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p><blockquote><p>{txt}</p><p>{txt}</p><p>{txt}</p></blockquote>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_quote_within_text_test() {
        let source = format!(
            "{txt}{cr}{cr}“{txt}”{cr}{cr}{txt}",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p><blockquote><p>{txt}</p></blockquote><p class=\"first\">{txt}</p>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn unformat_quote_test() {
        let source = format!(
            "{txt}{cr}{cr}“{q}”¹{cr}{cr}{txt}”",
            txt = LIPSUM,
            q = QUOTE,
            cr = NEW_LINE
        );
        let target = format!("{txt}[Q]{p}¹[/Q]{txt}”", txt = LIPSUM, p = QUOTE);
        assert_eq!(unformat_quote(&source), target);
    }

    #[test]
    fn format_quote_test() {
        let source = format!(
            "{txt}</p>[Q]{q}<sup>¹</sup>[/Q]{txt}”",
            txt = LIPSUM,
            q = QUOTE
        );
        let target = format!(
            "{txt}</p><blockquote><p>{q}<sup>¹</sup></p></blockquote><p class=\"first\">{txt}”",
            txt = LIPSUM,
            q = QUOTE
        );
        assert_eq!(format_quote(&source), target);
    }

    #[test]
    fn caption_block_quote_ellipsis_test() {
        let source = format!(
            "{txt}{cr}{cr}“{q}”¹{cr}{cr}{txt}",
            txt = LIPSUM,
            q = QUOTE,
            cr = NEW_LINE
        );
        let target = format!("<p>{txt}</p><blockquote><p>{p}<sup>¹</sup></p></blockquote><p class=\"first\">{txt}</p>", p = QUOTE, txt = LIPSUM);

        assert_eq!(caption(&source), target);
    }
}
