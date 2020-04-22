//! Context and methods for rendering HTML templates

use crate::{
    config::{BlogConfig, CategoryIcon, FacebookConfig, FeaturedPost, PostLog},
    html, json_ld,
    models::{Blog, Category, CategoryKind, PhotoPath, Post},
    tools::{config_regex, path_slice, write_result},
};
use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use regex::Regex;
use std::{fs, path::Path};
use yarte::Template;

// TODO: render photo tag page
// TODO: render map page

/// Render template and write content to `path` file
fn write_page(path: &Path, template: impl Template) {
    write_result(path, || template.call());
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
    org: json_ld::Organization<'a>,
    owner: json_ld::Person<'a>,
    about_page: json_ld::WebPage<'a>,
    home_page: json_ld::WebPage<'a>,
}

impl<'a> Writer<'a> {
    pub fn new(root: &'a Path, config: &'a BlogConfig, blog: &'a Blog) -> Self {
        // sort category kinds to match config.category.display
        // TODO: sort category vector within kind
        let categories: Vec<(CategoryKind, &'a Vec<Category>)> = config
            .category
            .display
            .iter()
            // get enum for name
            .filter_map(|name| CategoryKind::from_str(name))
            // get list of categories for kind
            .map(|kind| (kind, blog.categories.get(&kind)))
            // filter out category kinds that have no categories
            .filter_map(|(kind, cats)| cats.and_then(|c| Some((kind, c))))
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
                categories,
                mode_icons: config_regex(&config.category.what_regex),
                category_icons: &config.category.icon,
            },
            org: json_ld::Organization::from_config(&config.site, false),
            owner: json_ld::Person::from_config(&config.owner, false),
            home_page: json_ld::WebPage::new(
                config.site.url.to_string(),
                false,
            ),
            about_page: json_ld::WebPage::new(
                format!("{}/about", &config.site.url),
                false,
            ),
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
                enable: Enable::default(),
                ctx: &self.context,
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
        let publisher = Some(json_ld::Agent::Organization(&self.org));

        let link_data: serde_json::Result<String> = if home_page {
            let mut blog = json_ld::Blog::new(true);

            blog.creative_work.publisher = publisher;
            blog.creative_work.thing.url =
                Some(self.context.site_url.to_string());
            blog.creative_work.thing.name =
                Some(self.context.site_title.to_string());
            blog.creative_work.thing.description =
                Some(self.context.site_description.to_string());
            blog.creative_work.thing.main_entity_of_page =
                Some(json_ld::ObjectOrURL::Object(&self.home_page));

            serde_json::to_string(&blog)
        } else {
            let mut web_page = json_ld::WebPage::new(
                format!("{}/{}", &self.context.site_url, &category.path),
                true,
            );
            let mut position: usize = 1;

            web_page.creative_work.thing.name =
                Some(self.context.site_title.to_string());
            web_page.creative_work.publisher = publisher;
            web_page.add_breadcrumb(
                self.context.site_url.to_string(),
                "Home".to_string(),
                position,
            );

            // TODO: add remaining breadcrumbs
            // if (category.key.includes("/")) {
            //     // implies category is a subscategory
            //     const rootKey = category.key.split("/")[0];
            //     const rootCategory = blog.categoryWithKey(rootKey);

            //     if (rootCategory !== undefined) {
            //       schema.breadcrumb.push(
            //         breadcrumb(
            //           config.site.url + "/" + rootCategory.key,
            //           rootCategory.title,
            //           position++
            //         )
            //       );
            //     }
            //   }
            //   schema.breadcrumb.push(
            //     breadcrumb(config.site.url + "/" + category.key, category.title, position)
            //   );

            serde_json::to_string(&web_page)
        };

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
                json_ld: link_data.ok(),
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
                // TODO: render JSON-LD for category kind
                json_ld: None,
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
                // TODO: render JSON-LD for category kind
                json_ld: None,
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
        for (slug, tag_photos) in self.context.blog.tags.iter() {
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
}

// TODO: render EXIF data
#[derive(Template)]
#[template(path = "post.hbs")]
struct PostContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub post: &'c Post,
    pub enable: Enable,
}

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
// TODO: render static map with photo locations
#[derive(Template)]
#[template(path = "category.hbs")]
struct CategoryContext<'c> {
    pub ctx: &'c CommonContext<'c>,
    pub category: &'c Category,
    pub enable: Enable,
    pub sub_title: String,
    pub json_ld: Option<String>,
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
