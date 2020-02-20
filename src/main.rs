use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

type Pairs = Vec<(String, String)>;

#[derive(Serialize, Deserialize, Debug)]
struct ExifConfig {
    camera: Pairs,
    software: Pairs,
    lens: Pairs,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    categories: Vec<String>,
    what: Vec<String>,
    exif: ExifConfig,
}

fn main() {
    let path = Path::new("./public/config.toml");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    //let parsed = contents.parse::<Value>().unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    println!("{}", config.what[0])
}
