use crate::tools::slugify;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

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

impl Hash for CategoryKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Display for CategoryKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
}
