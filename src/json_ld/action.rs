use super::{
    agent::{Agent, SoftwareApplication},
    location::Location,
    ObjectOrURL, Thing,
};
use serde::Serialize;

/// An entry point, within some Web-based protocol.
/// http://schema.org/EntryPoint
#[derive(Serialize, Debug)]
pub struct EntryPoint<'a> {
    #[serde(flatten)]
    pub thing: Thing<'a>,

    /// An application that can complete the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_application: Option<&'a SoftwareApplication<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<&'a str>,

    /// An HTTP method that specifies the appropriate HTTP method for a request
    /// to an HTTP EntryPoint. Values are capitalized strings as used in HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<&'a str>,

    /// An url template (RFC6570) that will be used to construct the target of
    /// the execution of the action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_emplate: Option<&'a str>,
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
    pub thing: Thing<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Agent<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Thing<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ObjectOrURL<'a, EntryPoint<'a>>>,
}
impl<'a> Action<'a> {
    pub fn extend(type_name: String, is_root: bool) -> Self {
        Action {
            thing: Thing::extend(type_name, None, is_root),
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
    pub action: Action<'a>,
}
impl<'a> SearchAction<'a> {
    pub fn new(url: &'a str, is_root: bool) -> Self {
        SearchAction {
            action: Action::extend("SearchAction".to_string(), is_root),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DiscoverAction<'a> {
    #[serde(flatten)]
    action: Action<'a>,
}
impl<'a> DiscoverAction<'a> {
    pub fn new(url: String, is_root: bool) -> Self {
        let mut action = Action::extend("DiscoverAction".to_string(), is_root);

        action.target = Some(ObjectOrURL::URL(url));

        DiscoverAction { action }
    }
}
