//! Context for rendering HTML templates

use crate::{config::BlogConfig, html, Blog, Post};
use yarte::Template;

pub struct Helpers {}

impl Helpers {
    pub fn icon(&self, name: &str) -> String {
        html::icon_tag(name)
    }
}

#[derive(Template)]
#[template(path = "post.hbs")]
pub struct PostContext<'c> {
    pub post: &'c Post,
    pub blog: &'c Blog,
    pub html: Helpers,
    pub config: &'c BlogConfig,
}

#[derive(Template)]
#[template(path = "sitemap-xml.hbs")]
pub struct SitemapContext<'c> {
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
}
