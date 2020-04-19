//! Context and methods for rendering HTML templates

use crate::{
    config::BlogConfig,
    html,
    models::{Blog, Category, CategoryKind, Post},
    tools::{config_regex, final_path_name, write_result},
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use regex::Regex;
use std::{fs, path::Path};
use yarte::Template;

// TODO: render ruminations page
// TODO: render category kind page
// TODO: render photo tag page
// TODO: render map page

/// Template rendering helpers
pub struct Helpers {
    mode_icons: HashMap<String, Regex>,
}

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
    pub fn travel_icon(&self, categories: &Vec<Category>) -> String {
        match html::travel_mode_icon(categories, &self.mode_icons) {
            Some(icon) => icon,
            None => String::new(),
        }
    }
}

/// Render template and write content to `path` file
fn write_page(path: &Path, template: impl Template) {
    write_result(path, || template.call());
}

/// Methods to render and write standard web pages with loaded configuration and
/// models
pub struct Writer<'a> {
    config: &'a BlogConfig,
    blog: &'a Blog,
    helpers: Helpers,
    root: &'a Path,
}

impl<'a> Writer<'a> {
    pub fn new(root: &'a Path, config: &'a BlogConfig, blog: &'a Blog) -> Self {
        Writer {
            root,
            blog,
            config,
            helpers: Helpers {
                mode_icons: config_regex(&config.category.what_regex),
            },
        }
    }

    /// Render template and write content to "index.html" in `folder`
    fn default_page(&self, folder: &str, template: impl Template) {
        let path = self.root.join(folder);

        if !path.is_dir() {
            println!(
                "   Attempting to create directory {}",
                final_path_name(&path)
            );
            // ignore error here since it will be caught in the next step
            fs::create_dir(&path).unwrap_or(());
        }

        write_page(&path.join("index.html"), template)
    }

    pub fn post(&self, post: &Post) {
        self.default_page(
            &post.path,
            PostContext {
                post,
                blog: &self.blog,
                config: &self.config,
                html: &self.helpers,
                feature: Features::default(),
            },
        );
    }

    pub fn category(&self, category: &Category) {
        self.default_page(
            &category.path,
            CategoryContext {
                category,
                blog: &self.blog,
                config: &self.config,
                html: &self.helpers,
                feature: Features::default(),
            },
        );
    }

    pub fn home_page(&self) {
        // home page is the latest year category
        if let Some(category) = self
            .blog
            .categories
            .get(&CategoryKind::When)
            .and_then(|list| list.first())
        {
            self.default_page(
                "",
                CategoryContext {
                    category,
                    blog: &self.blog,
                    config: &self.config,
                    html: &self.helpers,
                    feature: Features::default(),
                },
            );
        }
    }

    pub fn about_page(&self) {
        self.default_page(
            "about",
            AboutContext {
                config: &self.config,
                html: &self.helpers,
                feature: Features::new(true, false),
            },
        );
    }

    pub fn category_menu(&self) {
        self.default_page(
            "category-menu",
            CategoryMenuContext { blog: &self.blog },
        );
    }

    pub fn mobile_menu(&self) {
        self.default_page(
            "mobile-menu",
            MobileMenuContext {
                blog: &self.blog,
                html: &self.helpers,
            },
        );
    }

    pub fn sitemap(&self) {
        write_page(
            &self.root.join("sitemap.xml"),
            SitemapContext {
                blog: &self.blog,
                config: &self.config,
            },
        );
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
    pub timestamp: u8, // TODO: remove from JavaScript
}

impl Default for Features {
    fn default() -> Self {
        Features {
            scroll_nav: false,
            use_facebook: true,
            timestamp: 0,
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

// TODO: render EXIF data
#[derive(Template)]
#[template(path = "post.hbs")]
struct PostContext<'c> {
    pub post: &'c Post,
    pub blog: &'c Blog,
    pub html: &'c Helpers,
    pub config: &'c BlogConfig,
    pub feature: Features,
}

// TODO: re-use partials/category for post category list
// TODO: add post count subtitle
// TODO: render static map with photo locations
#[derive(Template)]
#[template(path = "category.hbs")]
struct CategoryContext<'c> {
    pub html: &'c Helpers,
    pub category: &'c Category,
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
    pub feature: Features,
}

#[derive(Template)]
#[template(path = "about.hbs")]
struct AboutContext<'c> {
    pub config: &'c BlogConfig,
    pub feature: Features,
    pub html: &'c Helpers,
}

// TODO: sort when, who, what, where
#[derive(Template)]
#[template(path = "category_menu.hbs")]
struct CategoryMenuContext<'c> {
    pub blog: &'c Blog,
}

#[derive(Template)]
#[template(path = "mobile_menu.hbs")]
struct MobileMenuContext<'c> {
    pub blog: &'c Blog,
    pub html: &'c Helpers,
}

#[derive(Template)]
#[template(path = "sitemap_xml.hbs")]
struct SitemapContext<'c> {
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
}
