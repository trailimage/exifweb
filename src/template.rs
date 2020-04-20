//! Context and methods for rendering HTML templates

use crate::{
    config::{BlogConfig, CategoryIcon, PostLog},
    html,
    models::{Blog, Category, CategoryKind, Post},
    tools::{config_regex, folder_name, write_result},
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use regex::Regex;
use std::{fs, path::Path};
use yarte::Template;

// TODO: render photo tag page
// TODO: render map page

/// Template rendering helpers
pub struct Helpers<'a> {
    mode_icons: HashMap<String, Regex>,
    category_icons: &'a CategoryIcon,
}

impl<'a> Helpers<'a> {
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
    pub fn category_icon(&self, kind: &CategoryKind) -> String {
        html::category_icon(kind, &self.category_icons)
    }
    pub fn plural(&self, count: usize) -> &str {
        if count == 1 {
            ""
        } else {
            "s"
        }
    }
    pub fn fraction(&self, number: &str) -> String {
        html::fraction(number)
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
    helpers: Helpers<'a>,
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
                category_icons: &config.category.icon,
            },
        }
    }

    /// Render template and write content to "index.html" in `folder`
    fn default_page(&self, folder: &str, template: impl Template) {
        let path = self.root.join(folder);

        if !path.is_dir() {
            println!(
                "   Attempting to create directory {}",
                folder_name(&path)
            );
            // ignore error here since it will be caught in the next step
            fs::create_dir(&path).unwrap_or(());
        }

        write_page(&path.join("index.html"), template)
    }

    pub fn posts(&self) {
        for (_, p) in &self.blog.posts {
            if p.needs_render {
                self.post(&p);
                // TODO: spawn thread to write log
                PostLog::write(self.root, &p);
            }
        }
    }

    fn post(&self, post: &Post) {
        self.default_page(
            &post.path,
            PostContext {
                post,
                blog: &self.blog,
                config: &self.config,
                html: &self.helpers,
                enable: Enable::default(),
            },
        );
    }

    pub fn categories(&self) {
        for (kind, list) in &self.blog.categories {
            self.category_kind(kind, list);
            for c in list {
                self.category(&c, &c.path);
            }
        }
    }

    fn category(&self, category: &Category, path: &str) {
        let post_count = category.post_paths.len();

        self.default_page(
            path,
            CategoryContext {
                category,
                blog: &self.blog,
                config: &self.config,
                html: &self.helpers,
                enable: Enable::none(),
                sub_title: format!(
                    "{} {}{}",
                    html::say_number(post_count),
                    self.config.site.post_alias,
                    self.helpers.plural(post_count)
                ),
            },
        );
    }

    fn category_kind(
        &self,
        category_kind: &CategoryKind,
        categories: &Vec<Category>,
    ) {
        self.default_page(
            category_kind.to_string().to_lowercase().as_str(),
            CategoryKindContext {
                kind: category_kind,
                categories,
                config: &self.config,
                html: &self.helpers,
                enable: Enable::none(),
                sub_title: format!(
                    "{} {}",
                    html::say_number(categories.len()),
                    if categories.len() == 1 {
                        "Category"
                    } else {
                        "Categories"
                    }
                ),
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
            self.category(category, "");
        }
    }

    pub fn about_page(&self) {
        self.default_page(
            "about",
            AboutContext {
                config: &self.config,
                html: &self.helpers,
                enable: Enable::new(true, false),
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
struct Enable {
    /// If `true` then main navigation elements will scroll with the page,
    /// otherwise they remain fixed in place while the page scrolls
    pub scroll_nav: bool,
    /// Whether to load Facebook scripts
    pub facebook: bool,
}

impl Default for Enable {
    fn default() -> Self {
        Enable {
            scroll_nav: false,
            facebook: true,
        }
    }
}

impl Enable {
    fn new(scroll_nav: bool, facebook: bool) -> Self {
        Enable {
            scroll_nav,
            facebook,
        }
    }

    fn none() -> Self {
        Enable::new(false, false)
    }
}

// TODO: render EXIF data
#[derive(Template)]
#[template(path = "post.hbs")]
struct PostContext<'c> {
    pub post: &'c Post,
    pub blog: &'c Blog,
    pub html: &'c Helpers<'c>,
    pub config: &'c BlogConfig,
    pub enable: Enable,
}

// TODO: re-use partials/category for post category list
// TODO: render static map with photo locations
#[derive(Template)]
#[template(path = "category.hbs")]
struct CategoryContext<'c> {
    pub html: &'c Helpers<'c>,
    pub category: &'c Category,
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
    pub enable: Enable,
    pub sub_title: String,
}

#[derive(Template)]
#[template(path = "category_kind.hbs")]
struct CategoryKindContext<'c> {
    pub html: &'c Helpers<'c>,
    pub config: &'c BlogConfig,
    pub categories: &'c Vec<Category>,
    pub enable: Enable,
    pub kind: &'c CategoryKind,
    pub sub_title: String,
}

#[derive(Template)]
#[template(path = "about.hbs")]
struct AboutContext<'c> {
    pub config: &'c BlogConfig,
    pub enable: Enable,
    pub html: &'c Helpers<'c>,
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
    pub html: &'c Helpers<'c>,
}

#[derive(Template)]
#[template(path = "sitemap_xml.hbs")]
struct SitemapContext<'c> {
    pub blog: &'c Blog,
    pub config: &'c BlogConfig,
}
