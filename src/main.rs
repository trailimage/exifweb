mod blog;
mod caption;
mod category;
mod config;
mod exif;
mod photo;
mod post;

pub use blog::Blog;
pub use caption::Caption;
pub use category::Category;
pub use config::*;
pub use exif::EXIF;
pub use photo::Photo;
pub use post::{slugify, Post};

use regex::Regex;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

fn main() {
    let root = Path::new("./public/");
    let config: BlogConfig = load_config(root);
    let mut blog = Blog::default();
    // It is apparently tricky to have Serde automatically deserialize these
    // to Regex instances so instead to it manually
    let matcher = Match {
        series_index: Regex::new(&config.series_index_pattern).unwrap(),
        photo_index: Regex::new(&config.photo.index_pattern).unwrap(),
    };

    for entry in fs::read_dir(root.join("img/")).unwrap() {
        let path: PathBuf = entry.unwrap().path();

        if !path.is_dir() {
            continue;
        }

        for entry in fs::read_dir(path).unwrap() {
            let path: PathBuf = entry.unwrap().path();

            match load_series(&path, &matcher) {
                Some(posts) => {
                    println!("Found {} series posts", posts.len());
                    for p in posts {
                        blog.add_post(p);
                    }
                    continue;
                }
                None => blog.add_post(load_post(path.as_path(), &matcher)),
            }
        }
    }

    println!("Found {} total posts", blog.posts.len());

    //blog.correlate_posts()
}

fn load_series<'a>(path: &Path, re: &Match) -> Option<Vec<Post<'a>>> {
    let sub_dirs: Vec<PathBuf> = fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();

    if sub_dirs.is_empty() {
        return None;
    }

    let series_config: SeriesConfig = load_config(path);
    let series_posts: Vec<Post> = sub_dirs
        .iter()
        .map(|p| load_series_post(p.as_path(), &series_config, re))
        .collect();

    Some(series_posts)
}

/// Create post that is part of a series.
fn load_series_post<'a>(
    path: &Path,
    series_config: &SeriesConfig,
    re: &Match,
) -> Post<'a> {
    let post_config: PostConfig = load_config(&path);
    // name of series sub-folder
    let dir = path.file_name().unwrap().to_str().unwrap();
    let caps = re.series_index.captures(dir).unwrap();
    let part: u8 = caps[1].parse().unwrap();

    Post {
        title: series_config.title.clone(),
        sub_title: post_config.title,
        summary: post_config.summary,
        part,
        is_partial: true,
        total_parts: series_config.parts,
        prev_is_part: part > 1,
        next_is_part: part < series_config.parts,
        ..Post::default()
    }
}

/// Create post that is not part of a series.
fn load_post<'a>(path: &Path, re: &Match) -> Post<'a> {
    let config: PostConfig = load_config(&path);
    Post {
        title: config.title,
        summary: config.summary,
        ..Post::default()
    }
}

/// Load configuration from given path.
///
/// *See* https://gitter.im/rust-lang/rust/archives/2018/09/07
fn load_config<D: DeserializeOwned>(path: &Path) -> D {
    static FILE_NAME: &str = "config.toml";
    let content =
        fs::read_to_string(path.join(FILE_NAME)).unwrap_or_else(|_e| {
            panic!("{} not found in {:?}", FILE_NAME, path.to_str())
        });

    toml::from_str(&content).unwrap()
}

struct Match {
    series_index: Regex,
    photo_index: Regex,
}
