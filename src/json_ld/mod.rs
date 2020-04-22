mod action;
mod agent;
mod creative_work;
mod location;

pub use agent::{Agent, Organization, Person};
pub use creative_work::{Blog, BlogPosting, ImageObject, WebPage};
use serde::Serialize;

const DEFAULT_CONTEXT: &str = "http://schema.org";

pub enum Value {
    Property(PropertyValue),
    Text(String),
}

/// Image object or URL
///
/// According to the JSON-LD specification, the `URL` may itself be a `Thing`
/// but that option is ignored here.
#[derive(Serialize, Debug)]
pub enum ObjectOrURL<'a, T> {
    Object(&'a T),
    URL(String),
}

/// Common JSON-LD fields
///
/// `E` is the type of `main_entity_of_page` if not a URL
///
#[derive(Serialize, Debug)]
pub struct Thing<'a> {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<&'a str>,

    #[serde(rename = "@type")]
    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "sameAs", skip_serializing_if = "Option::is_none")]
    pub same_as: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    /// Indicates a page (or other `CreativeWork`) for which this thing is the
    /// main entity being described
    #[serde(
        rename = "mainEntityOfPage",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_entity_of_page: Option<ObjectOrURL<'a, WebPage<'a>>>,
}
impl<'a> Thing<'a> {
    /// - `is_root` Whther to write fields, like `context`, that are only
    ///    pertinent for the root element
    pub fn extend(
        type_name: String,
        id: Option<String>,
        is_root: bool,
    ) -> Thing<'a> {
        Thing {
            id,
            r#type: type_name,
            context: if is_root { Some(DEFAULT_CONTEXT) } else { None },
            name: None,
            description: None,
            same_as: None,
            url: None,
            image: None,
            main_entity_of_page: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ListItem<'a, T> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    pub item: T,

    #[serde(rename = "nextItem", skip_serializing_if = "Option::is_none")]
    pub next_item: Option<&'a ListItem<'a, T>>,

    pub position: usize,

    #[serde(rename = "previousItem", skip_serializing_if = "Option::is_none")]
    pub previous_item: Option<&'a ListItem<'a, T>>,
}
impl<'a, T> ListItem<'a, T> {
    pub fn new(type_name: String, item: T, position: usize) -> Self {
        ListItem {
            item,
            thing: Thing::extend(type_name, None, false),
            previous_item: None,
            position,
            next_item: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PropertyValue {
    max_value: Option<usize>,
    min_value: Option<usize>,
    property_id: Option<String>,
}
