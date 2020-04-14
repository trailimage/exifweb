use serde::{Deserialize, Serialize};
use serde_json;

struct CreateWork {
    author: Option<String>,

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
    same_as: Option<URL>,
    url: Option<URL>,
}
#[derive(Deserialize, Debug)]
struct URL(Thing);
