use super::{creative_work::ImageObject, ObjectOrURL, Thing};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Location<'a> {
    Place(Place<'a>),
    PostalAddress,
    Text(String),
}

#[derive(Serialize, Debug)]
pub struct Place<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ImageObject<'a>>,

    #[serde(rename = "hasMap", skip_serializing_if = "Option::is_none")]
    pub has_map: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<&'a GeoCoordinates<'a>>, // can also be GeoPlace in spec
}
impl<'a> Place<'a> {
    pub fn new(map_url: String, is_root: bool) -> Self {
        Place {
            thing: Thing::extend("Place".to_string(), None, is_root),
            logo: None,
            photo: None,
            has_map: Some(ObjectOrURL::URL(map_url)),
            telephone: None,
            geo: None,
        }
    }
}

/// http://schema.org/GeoCoordinates
#[derive(Serialize, Debug)]
pub struct GeoCoordinates<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    pub latitude: f64,
    pub longitude: f64,

    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
}
impl<'a> GeoCoordinates<'a> {
    pub fn new(lat: f64, lon: f64, is_root: bool) -> Self {
        GeoCoordinates {
            thing: Thing::extend("Place".to_string(), None, is_root),
            elevation: None,
            latitude: lat,
            longitude: lon,
            postal_code: None,
        }
    }
}

/// A set of links that can help a user understand and navigate a website
/// hierarchy
///
/// http://schema.org/breadcrumb
///
#[derive(Serialize, Debug)]
pub struct Breadcrumb<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,
}
impl<'a> Breadcrumb<'a> {
    pub fn new(url: String, name: String) -> Self {
        let mut thing = Thing::extend("Breadcrumb".to_string(), None, false);

        thing.url = Some(url);
        thing.name = Some(name);

        Breadcrumb { thing }
    }
}

// A `BreadcrumbList` is an `ItemList` consisting of a chain of linked Web
// pages, typically described using at least their URL and their name, and
// typically ending with the current page.
//
// The `position` property is used to reconstruct the order of the items in a
// BreadcrumbList The convention is that a breadcrumb list has an
// `itemListOrder` of `ItemListOrderAscending` (lower values listed first),
// and that the first items in this list correspond to the "top" or beginning
// of the breadcrumb trail, e.g. with a site or section homepage. The
// specific values of `position` are not assigned meaning for a
// `BreadcrumbList`, but they should be integers, e.g. beginning with `1` for
// the first item in the list.
//
// @see http://schema.org/BreadcrumbList
