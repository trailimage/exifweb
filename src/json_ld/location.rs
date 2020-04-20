use super::Thing;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Location {
    Place(Place),
    PostalAddress,
    Text(String),
}

#[derive(Serialize, Debug)]
pub struct Place {
    #[serde(flatten)]
    thing: Thing,
}
