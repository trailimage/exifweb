use super::{
    agent::{Agent, SoftwareApplication},
    creative_work::CreativeWork,
    location::Location,
    ObjectOrURL, Thing,
};
use serde::Serialize;

/// An entry point, within some Web-based protocol.
/// http://schema.org/EntryPoint
#[derive(Serialize, Debug)]
pub struct EntryPoint<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    /// An application that can complete the request
    action_application: Option<&'a SoftwareApplication<'a>>,
    content_type: Option<&'a str>,
    encoding_type: Option<&'a str>,
    /// An HTTP method that specifies the appropriate HTTP method for a request
    /// to an HTTP EntryPoint. Values are capitalized strings as used in HTTP.
    http_method: Option<&'a str>,
    /// An url template (RFC6570) that will be used to construct the target of
    /// the execution of the action
    url_emplate: Option<&'a str>,
}

/// An action performed by a direct agent and indirect participants upon a
/// direct object. Optionally happens at a location with the help of an
/// inanimate instrument. The execution of the action may produce a result.
/// Specific action sub-type documentation specifies the exact expectation of
/// each argument/role.
///
/// http://schema.org/Action
///
#[derive(Serialize, Debug)]
pub struct Action<'a> {
    #[serde(flatten)]
    thing: Thing<'a, CreativeWork<'a>>,

    agent: Option<Agent<'a>>,
    participant: Option<Agent<'a>>,
    error: Option<Thing<'a, CreativeWork<'a>>>,
    location: Option<Location<'a>>,

    target: Option<ObjectOrURL<'a, EntryPoint<'a>>>,
}
impl<'a> Action<'a> {
    pub fn extend(_type: &'a str) -> Self {
        Action {
            thing: Thing::extend(_type, None),
            agent: None,
            participant: None,
            error: None,
            location: None,
            target: None,
        }
    }
}

/// http://schema.org/SearchAction
#[derive(Serialize, Debug)]
pub struct SearchAction<'a> {
    #[serde(flatten)]
    action: Action<'a>,
}
impl<'a> SearchAction<'a> {
    pub fn new(url: &'a str) -> Self {
        SearchAction {
            action: Action::extend("SearchAction"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DiscoverAction<'a> {
    #[serde(flatten)]
    action: Action<'a>,
}
impl<'a> DiscoverAction<'a> {
    pub fn new(url: &'a str) -> Self {
        let mut action = Action::extend("DiscoverAction");

        action.target = Some(ObjectOrURL::URL(url));

        DiscoverAction { action }
    }
}
