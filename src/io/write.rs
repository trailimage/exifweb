//! Context and methods for rendering HTML templates

use crate::{
    config::{BlogConfig, CategoryIcon, FacebookConfig, FeaturedPost, PostLog},
    html,
    models::{Blog, Category, CategoryKind, PhotoPath, Post},
    tools::{config_regex, path_slice, rot13, write_result},
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use regex::Regex;
use serde_json;
use std::{fs, path::Path};
use yarte::Template;

// TODO: render map page
// TODO: create 404.html page
// https://help.github.com/en/github/working-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site

/// Render template and write content to `path` file
fn write_page(path: &Path, template: impl Template) {
    write_result(path, || template.call());
}

/// Helper methods for MapBox static maps
struct StaticMap<'a> {
    config: &'a BlogConfig,
}
impl<'a> StaticMap<'a> {
    /// JSON serialized list of photo locations used to generate pins on map
    /// image
    fn locations(&self, post: &Post) -> String {
        serde_json::to_string(&post.photo_locations).unwrap()
    }

    /// MapBox URL to static image source
    fn url(&self, post: &Post, by_cover_photo: bool) -> String {
        if post.photo_locations.is_empty() {
            String::new()
        } else {
            let mut width: u16 = self.config.style.content_width;
            let mut height: u16 = 200;

            if by_cover_photo {
                width = self.width_by_cover(post);
                height = self.height_by_cover(post);
            }

            format!("https://api.mapbox.com/styles/v1/{}/static/-pins-/auto/{}x{}?access_token={}&attribution=false&logo=false",
                self.config.mapbox.style.r#static,
                width,
                height,
                self.config.mapbox.access_token)
        }
    }

    /// Height of map image next to cover photo computed so map matches height
    /// of landscape images but is less than height of portrait images
    fn height_by_cover(&self, post: &Post) -> u16 {
        let max_height = self.config.style.inline_map_height;

        post.cover_photo().map_or(max_height, |p| {
            if p.size.small.height > p.size.small.width {
                // limit height next to portrait images
                max_height
            } else {
                p.size.small.height
            }
        })
    }

    /// Width of map image next to cover computed so that side-by-side they fill
    /// the `content_width`
    fn width_by_cover(&self, post: &Post) -> u16 {
        let width = self.config.style.content_width;

        post.cover_photo()
            .map_or(width, |p| width - p.size.small.width)
    }
}

/// Context available to each render template
struct CommonContext<'a> {
    pub blog: &'a Blog,
    pub categories: Vec<(CategoryKind, &'a Vec<Category>)>,
    pub site_url: &'a str,
    pub site_title: &'a str,
    pub site_description: &'a str,
    pub repo_url: &'a str,
    pub author_name: &'a str,
    pub featured_post: &'a Option<FeaturedPost>,
    pub post_alias: &'a str,
    pub facebook: &'a FacebookConfig,
    pub content_width: u16,
    pub contact_link: String,

    mode_icons: HashMap<String, Regex>,
    category_icons: &'a CategoryIcon,
}

impl<'a> CommonContext<'a> {
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
    pub fn fraction(&self, number: &str) -> String {
        html::fraction(number)
    }
    pub fn plural(&self, word: &str, count: usize) -> String {
        html::plural(word, count)
    }
    pub fn say_number(&self, number: usize) -> String {
        html::say_number(number)
    }
    pub fn list_label<T>(&self, word: &str, list: &Vec<T>) -> String {
        html::list_label(word, list)
    }
}

/// Methods to render and write standard web pages with loaded configuration and
/// models
pub struct Writer<'a> {
    root: &'a Path,
    context: CommonContext<'a>,
    config: &'a BlogConfig,
}

