use super::{
    creative_work::{CreativeWork, ImageObject, WebPage},
    location::Location,
    ObjectOrURL, Thing,
};
use crate::config::OwnerConfig;
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
    thing: Thing<'a, WebPage<'a>>,

    name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    same_as: Option<Vec<&'a str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<&'a Location<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ObjectOrURL<'a, ImageObject<'a>>>,
    //knows: Option<Vec<&'a Person<'a>>>,
    //children: Option<Vec<&'a Person<'a>>>,
    //job_title: Option<&'a str>,
    //works_for: Option<&'a Organization<'a>>,
}
impl<'a> Person<'a> {
    pub fn from_config(owner: &'a OwnerConfig) -> Self {
        let mut thing = Thing::extend("Person", None);

        // thing.main_entity_of_page =
        //     Some(ObjectOrURL::Object(WebPage::new("about")));

        // let mut email: Option<&'a str>;

        // if owner.email.is_some() {

        // }

        Person {
            thing,
            name: &owner.name,
            url: None,
            email: owner.email.as_deref(),
            same_as: None, //config.owner.urls,.
            address: None,
            image: None,
        }
    }
}

/// http://schema.org/Organization
#[derive(Serialize, Debug)]
pub struct Organization<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    email: Option<&'a str>,
    address: Option<Location<'a>>,
    logo: Option<ObjectOrURL<'a, ImageObject<'a>>>,

    member_of: Option<Agent<'a>>,
    employee: Option<Vec<&'a Person<'a>>>, // spec also allows single
    founder: Option<&'a Person<'a>>,       // spec also allows array
}
impl<'a> Organization<'a> {
    fn new(name: &'a str, logo_url: &'a str) -> Self {
        let mut thing = Thing::extend("Organization", None);
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
}

/// http://schema.org/SoftwareApplication
#[derive(Serialize, Debug)]
pub struct SoftwareApplication<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    application_category: Option<&'a str>,
    application_suite: Option<&'a str>,
    download_url: Option<&'a str>,
    operating_system: Option<&'a str>,
    software_version: Option<&'a str>,
}
impl<'a> SoftwareApplication<'a> {
    fn new() -> Self {
        SoftwareApplication {
            thing: Thing::extend("SoftwareApplication", None),
            application_category: None,
            application_suite: None,
            download_url: None,
            operating_system: None,
            software_version: None,
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
            urls: None,
            image: None,
        };
        let schema = Person::from_config(&config);

        let target = "{\
            \"@type\":\"Person\",\
            \"@context\":\"http://schema.org\",\
            \"name\":\"Test Person\",\
            \"email\":\"user@email.io\"\
        }";

        assert_eq!(
            serde_json::to_string(&schema).unwrap(),
            String::from(target)
        );
    }
}
