//! JSON-LD helper methods

use crate::config::BlogConfig;
use serde_json::{json, Value};

pub static CONTEXT: &str = "http://schema.org";

pub fn full_url(config: &BlogConfig, path: &str) -> String {
    format!("{}/{}", config.site.url, path)
}

pub fn owner(config: &BlogConfig) -> Value {
    let photo = config
        .owner
        .image
        .as_ref()
        .map(|p| image(p.width, p.height, full_url(config, &p.url)));

    json!({
        "@type": "Person",
        "name": config.owner.name,
        "url": full_url(config, "about"),
        "image": photo,
        "sameAs": config.owner.urls
    })
}

/// http://schema.org/Organization
pub fn organization(config: &BlogConfig) -> Value {
    let logo = &config.site.logo;
    let logo = image(logo.width, logo.height, full_url(config, &logo.url));

    json!({
        "@type": "Organization",
        "name": config.site.title,
        "logo": logo
    })
}

pub fn image(width: u16, height: u16, url: String) -> Value {
    json!({
        "@type": "ImageObject",
        "width": width,
        "height": height,
        "url": url
    })
}

/// http://schema.org/WebPage
pub fn web_page(config: &BlogConfig, path: &str) -> Value {
    json!({
        "@type": "WebPage",
        "id": full_url(config, path)
    })
}

/// A `BreadcrumbList` is an `ItemList` consisting of a chain of linked Web
/// pages, typically described using at least their URL and their name, and
/// typically ending with the current page.
///
/// The `position` property is used to reconstruct the order of the items in a
/// BreadcrumbList The convention is that a breadcrumb list has an
/// `itemListOrder` of `ItemListOrderAscending` (lower values listed first),
/// and that the first items in this list correspond to the "top" or beginning
/// of the breadcrumb trail, e.g. with a site or section homepage. The
/// specific values of `position` are not assigned meaning for a
/// `BreadcrumbList`, but they should be integers, e.g. beginning with `1` for
/// the first item in the list.
///
/// http://schema.org/BreadcrumbList
///
pub fn breadcrumb(
    config: &BlogConfig,
    path: &str,
    name: &str,
    position: usize,
) -> Value {
    json!({
        "@type": "ListItem",
        "item": {
            "@type": "Breadcrumb",
            "url": full_url(config, path),
            "name": name
        },
        "position": position
    })
}
