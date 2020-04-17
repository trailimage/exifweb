//! Context for rendering HTML templates

use crate::{config::BlogConfig, html, Blog, Post};
use chrono::Local;
use yarte::Template;

/// Template rendering helpers
pub struct Helpers {}

impl Helpers {
    pub fn icon(&self, name: &str) -> String {
        html::icon_tag(name)
    }
    pub fn tag_list(&self, list: &Vec<String>) -> String {
        html::photo_tag_list(list)
    }
}

pub struct Features {
    /// If `true` then main navigation elements will scroll with the page,
    /// otherwise they remain fixed in place while the page scrolls
    pub scroll_nav: bool,
    /// Whether to load Facebook scripts
    pub use_facebook: bool,
    /// Timestamp appended to URLs when loading JSON resources to break browser
    /// cache
    pub timestamp: i64, // TODO: re-validate for static gen
}

impl Default for Features {
    fn default() -> Self {
        Features {
            scroll_nav: false,
            use_facebook: true,
            timestamp: Local::now().timestamp(),
        }
    }
}

#[derive(Template)]
#[template(path = "post.hbs")]
pub struct PostContext<'c> {
    pub post: &'c Post,
    pub blog: &'c Blog,
    pub html: Helpers,
    pub config: &'c BlogConfig,
    pub feature: Features,
}

#[derive(Template)]
#[template(path = "sitemap-xml.hbs")]
pub struct SitemapContext<'c> {
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
}
