#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod blog;
mod caption;
mod category;
mod config;
mod html;
mod image;
mod photo;
mod post;
mod tools;

pub use blog::Blog;
pub use caption::Caption;
pub use category::Category;
pub use config::*;
//pub use photo::{Camera, ExposureMode, Location, Photo};
pub use post::Post;
pub use tools::{
    has_ext, min_date, path_name, pos_from_name, pos_from_path, replace_pairs,
    slugify, tab, LoadError, Pairs,
};

use colored::*;
use image::exif_tool::parse_dir;
use photo::Photo;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml;

static CONFIG_FILE: &str = "config.toml";

/// Patterns to extract position information from the names of photos and post
/// folders in a series
struct Match {
    // pattern to extract one-based index from series path name
    series_post: Regex,
    // pattern to extract one-based index from photo file name
    photo: Regex,
}

fn main() {
    // GitHub pages feature requires root at / or /docs
    let root = Path::new("./docs/");
    let config = match load_config::<BlogConfig>(root) {
        Some(config) => config,
        _ => {
            println!("{}", "Missing root configuration file".red());
            return;
        }
    };

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        _ => {
            println!(
                "{} {}",
                "Failed to open root directory".red(),
                path_name(root).red()
            );
            return;
        }
    };

    let mut blog = Blog::default();

    // It is apparently tricky to have Serde automatically deserialize these
    // to Regex instances so instead do it manually
    let infer_position = Match {
        series_post: Regex::new(&config.series_index_pattern).unwrap(),
        photo: Regex::new(&config.photo.index_pattern).unwrap(),
    };

    // iterate over every file or subdirectory within root
    for entry in entries {
        let path: PathBuf = entry.unwrap().path();

        if !path.is_dir() {
            // ignore root files
            continue;
        }

        println!(
            "\n{} {}",
            "Found root directory".bold(),
            path_name(&path).bold().yellow()
        );

        if let Some(posts) = load_series(&path, &infer_position) {
            println!(
                "{:tab$}Found {} series posts",
                "",
                posts.len(),
                tab = tab(1)
            );
            for p in posts {
                println!(
                    "{:tab$}{} ({} photos)",
                    "",
                    &p.key,
                    &p.photos.len(),
                    tab = tab(2)
                );
                blog.add_post(p);
            }
            // skip to next path entry if series was found
            continue;
        }

        if let Some(post) = load_post(path.as_path(), &infer_position) {
            blog.add_post(post);
        }
    }

    println!(
        "{}",
        format!("\nFound {} total posts", blog.posts.len())
            .bold()
            .green()
    );

    blog.correlate_posts();
    let blog = blog.collate_tags();
    blog.sanitize_exif(&config.photo.exif);
}

/// Attempt to load path entries as if they constitute a post series. `None` is
/// returned if there are no subdirectories or they don't contain valid posts.
fn load_series(path: &Path, re: &Match) -> Option<Vec<Post>> {
    let sub_dirs: Vec<PathBuf> = match fs::read_dir(&path) {
        Ok(entries) => entries
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_dir())
            .collect(),
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "Failed to open subdirectory".red(),
                path_name(&path).red().bold(),
                tab = tab(1)
            );
            return None;
        }
    };

    if sub_dirs.is_empty() {
        // not a series if there are no subdirectories
        return None;
    }

    if let Some(config) = load_config::<SeriesConfig>(path) {
        return Some(
            sub_dirs
                .iter()
                .map(|p| load_series_post(p.as_path(), &config, re))
                // ignore None results already logged to console
                .filter(|p| p.is_some())
                .map(|p| p.unwrap())
                .collect(),
        );
    }
    // making it here implies no configuration file
    None
}

/// Create post that is part of a series. This differs from non-series post
/// creation with the addition of several fields that identify how the post
/// relates to the series.
fn load_series_post(
    path: &Path,
    series_config: &SeriesConfig,
    re: &Match,
) -> Option<Post> {
    load_config::<PostConfig>(&path).and_then(|c| {
        let part = pos_from_path(&re.series_post, &path).unwrap_or(0);

        if part == 0 {
            return None;
        }

        let photos = load_photos(path, re, c.cover_photo_index);

        if photos.is_empty() {
            return None;
        }

        Some(Post {
            key: format!(
                "{}/{}",
                slugify(&series_config.title),
                slugify(&c.title)
            ),
            title: series_config.title.clone(),
            sub_title: c.title,
            summary: c.summary,
            part,
            is_partial: true,
            total_parts: series_config.parts,
            prev_is_part: part > 1,
            next_is_part: part < series_config.parts,
            photos,
            ..Post::default()
        })
    })
}

/// Create post that is not part of a series.
fn load_post(path: &Path, re: &Match) -> Option<Post> {
    load_config::<PostConfig>(&path).and_then(|c| {
        let photos = load_photos(path, re, c.cover_photo_index);

        if photos.is_empty() {
            return None;
        }

        Some(Post {
            key: slugify(&c.title),
            title: c.title,
            summary: c.summary,
            photos,
            ..Post::default()
        })
    })
}

/// Load configuration from file in given path
///
/// *See* https://gitter.im/rust-lang/rust/archives/2018/09/07
fn load_config<D: DeserializeOwned>(path: &Path) -> Option<D> {
    let content = match fs::read_to_string(path.join(CONFIG_FILE)) {
        Ok(txt) => txt,
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                CONFIG_FILE.red(),
                "not found — skipping".red(),
                tab = tab(1)
            );
            return None;
        }
    };
    match toml::from_str::<D>(&content) {
        Ok(config) => Some(config),
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "failed to parse".red(),
                CONFIG_FILE.red(),
                tab = tab(1)
            );
            None
        }
    }
}

/// Load information about each post photo
fn load_photos(path: &Path, re: &Match, cover_photo_index: u8) -> Vec<Photo> {
    let photos: Vec<Photo> = parse_dir(&path, cover_photo_index, &re.photo);

    if photos.is_empty() {
        println!("{:tab$}{}", "", "found no photos".red(), tab = tab(1));
    }
    photos
}
