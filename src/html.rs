use crate::{
    config::CategoryIcon,
    models::{Category, CategoryKind},
    tools::slugify,
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use lazy_static::*;
use regex::{Captures, NoExpand, Regex};

lazy_static! {
    static ref TRAILING_SPACE: Regex = Regex::new(r"[\r\n\s]*$").unwrap();
    static ref LINE_BREAK: Regex = Regex::new(r"\r*\n").unwrap();
    static ref NEW_LINE: Regex = Regex::new(r"(\r\n|\n|\r)").unwrap();
}

/// Material icon tag
///
/// https://material.io/icons/
pub fn icon_tag(name: &str) -> String {
    format!("<i class=\"material-icons {}\">{}</i>", name, name)
}

/// Month Day, Year (March 15, 1973)
pub fn date_string(d: DateTime<FixedOffset>) -> String {
    d.format("%B %e, %Y").to_string()
}

/// HTML tag for post category icon
pub fn category_icon(kind: &CategoryKind, config: &CategoryIcon) -> String {
    let icon = match kind.to_string().to_lowercase().as_str() {
        "who" => &config.who,
        "what" => &config.what,
        "when" => &config.when,
        "where" => &config.r#where,
        _ => &config.default,
    };
    icon_tag(icon)
}

/// HTML tag for mode of travel category icon
pub fn travel_mode_icon(
    categories: &Vec<Category>,
    mode_icons: &HashMap<String, Regex>,
) -> Option<String> {
    categories
        .iter()
        .find(|c: &'_ &Category| c.kind == CategoryKind::What)
        .and_then(|c: &Category| {
            for (icon_name, re) in mode_icons.iter() {
                if re.is_match(&c.name) {
                    return Some(icon_name.to_owned());
                }
            }
            None
        })
}

pub fn fraction(f: &str) -> String {
    lazy_static! {
        // two numbers separated by a forward slash
        static ref SLASH_NUMBERS: Regex = Regex::new(r"(\d+)/(\d+)").unwrap();
    }
    SLASH_NUMBERS
        .replace_all(f, "<sup>$1</sup>&frasl;<sub>$2</sub>")
        .into_owned()
}

fn format_superscript(text: &str) -> String {
    lazy_static! {
        // match superscripts but don't match atomic numbers
        static ref FOOTNOTE_NUMBER: Regex =
            Regex::new(r"([^/\s])([⁰¹²³⁴⁵⁶⁷⁸⁹]+)\B").unwrap();
    }
    FOOTNOTE_NUMBER
        .replace_all(text, "$1<sup>$2</sup>")
        .into_owned()
}

/// Convert bare URLs into HTML links
fn link_urls(text: &str) -> String {
    lazy_static! {
        static ref URL: Regex =
            Regex::new(r"\b(?P<url>https?://[^\s]+)\b").unwrap();
        static ref DOMAIN: Regex = Regex::new(r"https?://[^/]+/?").unwrap();
        static ref LAST_PATH: Regex =
            Regex::new(r"/([^/?#]+)(\?|\#|$)").unwrap();
    }

    URL.replace_all(&text, |c: &Captures| {
        let url: &str = &c["url"];
        let domain: &str = &DOMAIN.captures(url).unwrap()[0];
        let path = url.replace(domain, "");
        let domain = domain.replace("//www.", "//");

        if path.contains("/") {
            let page: &str = &LAST_PATH.captures(&path).unwrap()[1];
            format!("<a href=\"{}\">{}&hellip;/{}</a>", url, domain, page)
        } else {
            format!("<a href=\"{}\">{}{}</a>", url, domain, path)
        }
    })
    .into_owned()
}

/// Replace UTF superscript with HTML superscript
fn format_footnotes(notes: &str) -> String {
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
    let mut html: String = link_urls(&notes);

    html = SUPERSCRIPT.replace_all(&html, "").into_owned();
    html = LINE_BREAK
        .replace_all(&html, "</span></li><li><span>")
        .into_owned();
    html = EMPTY_ITEM.replace_all(&html, "").into_owned();
    html = format!(
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
pub fn photo_tag_list(list: &Vec<String>) -> String {
    lazy_static! {
        static ref NON_WORD: Regex = Regex::new(r"\W").unwrap();
    }
    let mut tag_list: Vec<String> = Vec::new();

    for t in list.iter() {
        tag_list.push(format!(
            "<a href=\"/photo-tag/{}\" rel=\"tag\">{}</a>",
            slugify(&t),
            t
        ));
    }

    tag_list.sort();
    tag_list.join("\n")
}

/// Remove block quotes and wrap in fake tags that won't match subsequent
/// operations
fn unformat_block_quote(html: &str) -> String {
    lazy_static! {
        // long quote followed by line break or end of text
        static ref BLOCK_QUOTE: Regex =
            Regex::new(r"(\r\n|\r|\n|^)\s*(?P<quote>“[^”]{200,}”[⁰¹²³⁴⁵⁶⁷⁸⁹]*)\s*(\r\n|\r|\n|$)").unwrap();
        static ref CURLY_QUOTE: Regex = Regex::new("[“”]").unwrap();
    }
    BLOCK_QUOTE
        .replace_all(&html, |c: &Captures| {
            let quote = CURLY_QUOTE.replace_all(&c["quote"], "").into_owned();
            format!("[Q]{}[/Q]", quote)
        })
        .into_owned()
}

/// Restore HTML blockquote tags
fn format_block_quote(text: &str) -> String {
    lazy_static! {
        static ref AFTER_BLOCK_QUOTE: Regex =
            Regex::new(r"\[/Q\][\r\n\s]*([^<]+)").unwrap();
        static ref START_BLOCK_QUOTE: Regex =
            Regex::new(r"(<p>)?\[Q\]").unwrap();
        static ref END_BLOCK_QUOTE: Regex =
            Regex::new(r"\[/Q\](</p>)?").unwrap();
        /// Starting block quote with closing p tag will make an orphan if there
        /// isn't any preceding text
        static ref P_ORPHAN: Regex = Regex::new(r"^</p>").unwrap();
    }
    let mut html: String = text.to_string();

    html = AFTER_BLOCK_QUOTE
        .replace_all(&html, "[/Q]<p class=\"first\">$1")
        .into_owned();
    html = START_BLOCK_QUOTE
        .replace_all(&html, "</p><blockquote><p>")
        .into_owned();
    html = END_BLOCK_QUOTE
        .replace_all(&html, "</p></blockquote>")
        .into_owned();

    P_ORPHAN.replace_all(&html, "").into_owned()
}

/// Convert new lines to HTML paragraphs and normalize links
pub fn caption(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    lazy_static! {
        static ref EMPTY_P_TAG: Regex = Regex::new(r"<p[^>]*></p>").unwrap();
        // match the first HTML paragraph if it's short and contains a quote
        static ref QUIP: Regex = Regex::new(r"^\s*<p>(?P<quote>“[^”]{4,80}”[^<]{0,50})</p>").unwrap();
        // poems are preceded and followed by lone tilde (~)
        static ref POEM: Regex = Regex::new(r"(^|\s+)~(\r\n|\r|\n)(?P<poem>([^\r\n]{3,100}([\r\n]+)){3,})~(\s+|$)").unwrap();
        // notes are preceded by three underscores (___) and followed by EOF
        static ref FOOTNOTES: Regex =
            Regex::new(r"(^|[\r\n]+)_{3}[\r\n]*(?P<notes>[\s\S]+)$").unwrap();
    }

    // unique placeholder key for each poem
    let poem_key = |i: usize| format!("[P_{}]", i);
    let mut footnotes: String = String::new();
    let mut poems: Vec<String> = Vec::new();
    let mut html: String = text.to_string();

    // set aside footnotes (no placeholder needed because always last)
    html = FOOTNOTES
        .replace_all(&html, |c: &Captures| {
            footnotes = format_footnotes(&c["notes"]);
            ""
        })
        .into_owned();

    // set aside poems and substitute with placeholder
    html = POEM
        .replace_all(&html, |c: &Captures| {
            poems.push(format_poem(&c["poem"]));
            poem_key(poems.len() - 1)
        })
        .into_owned();

    html = unformat_block_quote(&html);
    html = format!("<p>{}</p>", html);
    html = NEW_LINE.replace_all(&html, "</p><p>").into_owned();
    html = EMPTY_P_TAG.replace_all(&html, "").into_owned();
    html = QUIP
        .replace_all(&html, |c: &Captures| {
            format!("<p class=\"quip\">{}</p>", &c["quote"])
        })
        .into_owned();

    html = format_superscript(&html);
    html = format_block_quote(&html);

    for (i, p) in poems.iter().enumerate() {
        // re-insert poems
        let key = poem_key(i);
        html = html.replace(&key, &format!("</p>{}<p class=\"first\">", p));
        html = EMPTY_P_TAG.replace_all(&html, "").into_owned();
    }

    format!("{}{}", html, footnotes)
}

fn format_line_breaks(text: &str) -> String {
    lazy_static! {
        static ref MULTI_BREAK: Regex = Regex::new(r"(<br/>){2,}").unwrap();
    }
    let text = LINE_BREAK.replace_all(&text, "<br/>").into_owned();
    let text = MULTI_BREAK.replace_all(&text, "</p><p>").into_owned();

    format!("<p>{}</p>", text)
}

fn format_poem(text: &str) -> String {
    lazy_static! {
        // exactly three lines
        static ref HAIKU: Regex =
            Regex::new(r"^([^\r\n]{3,100}([\r\n]+|$)){3}$").unwrap();
        // indentations in poems are three spaces
        static ref POEM_INDENT: Regex = Regex::new(" {3}").unwrap();
    }

    let (css, icon) = if HAIKU.is_match(&text) {
        ("haiku", icon_tag("spa"))
    } else {
        ("poem", String::new())
    };
    let mut poem: String = text.to_owned();

    poem = TRAILING_SPACE.replace_all(&poem, "").into_owned();
    poem = format_line_breaks(&poem);
    poem = POEM_INDENT
        .replace_all(&poem, "<span class=\"tab\"></span>")
        .into_owned();
    poem = format_superscript(&poem);

    format!(
        "<blockquote class=\"{}\">{}{}</blockquote>",
        css, poem, icon
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CategoryConfig, CategoryIcon};
    use crate::{
        models::{Category, CategoryKind},
        tools::config_regex,
    };

    const NEW_LINE: &str = "\r\n";
    const LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    const QUOTE:&str = "Firefighters are working to get a handle on several wildfires that sparked during a lightning storm on Thursday night. Strong winds and poor visibility created challenges for firefighters working the blazes on Saturday ...";

    #[test]
    fn creates_icon_tag() {
        assert_eq!(
            icon_tag("star"),
            "<i class=\"material-icons star\">star</i>"
        );
    }

    #[test]
    fn fraction_html() {
        assert_eq!(fraction("1/2"), "<sup>1</sup>&frasl;<sub>2</sub>");
    }

    #[test]
    fn footnotes_as_ordered_list() {
        let source = format!(
            "* Note about photo credit{cr}\
            ¹ Some other note{cr}\
            ² Last note",
            cr = NEW_LINE
        );
        let target = "<ol class=\"footnotes\" start=\"0\">\
            <li class=\"credit\"><i class=\"material-icons star\">star</i><span>Note about photo credit</span></li>\
            <li><span>Some other note</span></li>\
            <li><span>Last note</span></li>\
            </ol>";

        assert_eq!(format_footnotes(&source), target);
    }

    #[test]
    fn photo_tag_lists() {
        let tags: Vec<String> = vec![
            "Second".to_owned(),
            "First".to_owned(),
            "Third and Last".to_owned(),
        ];
        let target = "<a href=\"/photo-tag/first\" rel=\"tag\">First</a>\n\
            <a href=\"/photo-tag/second\" rel=\"tag\">Second</a>\n\
            <a href=\"/photo-tag/third-and-last\" rel=\"tag\">Third and Last</a>";

        assert_eq!(photo_tag_list(&tags), target);
    }

    #[test]
    fn category_icon_tag() {
        let config = CategoryIcon {
            who: "person".to_owned(),
            what: "directions".to_owned(),
            when: "date_range".to_owned(),
            r#where: "map".to_owned(),
            default: "local_offer".to_owned(),
        };

        assert_eq!(
            category_icon(&CategoryKind::Who, &config),
            icon_tag("person")
        );
        assert_eq!(
            category_icon(&CategoryKind::What, &config),
            icon_tag("directions")
        );
        //assert_eq!(category_icon(&"nope", &config), icon_tag("local_offer"));
    }

    #[test]
    fn travel_mode_icon_tag() {
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

        let categories: Vec<Category> =
            vec![Category::new("KTM", CategoryKind::What)];

        assert_eq!(
            travel_mode_icon(&categories, &config_regex(&config.what_regex)),
            Some("motorcycle".to_owned())
        );
    }

    #[test]
    fn superscripts() {
        let source = format!("{}²", LIPSUM);
        let target = format!("<p>{}<sup>²</sup></p>", LIPSUM);
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn line_breaks() {
        let source =
            format!("one{cr}two{cr}{cr}three{cr}{cr}{cr}four", cr = NEW_LINE);
        let target = "<p>one<br/>two</p>\
            <p>three</p>\
            <p>four</p>";
        assert_eq!(format_line_breaks(&source), target);
    }

    #[test]
    fn url_formatting() {
        const URL1: &str = "http://en.wikipedia.org/wiki/Sweet_Pickles";
        const URL2: &str = "http://www.amazon.com/Cheryl-Dudley/e/B001JP7LNO/ref=ntt_athr_dp_pel_1";
        const URL3: &str = "http://www.trailimage.com/trinity-ridge-fire-tour";

        let source = format!(
            "¹ Wikipedia: {} ² Cheryl Reed, January 17, 2003: {}",
            URL1, URL2,
        );

        let target = format!(
            "¹ Wikipedia: <a href=\"{}\">http://en.wikipedia.org/&hellip;/Sweet_Pickles</a> \
            ² Cheryl Reed, January 17, 2003: <a href=\"{}\">http://amazon.com/&hellip;/ref=ntt_athr_dp_pel_1</a>",
            URL1, URL2,
        );

        assert_eq!(link_urls(&source), target);

        let source =
            format!("¹ Trail Image, “Trinity Ridge Fire Tour”: {}", URL3);

        let target = format!(
            "¹ Trail Image, “Trinity Ridge Fire Tour”: \
            <a href=\"{}\">http://trailimage.com/trinity-ridge-fire-tour</a>",
            URL3
        );

        assert_eq!(link_urls(&source), target);
    }

    #[test]
    fn poem_formatting() {
        let source = format!(
            "Begotten Not Born{cr}\
            Indwelling Transcendence{cr}      Infinite Regress{cr}\
            Uncertain Progress",
            cr = NEW_LINE
        );
        let target = "<blockquote class=\"poem\"><p>\
            Begotten Not Born<br/>\
            Indwelling Transcendence<br/>\
            <span class=\"tab\"></span><span class=\"tab\"></span>\
            Infinite Regress<br/>\
            Uncertain Progress</p></blockquote>";

        assert_eq!(format_poem(&source), target);
    }

    #[test]
    fn haiku_formatting() {
        let source = format!(
            "cow stands chewing{cr}\
            wet meadow grass{cr}\
            while mud swallows wheels{cr}",
            cr = NEW_LINE
        );
        let target = "<blockquote class=\"haiku\"><p>\
            cow stands chewing<br/>\
            wet meadow grass<br/>\
            while mud swallows wheels</p>\
            <i class=\"material-icons spa\">spa</i>\
            </blockquote>";

        assert_eq!(format_poem(&source), target);
    }

    #[test]
    fn caption_with_footnotes() {
        let source = format!(
            "{txt}{cr}\
            ___{cr}\
            * Note about photo credit{cr}\
            ¹ Some other note{cr}\
            ² Last note",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!("<p>{}</p>\
            <ol class=\"footnotes\" start=\"0\">\
                <li class=\"credit\">\
                <i class=\"material-icons star\">star</i><span>Note about photo credit</span></li>\
                <li><span>Some other note</span></li>\
                <li><span>Last note</span></li>\
            </ol>", LIPSUM);
        assert_eq!(caption(&source), target);

        let source = format!(
            "{txt}{cr}___{cr}¹ Some other note{cr}² Last note",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{}</p>\
            <ol class=\"footnotes\">\
                <li><span>Some other note</span></li>\
                <li><span>Last note</span></li>\
            </ol>",
            LIPSUM
        );
        assert_eq!(caption(&source), target);

        // should ignore trailing newline
        let source = format!("{}{}", source, NEW_LINE);

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_ending_with_block_quote() {
        let source =
            format!("{txt}{cr}{cr}“{txt}”", txt = LIPSUM, cr = NEW_LINE);
        let target = format!(
            "<p>{txt}</p>\
            <blockquote><p>{txt}</p></blockquote>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn block_quoted_paragraphs() {
        let source = format!(
            "{txt}{cr}{cr}\
            “{txt}{cr}{cr}\
            “{txt}{cr}{cr}\
            “{txt}”",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p>\
            <blockquote>\
                <p>{txt}</p>\
                <p>{txt}</p>\
                <p>{txt}</p>\
            </blockquote>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn block_quote_within_other_text() {
        let source = format!(
            "{txt}{cr}{cr}\
            “{txt}”{cr}{cr}\
            {txt}",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p>\
            <blockquote><p>{txt}</p></blockquote>\
            <p class=\"first\">{txt}</p>",
            txt = LIPSUM
        );
        assert_eq!(caption(&source), target);
    }

    #[test]
    fn intermediate_block_quote_format() {
        let source = format!(
            "{txt}{cr}{cr}“{q}”¹{cr}{cr}{txt}”",
            txt = LIPSUM,
            q = QUOTE,
            cr = NEW_LINE
        );
        let target = format!("{txt}[Q]{p}¹[/Q]{txt}”", txt = LIPSUM, p = QUOTE);
        assert_eq!(unformat_block_quote(&source), target);
    }

    #[test]
    fn footnoted_block_quote() {
        let source = format!(
            "{txt}[Q]{q}<sup>¹</sup>[/Q]{txt}”",
            txt = LIPSUM,
            q = QUOTE
        );
        let target = format!(
            "{txt}</p>\
            <blockquote><p>{q}<sup>¹</sup></p></blockquote>\
            <p class=\"first\">{txt}”",
            txt = LIPSUM,
            q = QUOTE
        );
        assert_eq!(format_block_quote(&source), target);
    }

    #[test]
    fn block_quote_with_ellipsis() {
        let source = format!(
            "{txt}{cr}{cr}“{q}”¹{cr}{cr}{txt}",
            txt = LIPSUM,
            q = QUOTE,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p>\
            <blockquote><p>{p}<sup>¹</sup></p></blockquote>\
            <p class=\"first\">{txt}</p>",
            p = QUOTE,
            txt = LIPSUM
        );

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_that_is_entirely_quote() {
        let source = format!("“{}”¹", LIPSUM);
        let target =
            format!("<blockquote><p>{}<sup>¹</sup></p></blockquote>", LIPSUM);

        assert_eq!(caption(&source), target);
    }

    // do no blockquote when quote is interrupted
    // “The constitutions of nearly all the states have qualifications for voters simply on citizenship,” Pefley countered, “without question with regard to what they believe on this or that question. Then I ask, why make a distinction of the people of Idaho?
    // “It appears to have been reserved for Idaho’s constitution to put in the first religious test in regard to the right of suffrage and holding office … Political and religious persecution are supposed to have died at the termination of the revolution but it appears that Idaho is again an exception.”¹
    // Pefley’s arguments were unheeded and the section was approved.
    #[test]
    fn interrupted_quotes() {
        let source = format!(
            "“{txt},” he said, “{txt}{cr}{cr}“{txt}”{cr}{cr}",
            txt = LIPSUM,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>“{txt},” he said, “{txt}</p>\
            <blockquote><p>{txt}</p></blockquote>",
            txt = LIPSUM
        );

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_that_is_entirely_poem() {
        let source = format!(
            "~{cr}\
            Begotten Not Born{cr}\
            Indwelling Transcendence{cr}      Infinite Regress{cr}\
            Uncertain Progress{cr}\
            ~",
            cr = NEW_LINE
        );
        let target = "<blockquote class=\"poem\"><p>\
            Begotten Not Born<br/>\
            Indwelling Transcendence<br/>\
            <span class=\"tab\"></span><span class=\"tab\"></span>\
            Infinite Regress<br/>\
            Uncertain Progress</p></blockquote>";

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn caption_beginning_with_haiku() {
        let source = format!(
            "~{cr}\
            cow stands chewing{cr}\
            wet meadow grass{cr}\
            while mud swallows wheels{cr}\
            ~{cr}\
            Here we have Joel “Runs with Cows” Abbott. \
            He did a little loop out among them—kind of became one of them.",
            cr = NEW_LINE
        );
        let target = "<blockquote class=\"haiku\"><p>\
            cow stands chewing<br/>\
            wet meadow grass<br/>\
            while mud swallows wheels\
            </p><i class=\"material-icons spa\">spa</i>\
            </blockquote>\
            <p class=\"first\">Here we have Joel “Runs with Cows” Abbott. \
            He did a little loop out among them—kind of became one of them.</p>";

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn long_inline_poem() {
        let poem_text = format!(
            "~{cr}\
            Have you ever stood on the top of a mountain{cr}\
            And gazed down on the grandeur below{cr}\
            And thought of the vast army of people{cr}   \
               Who never get out as we go?{cr}\
            {cr}\
            Have you ever trailed into the desert{cr}\
            Where the hills fade from gold into blue,{cr}\
            And then thought of some poor other fellow{cr}\
            Who would like to stand alongside of you?{cr}\
            ~",
            cr = NEW_LINE
        );

        const POEM_HTML: &str = "<blockquote class=\"poem\">\
            <p>\
            Have you ever stood on the top of a mountain<br/>\
            And gazed down on the grandeur below<br/>\
            And thought of the vast army of people<br/>\
            <span class=\"tab\"></span>Who never get out as we go?</p>\
            <p>\
            Have you ever trailed into the desert<br/>\
            Where the hills fade from gold into blue,<br/>\
            And then thought of some poor other fellow<br/>\
            Who would like to stand alongside of you?</p>\
            </blockquote>";

        // no text after poem
        let source = format!(
            "{txt}{cr}{cr}{poem}",
            txt = LIPSUM,
            cr = NEW_LINE,
            poem = poem_text
        );
        let target =
            format!("<p>{txt}</p>{poem}", txt = LIPSUM, poem = POEM_HTML);

        assert_eq!(caption(&source), target);

        // text after poem
        let source = format!(
            "{txt}{cr}{cr}{poem}{cr}{cr}{txt}",
            txt = LIPSUM,
            poem = poem_text,
            cr = NEW_LINE
        );
        let target = format!(
            "<p>{txt}</p>{poem}<p class=\"first\">{txt}</p>",
            txt = LIPSUM,
            poem = POEM_HTML
        );

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn does_not_make_conversation_into_poem() {
        let source = format!(
            "“What’s wrong Brenna?” I ask.{cr}\
            {cr}\
            “I can’t sleep.”{cr}\
            {cr}\
            “Just lay down.”{cr}\
            {cr}\
            “I can’t.”{cr}\
            {cr}\
            “Brenna,” I insist, “lay down.”",
            cr = NEW_LINE
        );
        let target = "<p class=\"quip\">“What’s wrong Brenna?” I ask.</p>\
            <p>“I can’t sleep.”</p>\
            <p>“Just lay down.”</p>\
            <p>“I can’t.”</p>\
            <p>“Brenna,” I insist, “lay down.”</p>";

        assert_eq!(caption(&source), target);
    }

    #[test]
    fn footnoted_poem() {
        const P1: &str = "Now many years have passed since we lived there \
            and little connects us to that place—now in other hands—other than \
            our shared memories. My mom has written of Our Old House:";

        const P3: &str =
            "This particular weekend had an additional attraction, my nephew \
            Kaden’s seventh birthday party. I don’t see my nephews often so I \
            was glad for the coincidence of events.";

        const URL1: &str = "http://en.wikipedia.org/wiki/Sweet_Pickles";
        const URL2: &str = "http://www.amazon.com/Cheryl-Dudley/e/B001JP7LNO/ref=ntt_athr_dp_pel_1";

        let source = format!(
            "{p1}{cr}\
            ~{cr}\
            When I drive by I always think I see myself{cr}\
            standing in the large picture window waving,{cr}\
            wishing I’d stop by and have a spot of tea.{cr}\
            {cr}\
            But I know its only what I want{cr}\
            because I didn’t want to leave, you see,{cr}\
            and when I drive by, smell the row{cr}\
            of lilacs I planted along the road,{cr}\
            see the gray smoke curling from the chimney,{cr}\
            {cr}\
            I want to pull in and stop,{cr}\
            pretend I never left, unload the groceries,{cr}\
            stoke the fire, straighten the photos on the wall{cr}\
            and wash the dishes that have stacked{cr}\
            by the sink for the last ten years.{cr}\
            {cr}\
            You’d be there, too, in your blue pajamas{cr}\
            asking for a story. We’d climb the narrow{cr}\
            staircase to your room and turn on the lamp,{cr}\
            listening for a moment to the frogs outside,{cr}\
            that bellowed thousands strong.{cr}\
            {cr}\
            I’d read your Sweet Pickles books¹{cr}\
            and sing that Bumble Bee song you loved.{cr}\
            Then we’d lay quietly and never grow old,{cr}\
            while time went on without us, down{cr}\
            the dusty country road, slipping over the horizon,{cr}\
            leaving a soft orange glow for us to read by.²{cr}\
            ~{cr}\
            In recent years I’ve tried to make the annual, three-hundred mile \
            pilgrimage to “Troy Days.”³ Starchy pancake-feed food, a couple \
            fire trucks and horses paraded down main street, and an evening of \
            under-age inebriation make a good time, of course, but my trip is \
            not for those things. Troy Days is when and where my dad’s \
            brothers reunite annually from their homes across the western U.S. \
            In their company, my mind can visit our old house, find a place \
            alongside my dad, my grandma and the rest seated around a fire, \
            our eyes all reflecting the same eternal glow.{cr}\
            {cr}\
            {p3}{cr}\
            ___{cr}\
            ¹ Wikipedia: {url1}{cr}\
            ² Cheryl Reed, January 17, 2003: {url2}",
            cr = NEW_LINE,
            p1 = P1,
            p3 = P3,
            url1 = URL1,
            url2 = URL2
        );

        let target = format!("<p>{p1}</p>\
            <blockquote class=\"poem\"><p>\
            When I drive by I always think I see myself<br/>\
            standing in the large picture window waving,<br/>\
            wishing I’d stop by and have a spot of tea.\
            </p><p>\
            But I know its only what I want<br/>\
            because I didn’t want to leave, you see,<br/>\
            and when I drive by, smell the row<br/>\
            of lilacs I planted along the road,<br/>\
            see the gray smoke curling from the chimney,\
            </p><p>\
            I want to pull in and stop,<br/>\
            pretend I never left, unload the groceries,<br/>\
            stoke the fire, straighten the photos on the wall<br/>\
            and wash the dishes that have stacked<br/>\
            by the sink for the last ten years.\
            </p><p>\
            You’d be there, too, in your blue pajamas<br/>\
            asking for a story. We’d climb the narrow<br/>\
            staircase to your room and turn on the lamp,<br/>\
            listening for a moment to the frogs outside,<br/>\
            that bellowed thousands strong.\
            </p><p>\
            I’d read your Sweet Pickles books<sup>¹</sup><br/>\
            and sing that Bumble Bee song you loved.<br/>\
            Then we’d lay quietly and never grow old,<br/>\
            while time went on without us, down<br/>\
            the dusty country road, slipping over the horizon,<br/>\
            leaving a soft orange glow for us to read by.<sup>²</sup>\
            </p></blockquote>\
            <p class=\"first\">\
            In recent years I’ve tried to make the annual, three-hundred mile \
            pilgrimage to “Troy Days.”<sup>³</sup> Starchy pancake-feed food, a couple \
            fire trucks and horses paraded down main street, and an evening of \
            under-age inebriation make a good time, of course, but my trip is \
            not for those things. Troy Days is when and where my dad’s \
            brothers reunite annually from their homes across the western U.S. \
            In their company, my mind can visit our old house, find a place \
            alongside my dad, my grandma and the rest seated around a fire, \
            our eyes all reflecting the same eternal glow.</p>\
            <p>{p3}</p>\
            <ol class=\"footnotes\">\
                <li><span>Wikipedia: {link1}</span></li>\
                <li><span>Cheryl Reed, January 17, 2003: {link2}</span></li>\
            </ol>",
            p1 = P1,
            p3 = P3,
            link1 = link_urls(URL1),
            link2 = link_urls(URL2)
        );

        assert_eq!(caption(&source), target);
    }
}
