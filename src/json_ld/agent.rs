use super::{
    action::Action,
    creative_work::{ImageObject, WebPage},
    location::Location,
    ObjectOrURL, Thing,
};
use crate::config::{OwnerConfig, SiteConfig};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Agent<'a> {
    Person(&'a Person<'a>),
    Organization(&'a Organization<'a>),
}

/// http://schema.org/Person
#[derive(Serialize, Debug)]
pub struct Person<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    pub name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    #[serde(rename = "alternateName", skip_serializing_if = "Option::is_none")]
    pub alternate_name: Option<&'a str>,

    #[serde(rename = "sameAs", skip_serializing_if = "Option::is_none")]
    pub same_as: Option<&'a [String]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<&'a Location<'a>>,

    #[serde(
        rename = "potentialAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub potential_action: Option<&'a Action<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    //knows: Option<Vec<&'a Person<'a>>>,
    //children: Option<Vec<&'a Person<'a>>>,
    //job_title: Option<&'a str>,
    //works_for: Option<&'a Organization<'a>>,
    #[serde(skip)]
    pub web_page: WebPage<'a>,
}
impl<'a> Person<'a> {
    pub fn from_config(owner: &'a OwnerConfig, is_root: bool) -> Self {
        Person {
            thing: Thing::extend("Person".to_string(), None, is_root),
            name: &owner.name,
            alternate_name: None,
            url: None,
            email: owner.email.as_deref(),
            same_as: owner.urls.as_deref(),
            web_page: WebPage::<'a>::new("about".to_string(), false),
            address: None,
            image: None,
            potential_action: None,
        }

        // p.thing.main_entity_of_page = Some(ObjectOrURL::Object(&p.web_page));

        // p
    }
}

/// http://schema.org/Organization
#[derive(Serialize, Debug)]
pub struct Organization<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Location<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    #[serde(rename = "memberOf", skip_serializing_if = "Option::is_none")]
    pub member_of: Option<Agent<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<Vec<&'a Person<'a>>>, // spec also allows single

    #[serde(skip_serializing_if = "Option::is_none")]
    pub founder: Option<&'a Person<'a>>, // spec also allows array
}
impl<'a> Organization<'a> {
    pub fn new(name: String, logo_url: String, is_root: bool) -> Self {
        let mut thing =
            Thing::extend("Organization".to_string(), None, is_root);

        thing.name = Some(name);

        Organization {
            thing,
            email: None,
            address: None,
            logo: Some(ObjectOrURL::URL(logo_url)),
            member_of: None,
            employee: None,
            founder: None,
        }
    }

    pub fn from_config(config: &'a SiteConfig, is_root: bool) -> Self {
        Self::new(config.title.to_string(), config.url.to_string(), is_root)
    }
}

/// http://schema.org/SoftwareApplication
#[derive(Serialize, Debug)]
pub struct SoftwareApplication<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    #[serde(
        rename = "applicationCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub category: Option<&'a str>,

    #[serde(
        rename = "applicationSuite",
        skip_serializing_if = "Option::is_none"
    )]
    pub suite: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<&'a str>,

    #[serde(
        rename = "operatingSystem",
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system: Option<&'a str>,

    #[serde(
        rename = "softwareVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<&'a str>,
}
impl<'a> SoftwareApplication<'a> {
    fn new(is_root: bool) -> Self {
        SoftwareApplication {
            thing: Thing::extend(
                "SoftwareApplication".to_string(),
                None,
                is_root,
            ),
            category: None,
            suite: None,
            download_url: None,
            operating_system: None,
            version: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OwnerConfig;
    use serde_json;

    #[test]
    fn person_json() {
        let config = OwnerConfig {
            name: String::from("Test Person"),
            email: Some(String::from("user@email.io")),
            urls: Some(vec![
                "http://first.url".to_owned(),
                "http://second.url".to_owned(),
            ]),
            image: None,
        };
        let schema = Person::from_config(&config, true);

        let target = "{\
            '@context':'http://schema.org',\
            '@type':'Person',\
            'name':'Test Person',\
            'email':'user@email.io',\
            'sameAs':['http://first.url','http://second.url']\
        }"
        .replace("'", "\"");

        assert_eq!(serde_json::to_string(&schema).unwrap(), target.to_string());
    }
}
