use super::{env_or_empty, ReadsEnv};
use serde::Deserialize;

// https://developers.facebook.com/docs/reference/plugins/like/
// https://developers.facebook.com/apps/110860435668134/summary
#[derive(Deserialize, Debug)]
pub struct FacebookConfig {
    pub app_id: String,
    pub admin_id: String,
    pub page_id: String,
    pub site_id: String,
    pub author_url: String,
}

#[derive(Deserialize, Debug)]
pub struct MapBoxStyles {
    pub dynamic: String,
    pub r#static: String,
}

#[derive(Deserialize, Debug)]
pub struct MapBoxConfig {
    #[serde(skip)]
    pub access_token: String,
    /// Maximum number of photo markers to show on static map
    pub max_static_markers: u16,
    pub style: MapBoxStyles,
    /// Fully qualified path to image used to render pins on static map
    pub pin_image: String,
}
impl ReadsEnv for MapBoxConfig {
    fn from_env(&mut self) {
        self.access_token = env_or_empty("MAPBOX_ACCESS_TOKEN")
    }
}

#[derive(Deserialize, Debug)]
pub struct GoogleConfig {
    #[serde(skip)]
    pub api_key: String,

    pub project_id: String,
    /// Shown as `UA-<analytics_id>-1`
    pub analytics_id: String,

    #[serde(skip)]
    pub search_engine_id: String,
    pub blog_id: String,
}
impl ReadsEnv for GoogleConfig {
    fn from_env(&mut self) {
        self.api_key = env_or_empty("GOOGLE_KEY");
        self.search_engine_id = env_or_empty("GOOGLE_SEARCH_ID");
    }
}
