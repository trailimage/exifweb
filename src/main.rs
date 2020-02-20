mod config;

use std::fs;
use std::path::Path;
use toml;

fn main() {
    let path = Path::new("./public/config.toml");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let c: config::Config = toml::from_str(&contents).unwrap();

    println!("{}", c.what[0])
}
