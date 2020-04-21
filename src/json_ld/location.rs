use super::{creative_work::ImageObject, CreativeWork, ObjectOrURL, Thing};
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
    thing: Thing<'a, CreativeWork<'a>>,

    logo: Option<ObjectOrURL<'a, ImageObject<'a>>>,
    photo: Option<ImageObject<'a>>,
    has_map: Option<ObjectOrURL<'a, ImageObject<'a>>>,
}
impl<'a> Place<'a> {
    fn new(map_url: &'a str) -> Self {
        Place {
            thing: Thing::extend("Place", None),
            logo: None,
            photo: None,
            has_map: Some(ObjectOrURL::URL(map_url)),
        }
    }
}
