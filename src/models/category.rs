use crate::tools::slugify;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CategoryKind {
    Who,
    When,
    Where,
    What,
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

#[derive(Debug, Clone)]
pub struct Category {
    pub path: String,
    pub name: String,
    pub kind: CategoryKind,
    pub post_paths: Vec<String>,
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