impl<'a> Writer<'a> {
    pub fn new(root: &'a Path, config: &'a BlogConfig, blog: &'a Blog) -> Self {
        // sort category kinds to match config.category.display
        let category_kinds: Vec<(CategoryKind, &'a Vec<Category>)> = config
            .category
            .display
            // iterate over category kinds in configured order
            .iter()
            // get enum for name
            .filter_map(|name| CategoryKind::from_str(name))
            // get list of categories for kind
            .map(|kind| (kind, blog.categories.get(&kind)))
            // filter out kinds that have no categories
            .filter_map(|(kind, categories)| categories.map(|c| (kind, c)))
            .collect();

        Writer {
            root,
            //   config: &config,
            context: CommonContext {
                blog,
                site_url: &config.site.url,
                site_title: &config.site.title,
                repo_url: &config.repo_url,
                author_name: &config.author_name,
                site_description: &config.site.description,
                featured_post: &config.featured_post,
                post_alias: &config.site.post_alias,
                facebook: &config.facebook,
                categories: category_kinds,
                mode_icons: config_regex(&config.category.what_regex),
                category_icons: &config.category.icon,
                content_width: config.style.content_width,
                contact_link: config
                    .owner
                    .email
                    .as_ref()
                    .map_or(String::new(), |e| {
                        rot13(&format!("<a href=\"mailto:{}\">Contact</a>", e))
                    }),
            },
            config,
        }
    }

    /// Render template and write content to "index.html" in `folder`
    fn default_page(&self, folder: &str, template: impl Template) {
        let path = self.root.join(folder);

        if !path.is_dir() {
            println!(
                "   Attempting to create directory {}",
                path_slice(&path, 2)
            );
            // ignore error here since it will be caught in the next step
            fs::create_dir(&path).unwrap_or(());
        }

        write_page(&path.join("index.html"), template)
    }

    pub fn posts(&self) {
        for (_, p) in &self.context.blog.posts {
            if p.needs_render {
                self.post(&p);
                // GOAL: spawn thread to write log
                PostLog::write(self.root, &p);
            }
        }
    }

    fn post(&self, post: &Post) {
        let mut title = post.title.clone();
        let mut sub_title = String::new();

        if let Some(series) = &post.series {
            sub_title = title;
            title = series.title.clone();
        }

        self.default_page(
            &post.path,
            PostContext {
                post,
                enable: Enable::all(),
                ctx: &self.context,
                json_ld: Some(post.json_ld(&self.config).to_string()),
                title,
                sub_title,
                map_image: StaticMap {
                    config: self.config,
                },
            },
        );
    }

    pub fn categories(&self) {
        for (kind, list) in &self.context.blog.categories {
            self.category_kind(kind, list);
            for c in list {
                self.category(&c, &c.path, false);
            }
        }
    }

    fn category(&self, category: &Category, path: &str, home_page: bool) {
        self.default_page(
            path,
            CategoryContext {
                ctx: &self.context,
                category,
                enable: Enable::none(),
                sub_title: html::list_label(
                    self.context.post_alias,
                    &category.post_paths,
                ),
                json_ld: Some(
                    category.json_ld(&self.config, home_page).to_string(),
                ),
                map_image: StaticMap {
                    config: self.config,
                },
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
                ctx: &self.context,
                kind: category_kind,
                categories,
                enable: Enable::none(),
                sub_title: html::list_label("Category", &categories),
                json_ld: Some(category_kind.json_ld(self.config).to_string()),
            },
        );
    }

    pub fn home_page(&self) {
        // home page is the latest year category
        if let Some(category) = self
            .context
            .blog
            .categories
            .get(&CategoryKind::When)
            .and_then(|list| list.first())
        {
            self.category(category, "", true);
        }
    }

    pub fn about_page(&self) {
        self.default_page(
            "about",
            AboutContext {
                ctx: &self.context,
                enable: Enable::new(true, false),
                // TODO: render JSON-LD for about page
                json_ld: None,
                title: format!("About {}", self.context.site_title),
            },
        );
    }

    pub fn category_menu(&self) {
        self.default_page(
            "category-menu",
            CategoryMenuContext { ctx: &self.context },
        );
    }

    pub fn mobile_menu(&self) {
        self.default_page(
            "mobile-menu",
            MobileMenuContext { ctx: &self.context },
        );
    }

    pub fn photo_tags(&self) {
        for (slug, tag_photos) in self
            .context
            .blog
            .tags
            .iter()
            .filter(|(_, tag_photos)| tag_photos.changed)
        {
            // only render tags that have changes
            self.default_page(
                &format!("photo-tag/{}", slug),
                PhotoTagContext {
                    ctx: &self.context,
                    enable: Enable::none(),
                    slug,
                    name: &tag_photos.name,
                    photos: &tag_photos.photos,
                    sub_title: html::list_label("Photo", &tag_photos.photos),
                },
            );
        }
    }

    pub fn sitemap(&self) {
        write_page(
            &self.root.join("sitemap.xml"),
            SitemapContext { ctx: &self.context },
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

    fn all() -> Self {
        Enable::new(true, true)
    }
}

// TODO: render EXIF data
#[derive(Template)]
#[template(path = "post.hbs")]
struct PostContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub post: &'c Post,
    pub enable: Enable,
    pub json_ld: Option<String>,
    pub title: String,
    pub sub_title: String,
    pub map_image: StaticMap<'c>,
}

// TODO: update template with actual image thumbnails
#[derive(Template)]
#[template(path = "photo_tag.hbs")]
struct PhotoTagContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub enable: Enable,
    pub slug: &'c str,
    pub name: &'c str,
    pub photos: &'c Vec<PhotoPath>,
    pub sub_title: String,
}

// TODO: re-use partials/category for post category list
#[derive(Template)]
#[template(path = "category.hbs")]
struct CategoryContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub category: &'c Category,
    pub enable: Enable,
    pub sub_title: String,
    pub json_ld: Option<String>,
    pub map_image: StaticMap<'c>,
}

#[derive(Template)]
#[template(path = "category_kind.hbs")]
struct CategoryKindContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub categories: &'c Vec<Category>,
    pub enable: Enable,
    pub kind: &'c CategoryKind,
    pub sub_title: String,
    pub json_ld: Option<String>,
}

#[derive(Template)]
#[template(path = "about.hbs")]
struct AboutContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub enable: Enable,
    pub json_ld: Option<String>,
    pub title: String,
}

#[derive(Template)]
#[template(path = "category_menu.hbs")]
struct CategoryMenuContext<'c> {
    pub ctx: &'c CommonContext<'c>,
}

#[derive(Template)]
#[template(path = "mobile_menu.hbs")]
struct MobileMenuContext<'c> {
    pub ctx: &'c CommonContext<'c>,
}

#[derive(Template)]
#[template(path = "sitemap_xml.hbs")]
struct SitemapContext<'c> {
    pub ctx: &'c CommonContext<'c>,
}
