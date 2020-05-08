mod blog;
mod log;
pub mod post;
mod series;
mod vendors;

pub use blog::{
    BlogConfig, CategoryConfig, CategoryIcon, ExifConfig, FeaturedPost,
    ImageConfig, OwnerConfig, PhotoConfig, SiteConfig, SizeConfig,
};
pub use log::{BlogLog, PostLog};
pub use post::PostConfig;
pub use series::SeriesConfig;
pub use vendors::FacebookConfig;

use colored::*;

use serde::de::DeserializeOwned;
use std::{env, fs, path::Path};
use toml;

/// Configuration file for blog, for post series and for posts
pub static CONFIG_FILE: &str = "exifweb.toml";

/// Configuration that reads some values from environment variables
trait ReadsEnv {
    fn from_env(&mut self);
}

/// Value of environment variable or empty string if not found
fn env_or_empty(name: &str) -> String {
    env::var(name).unwrap_or("".to_string())
}

/// Load configuration from file in given path
///
/// *See* https://gitter.im/rust-lang/rust/archives/2018/09/07
fn load_config<D: DeserializeOwned>(path: &Path) -> Option<D> {
    load_toml::<D>(path, CONFIG_FILE, true)
}

fn load_ron<D: DeserializeOwned>(
    path: &Path,
    file_name: &str,
    print_when_missing: bool,
) -> Option<D> {
    let content = match fs::read_to_string(path.join(file_name)) {
        Ok(txt) => txt,
        _ => {
            if print_when_missing {
                println!(
                    "   {} {}",
                    file_name.purple(),
                    "not found: skipping".purple()
                );
            }
            return None;
        }
    };
    match ron::de::from_str::<D>(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            println!(
                "   {} {}, {:?}",
                "failed to parse".red(),
                file_name.red(),
                e
            );
            None
        }
    }
}

fn load_toml<D: DeserializeOwned>(
    path: &Path,
    file_name: &str,
    print_when_missing: bool,
) -> Option<D> {
    let content = match fs::read_to_string(path.join(file_name)) {
        Ok(txt) => txt,
        _ => {
            if print_when_missing {
                println!(
                    "   {} {}",
                    file_name.purple(),
                    "not found: skipping".purple()
                );
            }
            return None;
        }
    };
    match toml::from_str::<D>(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            println!(
                "   {} {}, {:?}",
                "failed to parse".red(),
                file_name.red(),
                e
            );
            None
        }
    }
}
