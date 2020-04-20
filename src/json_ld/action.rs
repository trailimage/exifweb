use super::{Agent, Location, Thing};
use serde::Serialize;

/// An action performed by a direct agent and indirect participants upon a
/// direct object. Optionally happens at a location with the help of an
/// inanimate instrument. The execution of the action may produce a result.
/// Specific action sub-type documentation specifies the exact expectation of
/// each argument/role.
///
/// http://schema.org/Action
///
#[derive(Serialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    thing: Thing,

    agent: Agent,
    participant: Agent,
    error: Thing,
    location: Location,
}
