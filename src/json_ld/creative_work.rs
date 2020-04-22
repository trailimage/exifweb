use super::{
    agent::{Agent, Person},
    location::Breadcrumb,
    ListItem, Thing,
};
use crate::config::ImageConfig;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreativeWork<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<&'a Person<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<&'a str>,

    #[serde(rename = "thumbnailUrl", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Agent<'a>>,
}
impl<'a> CreativeWork<'a> {
    pub fn extend(
        type_name: String,
        id: Option<String>,
        is_root: bool,
    ) -> Self {
        CreativeWork {
            thing: Thing::extend(type_name, id, is_root),
            author: None,
            height: None,
            width: None,
            keywords: None,
            thumbnail_url: None,
            publisher: None,
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
    pub fn new(url: String, is_root: bool) -> Self {
        Photograph {
            creative_work: CreativeWork::extend(
                "Photograph".to_string(),
                Some(url),
                is_root,
            ),
        }
    }
}

/// http://schema.org/WebPage
#[derive(Serialize, Debug)]
pub struct WebPage<'a> {
    #[serde(flatten)]
    pub creative_work: CreativeWork<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<Vec<ListItem<'a, Breadcrumb<'a>>>>,

    #[serde(
        rename = "mainContentOfPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_content: Option<&'a str>,

    #[serde(
        rename = "primaryImageOfPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image: Option<ImageObject<'a>>,
}
impl<'a> WebPage<'a> {
    pub fn new(url: String, is_root: bool) -> Self {
        WebPage {
            creative_work: CreativeWork::extend(
                "WebPage".to_string(),
                Some(url),
                is_root,
            ),
            main_content: None,
            primary_image: None,
            breadcrumb: None,
        }
    }

    pub fn add_breadcrumb(
        &mut self,
        url: String,
        name: String,
        position: usize,
    ) {
        let b = Breadcrumb::new(url, name);

        self.breadcrumb
            .get_or_insert(Vec::new())
            .push(ListItem::new(String::new(), b, position));
    }
}

/// http://schema.org/WebSite
#[derive(Serialize, Debug)]
pub struct WebSite<'a> {
    #[serde(flatten)]
    pub creative_work: CreativeWork<'a>,
}
impl<'a> WebSite<'a> {
    pub fn new(is_root: bool) -> Self {
        WebSite {
            creative_work: CreativeWork::extend(
                "WebSite".to_string(),
                None,
                is_root,
            ),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MediaObject<'a> {
    #[serde(flatten)]
    pub creative_work: CreativeWork<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
}
impl<'a> MediaObject<'a> {
    pub fn extend(
        type_name: String,
        id: Option<String>,
        is_root: bool,
    ) -> Self {
        MediaObject {
            creative_work: CreativeWork::extend(type_name, id, is_root),
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
    pub media_object: MediaObject<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_of_page: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<&'a ImageObject<'a>>,
}

impl<'a> ImageObject<'a> {
    pub fn from_config(image: &'a ImageConfig, is_root: bool) -> Self {
        let mut media_object =
            MediaObject::extend("ImageObject".to_string(), None, is_root);

        media_object.size(image.width, image.height);
        media_object.creative_work.thing.url = Some(image.url.to_string());

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
    pub creative_work: CreativeWork<'a>,

    #[serde(rename = "articleBody", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(
        rename = "articleSection",
        skip_serializing_if = "Option::is_none"
    )]
    pub section: Option<String>,
}
impl<'a> Article<'a> {
    pub fn extend(type_name: String, is_root: bool) -> Self {
        Article {
            creative_work: CreativeWork::extend(type_name, None, is_root),
            body: None,
            section: None,
        }
    }
}

/// http://schema.org/SocialMediaPosting
#[derive(Serialize, Debug)]
pub struct SocialMediaPosting<'a> {
    #[serde(flatten)]
    pub article: Article<'a>,

    #[serde(rename = "sharedContent", skip_serializing_if = "Option::is_none")]
    pub shared_content: Option<CreativeWork<'a>>,
}
impl<'a> SocialMediaPosting<'a> {
    pub fn extend(type_name: String, is_root: bool) -> Self {
        SocialMediaPosting {
            article: Article::extend(type_name, is_root),
            shared_content: None,
        }
    }
}

/// http://schema.org/BlogPosting
#[derive(Serialize, Debug)]
pub struct BlogPosting<'a> {
    #[serde(flatten)]
    pub social_media_posting: SocialMediaPosting<'a>,
}
impl<'a> BlogPosting<'a> {
    pub fn new(is_root: bool) -> Self {
        BlogPosting {
            social_media_posting: SocialMediaPosting::extend(
                "BlogPosting".to_string(),
                is_root,
            ),
        }
    }
}

/// http://schema.org/Blog
#[derive(Serialize, Debug)]
pub struct Blog<'a> {
    #[serde(flatten)]
    pub creative_work: CreativeWork<'a>,

    #[serde(rename = "blogPost")]
    pub blog_post: Vec<BlogPosting<'a>>,
}
impl<'a> Blog<'a> {
    pub fn new(is_root: bool) -> Self {
        Blog {
            creative_work: CreativeWork::extend(
                "Blog".to_string(),
                None,
                is_root,
            ),
            blog_post: Vec::new(),
        }
    }
}

// {
//     "@id": "when/2019",
//     "@type": "WebPage",
//     "@context": "http://schema.org",
//     "name": "2019",
//     "publisher": {
//       "name": "Trail Image",
//       "logo": {
//         "url": "http://www.trailimage.com/img/logo-title.png",
//         "width": 308,
//         "height": 60,
//         "@type": "ImageObject"
//       },
//       "@type": "Organization"
//     },
//     "breadcrumb": [
//       {
//         "item": {
//           "id": "http://www.trailimage.com",
//           "name": "Home"
//         },
//         "position": 1,
//         "@type": "Breadcrumb"
//       },
//       {
//         "item": {
//           "id": "http://www.trailimage.com/when",
//           "name": "When"
//         },
//         "position": 2,
//         "@type": "Breadcrumb"
//       },
//       {
//         "item": {
//           "id": "http://www.trailimage.com/when/2019",
//           "name": "2019"
//         },
//         "position": 3,
//         "@type": "Breadcrumb"
//       }
//     ]
//   }
