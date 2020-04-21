mod action;
mod agent;
mod creative_work;
mod location;

use agent::Person;
use creative_work::{CreativeWork, ImageObject};
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
    URL(&'a str),
}

/// Common JSON-LD fields
///
/// `E` is the type of `main_entity_of_page` if not a URL
///
#[derive(Serialize, Debug)]
struct Thing<'a, E> {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,

    #[serde(rename = "@type")]
    r#type: &'a str,

    #[serde(rename = "@context")]
    context: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    same_as: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    main_entity_of_page: Option<ObjectOrURL<'a, E>>,
}

impl<'a, E> Thing<'a, E> {
    pub fn extend(_type: &'a str, id: Option<&'a str>) -> Thing<'a, E> {
        Thing {
            id,
            r#type: _type,
            context: DEFAULT_CONTEXT,
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
pub struct PropertyValue {
    max_value: Option<usize>,
    min_value: Option<usize>,
    property_id: Option<String>,
}
