#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod config;
mod deserialize;
mod html;
mod image;
mod models;
mod tools;

use ::regex::Regex;
use chrono::{DateTime, FixedOffset, Local};
use colored::*;
use config::*;
use image::exif_tool;
use models::{Blog, Category, CategoryKind, Photo, Post};
use serde::de::DeserializeOwned;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml;
use tools::{
    earliest_photo_date, identify_outliers, path_name, pos_from_path, slugify,
};

/// Configuration file for blog, for post series and for posts
static CONFIG_FILE: &str = "config.toml";
/// File the stores photo tag information and last process time
static LOG_FILE: &str = "log.toml";

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

        if let Some(posts) = load_series(&path, &config) {
            println!("   Found {} series posts", posts.len());
            for p in posts {
                println!("{:>6} ({} photos)", &p.key, &p.photos.len());
                blog.add_post(p);
            }
            // skip to next path entry if series was found
            continue;
        }

        if let Some(post) = load_post(path.as_path(), &config) {
            blog.add_post(post);
        }
    }

    println!(
        "{}",
        format!("\nFound {} total posts", blog.post_count())
            .bold()
            .green()
    );

    if !blog.is_empty() {
        blog.correlate_posts();
        blog.collate_tags();

        println!(
            "{}",
            format!("\nFound {} unique photo tags", blog.tag_count())
                .bold()
                .green()
        );

        blog.sanitize_exif(&config.photo.exif);
    }
}

/// Attempt to load path entries as if they constitute a post series. `None` is
/// returned if there are no subdirectories or they don't contain valid posts.
fn load_series(path: &Path, config: &BlogConfig) -> Option<Vec<Post>> {
    let sub_dirs: Vec<PathBuf> = match fs::read_dir(&path) {
        Ok(entries) => entries
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_dir())
            .collect(),
        _ => {
            println!(
                "{:>3} {}",
                "Failed to open subdirectory".red(),
                path_name(&path).red().bold()
            );
            return None;
        }
    };

    if sub_dirs.is_empty() {
        // not a series if there are no subdirectories
        return None;
    }

    if let Some(series_config) = load_config::<SeriesConfig>(path) {
        return Some(
            sub_dirs
                .iter()
                .map(|p| load_series_post(p.as_path(), config, &series_config))
                // ignore None results already logged to console
                .filter(|p| p.is_some())
                .map(|p| p.unwrap())
                .collect(),
        );
    }
    // making it here implies no configuration file
    None
}

/// Convert configured categories to a vector
fn parse_categories(config: &PostConfig) -> Vec<Category> {
    let mut categories: Vec<Category> = vec![
        Category {
            name: config.categories.when.clone(),
            kind: CategoryKind::When,
        },
        Category {
            name: config.categories.what.clone(),
            kind: CategoryKind::What,
        },
        Category {
            name: config.categories.who.clone(),
            kind: CategoryKind::Who,
        },
    ];

    for w in config.categories.r#where.iter() {
        categories.push(Category {
            name: w.clone(),
            kind: CategoryKind::Where,
        });
    }
    categories
}

/// Create post that is part of a series. This differs from non-series post
/// creation with the addition of several fields that identify how the post
/// relates to the series.
fn load_series_post(
    path: &Path,
    config: &BlogConfig,
    series_config: &SeriesConfig,
) -> Option<Post> {
    load_config::<PostConfig>(&path).and_then(|c| {
        // TODO: load log
        let part =
            pos_from_path(&config.capture_series_index, &path).unwrap_or(0);

        if part == 0 {
            return None;
        }
        let (photos, happened_on) =
            load_photos(path, &config.photo.capture_index, c.cover_photo_index);

        if photos.is_empty() {
            None
        } else {
            let categories = parse_categories(&c);

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
                happened_on,
                photos,
                categories,
                ..Post::default()
            })
        }
    })
}

/// Create post that is not part of a series.
fn load_post(path: &Path, config: &BlogConfig) -> Option<Post> {
    load_config::<PostConfig>(&path).and_then(|c| {
        // TODO: load log
        let (photos, happened_on) =
            load_photos(path, &config.photo.capture_index, c.cover_photo_index);

        if photos.is_empty() {
            None
        } else {
            let categories = parse_categories(&c);
            Some(Post {
                key: slugify(&c.title),
                title: c.title,
                summary: c.summary,
                happened_on,
                photos,
                categories,
                ..Post::default()
            })
        }
    })
}

/// Load configuration from file in given path
///
/// *See* https://gitter.im/rust-lang/rust/archives/2018/09/07
fn load_config<D: DeserializeOwned>(path: &Path) -> Option<D> {
    load_toml::<D>(path, CONFIG_FILE)
}

fn load_Log(path: &Path) -> Option<PostPhotos> {
    load_toml(path, LOG_FILE)
}

fn load_toml<D: DeserializeOwned>(path: &Path, file_name: &str) -> Option<D> {
    let content = match fs::read_to_string(path.join(file_name)) {
        Ok(txt) => txt,
        _ => {
            println!("{:>3} {}", file_name.red(), "not found: skipping".red());
            return None;
        }
    };
    match toml::from_str::<D>(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            println!(
                "{:>3} {}, {:?}",
                "failed to parse".red(),
                file_name.red(),
                e
            );
            None
        }
    }
}

/// Load information about each post photo
fn load_photos(
    path: &Path,
    re: &Regex,
    cover_photo_index: u8,
) -> (Vec<Photo>, Option<DateTime<FixedOffset>>) {
    let mut photos: Vec<Photo> =
        exif_tool::parse_dir(&path, cover_photo_index, &re);

    if photos.is_empty() {
        println!("{:>3}", "found no photos".red());

        (photos, None)
    } else {
        identify_outliers(&mut photos);
        let happened_on = earliest_photo_date(&photos);
        write_log(path, happened_on, &photos);

        (photos, happened_on)
    }
}

fn write_log(
    path: &Path,
    earliest_date: Option<DateTime<FixedOffset>>,
    photos: &Vec<Photo>,
) {
    let mut tags: Vec<String> = Vec::new();

    for p in photos.iter() {
        for t in p.tags.iter() {
            if !tags.contains(&t) {
                tags.push(t.clone())
            }
        }
    }

    tags.sort();

    let log = PostPhotos {
        when: earliest_date,
        processed: Local::now(),
        // TODO: I think these need to be tags that map to photos
        tags,
    };

    match toml::to_string(&log) {
        Ok(content) => {
            match fs::write(path.join(LOG_FILE), &content) {
                Ok(_) => (),
                Err(e) => eprintln!("Error writing {:?}", e),
            };
            return;
        }
        Err(e) => eprintln!("Error serializaing {:?}", e),
    }
}
