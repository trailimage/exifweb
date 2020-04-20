use super::{Person, Thing};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreativeWork {
    #[serde(flatten)]
    thing: Thing,

    author: Option<Person>,

    height: Option<usize>,
    width: Option<usize>,
    keywords: Option<String>,
    thumbnail_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct MediaObject {
    #[serde(flatten)]
    creative_work: CreativeWork,

    embed_url: Option<String>,
}

/// http://schema.org/ImageObject
#[derive(Serialize, Debug)]
pub struct ImageObject<'a> {
    #[serde(flatten)]
    media_object: MediaObject,

    representative_of_page: Option<bool>,
    caption: Option<String>,
    thumbnail: Option<&'a ImageObject<'a>>,
}

/// An article, such as a news article or piece of investigative report.
/// Newspapers and magazines have articles of many different types and this is
/// intended to cover them all.
///
/// http://schema.org/Article
///
#[derive(Serialize, Debug)]
pub struct Article {
    #[serde(flatten)]
    creative_work: CreativeWork,

    article_body: Option<String>,
    article_section: Option<String>,
}

/// http://schema.org/SocialMediaPosting
#[derive(Serialize, Debug)]
pub struct SocialMediaPosting {
    #[serde(flatten)]
    article: Article,

    shared_content: Option<CreativeWork>,
}

/// http://schema.org/BlogPosting
#[derive(Serialize, Debug)]
pub struct BlogPosting {
    #[serde(flatten)]
    social_media_posting: SocialMediaPosting,
}

/// http://schema.org/Blog
#[derive(Serialize, Debug)]
pub struct Blog {
    #[serde(flatten)]
    thing: Thing,

    blog_post: Vec<BlogPosting>,
}
