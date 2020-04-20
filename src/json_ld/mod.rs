mod action;
mod agent;
mod creative_work;
mod location;

use serde::Serialize;

pub use agent::{Agent, Organization, Person};
pub use creative_work::{Blog, BlogPosting};
pub use location::{Location, Place};

// TODO: create JSON-LD models

pub enum Value {
    Property(PropertyValue),
    Text(String),
}

#[derive(Serialize, Debug)]
struct Thing {
    #[serde(rename = "@id")]
    id: Option<String>,

    #[serde(rename = "@type")]
    r#type: Option<String>,

    #[serde(rename = "@context")]
    context: Option<String>,

    name: Option<String>,
    description: Option<String>,
    same_as: Option<String>,
    url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct PropertyValue {
    max_value: Option<usize>,
    min_value: Option<usize>,
    property_id: Option<String>,
}
