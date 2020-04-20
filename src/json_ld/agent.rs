use super::Thing;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Agent {
    Person(Person),
    Organization(Organization),
}

/// http://schema.org/Person
#[derive(Serialize, Debug)]
pub struct Person {
    #[serde(flatten)]
    thing: Thing,

    name: String,
    url: String,
    same_as: String,
    main_entity_of_page: String,
    image: String,
    job_title: Option<String>,
}

/// http://schema.org/Organization
#[derive(Serialize, Debug)]
pub struct Organization {
    #[serde(flatten)]
    thing: Thing,

    email: Option<String>,
}
