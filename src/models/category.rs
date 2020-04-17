use crate::tools::slugify;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CategoryKind {
    Who,
    When,
    Where,
    What,
}

impl Display for CategoryKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub slug: String,
    pub name: String,
    pub kind: CategoryKind,
}

impl Category {
    pub fn new(name: &str, kind: CategoryKind) -> Self {
        Category {
            name: name.to_owned(),
            kind,
            slug: format!("{}/{}", slugify(&kind.to_string()), slugify(&name)),
        }
    }
}
