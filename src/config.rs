use serde_derive::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
struct ExifConfig {
    camera: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    categories: Vec<String>,
    what: Vec<String>,
    exfi: ExifConfig,
}

fn main() {
    let config: Config = from_str("");
}
