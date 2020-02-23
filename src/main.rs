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
pub use post::Post;

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

fn main() {
    let root = Path::new("./public/");
    let config: BlogConfig = load_config(root);
    let mut blog = Blog::default();

    for entry in fs::read_dir(root.join("img/")).unwrap() {
        let path: PathBuf = entry.unwrap().path();

        if !path.is_dir() {
            continue;
        }

        for entry in fs::read_dir(path).unwrap() {
            let path: PathBuf = entry.unwrap().path();

            match load_series(&path) {
                Some(posts) => {
                    println!("{} series posts", posts.len());
                    for p in posts {
                        blog.posts.push(p);
                    }
                    continue;
                }
                None => blog.posts.push(load_post(path.as_path())),
            }
        }
    }

    println!("{} posts", blog.posts.len());
}

fn load_series<'a>(path: &Path) -> Option<Vec<Post<'a>>> {
    let sub_dirs: Vec<PathBuf> = fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();

    if sub_dirs.is_empty() {
        return None;
    }

    let config: SeriesConfig = load_config(path);
    let series_posts: Vec<Post> = sub_dirs
        .iter()
        .map(|p| load_series_post(p.as_path(), &config))
        .collect();

    Some(series_posts)
}

/// Create post that is part of a series.
fn load_series_post<'a>(path: &Path, series: &SeriesConfig) -> Post<'a> {
    let config: PostConfig = load_config(&path);
    Post {
        title: series.title,
        sub_title: config.title,
        summary: config.summary,
        is_partial: true,
        total_parts: series.parts,
        chronological: true,
        ..Post::default()
    }
}

/// Create post that is not part of a series.
fn load_post<'a>(path: &Path) -> Post<'a> {
    let config: PostConfig = load_config(&path);
    Post {
        title: config.title,
        summary: config.summary,
        is_partial: false,
        total_parts: 0,
        chronological: true,
        ..Post::default()
    }
}

/// Load configuration from given path.
fn load_config<'de, T: Deserialize<'de>>(path: &'de Path) -> T {
    let contents = fs::read_to_string(path.join("config.toml")).unwrap();
    toml::from_str(&contents).unwrap()
}
