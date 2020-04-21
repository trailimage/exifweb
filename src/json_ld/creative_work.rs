use super::{Person, Thing};
use crate::config::ImageConfig;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreativeWork<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    author: Option<&'a Person<'a>>,

    height: Option<u16>,
    width: Option<u16>,
    keywords: Option<&'a str>,
    thumbnail_url: Option<&'a str>,
}
impl<'a> CreativeWork<'a> {
    pub fn extend(_type: &'a str, id: Option<&'a str>) -> Self {
        CreativeWork {
            thing: Thing::extend(_type, id),
            author: None,
            height: None,
            width: None,
            keywords: None,
            thumbnail_url: None,
        }
    }

    /// Update width and height
    pub fn size(&mut self, width: u16, height: u16) {
        self.width = Some(width);
        self.height = Some(height);
    }
}

/// http://schema.org/Photograph
#[derive(Serialize, Debug)]
pub struct Photograph<'a> {
    #[serde(flatten)]
    creative_work: CreativeWork<'a>,
}
impl<'a> Photograph<'a> {
    pub fn new(url: &'a str) -> Self {
        Photograph {
            creative_work: CreativeWork::extend("Photograph", Some(url)),
        }
    }
}

/// http://schema.org/WebPage
#[derive(Serialize, Debug)]
pub struct WebPage<'a> {
    #[serde(flatten)]
    creative_work: CreativeWork<'a>,

    main_content_of_page: Option<&'a str>,
    primary_image_of_page: Option<ImageObject<'a>>,
}
impl<'a> WebPage<'a> {
    pub fn new(url: &'a str) -> Self {
        WebPage {
            creative_work: CreativeWork::extend("WebPage", Some(url)),
            main_content_of_page: None,
            primary_image_of_page: None,
        }
    }
}

/// http://schema.org/WebSite
#[derive(Serialize, Debug)]
pub struct WebSite<'a> {
    #[serde(flatten)]
    creative_work: CreativeWork<'a>,
}
impl<'a> WebSite<'a> {
    pub fn new() -> Self {
        WebSite {
            creative_work: CreativeWork::extend("WebSite", None),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MediaObject<'a> {
    #[serde(flatten)]
    creative_work: CreativeWork<'a>,

    embed_url: Option<String>,
}
impl<'a> MediaObject<'a> {
    pub fn extend(_type: &'a str, id: Option<&'a str>) -> Self {
        MediaObject {
            creative_work: CreativeWork::extend(_type, id),
            embed_url: None,
        }
    }

    /// Update width and height
    pub fn size(&mut self, width: u16, height: u16) {
        self.creative_work.size(width, height);
    }
}

/// http://schema.org/ImageObject
#[derive(Serialize, Debug)]
pub struct ImageObject<'a> {
    #[serde(flatten)]
    media_object: MediaObject<'a>,

    representative_of_page: Option<bool>,
    caption: Option<&'a str>,
    thumbnail: Option<&'a ImageObject<'a>>,
}

impl<'a> ImageObject<'a> {
    pub fn from_config(image: &'a ImageConfig) -> Self {
        let mut media_object = MediaObject::extend("ImageObject", None);

        media_object.size(image.width, image.height);
        media_object.creative_work.thing.url = Some(&image.url);

        ImageObject {
            media_object,
            representative_of_page: None,
            caption: None,
            thumbnail: None,
        }
    }
}

/// An article, such as a news article or piece of investigative report.
/// Newspapers and magazines have articles of many different types and this is
/// intended to cover them all.
///
/// http://schema.org/Article
///
#[derive(Serialize, Debug)]
pub struct Article<'a> {
    #[serde(flatten)]
    creative_work: CreativeWork<'a>,

    article_body: Option<String>,
    article_section: Option<String>,
}
impl<'a> Article<'a> {
    pub fn extend(_type: &'a str) -> Self {
        Article {
            creative_work: CreativeWork::extend(_type, None),
            article_body: None,
            article_section: None,
        }
    }
}

/// http://schema.org/SocialMediaPosting
#[derive(Serialize, Debug)]
pub struct SocialMediaPosting<'a> {
    #[serde(flatten)]
    article: Article<'a>,

    shared_content: Option<CreativeWork<'a>>,
}
impl<'a> SocialMediaPosting<'a> {
    pub fn extend(_type: &'a str) -> Self {
        SocialMediaPosting {
            article: Article::extend(_type),
            shared_content: None,
        }
    }
}

/// http://schema.org/BlogPosting
#[derive(Serialize, Debug)]
pub struct BlogPosting<'a> {
    #[serde(flatten)]
    social_media_posting: SocialMediaPosting<'a>,
}
impl<'a> BlogPosting<'a> {
    pub fn new() -> Self {
        BlogPosting {
            social_media_posting: SocialMediaPosting::extend("BlogPosting"),
        }
    }
}

/// http://schema.org/Blog
#[derive(Serialize, Debug)]
pub struct Blog<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    blog_post: Vec<BlogPosting<'a>>,
}
impl<'a> Blog<'a> {
    pub fn new() -> Self {
        Blog {
            thing: Thing::extend("Blog", None),
            blog_post: Vec::new(),
        }
    }
}
