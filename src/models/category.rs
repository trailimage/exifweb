use crate::{config::BlogConfig, json_ld, tools::slugify};
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

/// The kind of category. Each category belongs to a kind.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CategoryKind {
    Who,
    What,
    When,
    Where,
}

impl CategoryKind {
    /// Category kind that matches name
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "who" => Some(CategoryKind::Who),
            "what" => Some(CategoryKind::What),
            "when" => Some(CategoryKind::When),
            "where" => Some(CategoryKind::Where),
            _ => None,
        }
    }
}

impl CategoryKind {
    pub fn json_ld(&self, config: &BlogConfig) -> serde_json::Value {
        let name = self.to_string();
        let path = slugify(&name);
        let breadcrumbs: Vec<serde_json::Value> = vec![
            json_ld::breadcrumb(config, "", "Home", 1),
            json_ld::breadcrumb(config, &path, &name, 2),
        ];

        serde_json::json!({
            "@type": "WebPage",
            "@context": json_ld::CONTEXT,
            "url": json_ld::full_url(config, &path),
            "name": name,
            "publisher": json_ld::organization(config),
            "breadcrumb": breadcrumbs
        })
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for CategoryKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Display for CategoryKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Category {
    pub path: String,
    pub name: String,
    pub kind: CategoryKind,
    pub post_paths: Vec<String>,
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.kind == other.kind
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind {
            // reverse sort years so newest is first
            CategoryKind::When => other.name.cmp(&self.name),
            _ => self.name.cmp(&other.name),
        }
    }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Category {
    pub fn new(name: &str, kind: CategoryKind) -> Self {
        Category {
            name: name.to_owned(),
            kind,
            path: format!("{}/{}", slugify(&kind.to_string()), slugify(&name)),
            post_paths: Vec::new(),
        }
    }
    pub fn json_ld(
        &self,
        config: &BlogConfig,
        home_page: bool,
    ) -> serde_json::Value {
        let site = &config.site;

        if home_page {
            serde_json::json!({
                "@type": "Blog",
                "@context": json_ld::CONTEXT,
                "url": site.url,
                "name": site.title,
                "author": json_ld::owner(config),
                "description": site.description,
                "mainEntityOfPage": json_ld::web_page(config, ""),
                "publisher": json_ld::organization(config)
            })
        } else {
            let breadcrumbs: Vec<serde_json::Value> = vec![
                json_ld::breadcrumb(config, "", "Home", 1),
                json_ld::breadcrumb(
                    config,
                    &slugify(&self.kind.to_string()),
                    &self.kind.to_string(),
                    2,
                ),
                json_ld::breadcrumb(config, &self.path, &self.name, 3),
            ];

            serde_json::json!({
                "@type": "WebPage",
                "@context": json_ld::CONTEXT,
                "url": json_ld::full_url(config, &self.path),
                "name": self.name,
                "publisher": json_ld::organization(config),
                "breadcrumb": breadcrumbs
            })
        }
    }
}
