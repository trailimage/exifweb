//! Context and methods for rendering HTML templates

use crate::{config::BlogConfig, html, tools::write_result, Blog, Post};
use chrono::{DateTime, FixedOffset, Local};
use std::path::Path;
use yarte::Template;

// TODO: render category page
// TODO: render category kind page
// TODO: render photo tag page

/// Template rendering helpers
struct Helpers {}

impl Helpers {
    pub fn icon(&self, name: &str) -> String {
        html::icon_tag(name)
    }
    pub fn tag_list(&self, list: &Vec<String>) -> String {
        html::photo_tag_list(list)
    }
    pub fn date(&self, d: DateTime<FixedOffset>) -> String {
        html::date_string(d)
    }
}

/// Page features
struct Features {
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

impl Features {
    fn new(scroll_nav: bool, use_facebook: bool) -> Self {
        Features {
            scroll_nav,
            use_facebook,
            ..Features::default()
        }
    }
}

#[derive(Template)]
#[template(path = "post.hbs")]
struct PostContext<'c> {
    pub post: &'c Post,
    pub blog: &'c Blog,
    pub html: Helpers,
    pub config: &'c BlogConfig,
    pub feature: Features,
}

#[derive(Template)]
#[template(path = "about.hbs")]
struct AboutContext<'c> {
    pub config: &'c BlogConfig,
    pub feature: Features,
    pub html: Helpers,
}

#[derive(Template)]
#[template(path = "sitemap-xml.hbs")]
struct SitemapContext<'c> {
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
}

pub fn write_post(root: &Path, config: &BlogConfig, blog: &Blog, post: &Post) {
    write_page(
        &root.join(&post.path).join("index.html"),
        PostContext {
            post,
            blog,
            config,
            html: Helpers {},
            feature: Features::default(),
        },
    );
}

pub fn write_about(root: &Path, config: &BlogConfig) {
    write_page(
        &root.join("about").join("index.html"),
        AboutContext {
            config,
            html: Helpers {},
            feature: Features::new(true, false),
        },
    );
}

pub fn write_sitemap(root: &Path, config: &BlogConfig, blog: &Blog) {
    write_page(&root.join("sitemap.xml"), SitemapContext { blog, config });
}

fn write_page(path: &Path, template: impl Template) {
    write_result(path, || template.call());
}
