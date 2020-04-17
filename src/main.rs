//#![allow(warnings)]
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod config;
mod deserialize;
mod html;
mod image;
mod models;
mod template;
mod tools;

use ::regex::Regex;
use chrono::{DateTime, FixedOffset};
use colored::*;
use config::{BlogConfig, PhotoLog, PostConfig, SeriesConfig};
use image::exif_tool;
use models::{Blog, Category, CategoryKind, Photo, Post};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use template::{write_about, write_post, write_sitemap};

use tools::{
    earliest_photo_date, identify_outliers, path_name, path_slice,
    pos_from_path,
};

fn main() {
    // GitHub pages feature requires root at / or /docs
    let root = Path::new("./docs/");
    let mut config = match BlogConfig::load(root) {
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

    let args: Vec<String> = env::args().collect();
    let mut blog = Blog::default();

    config.force_rerender = args.contains(&"force".to_owned());

    println!(
        "{}",
        format!("\nForce re-render: {}", config.force_rerender)
            .cyan()
            .bold()
    );

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
                println!("{:6}{} ({} photos)", "", &p.path, &p.photos.len());
                blog.add_post(p);
            }
            // skip to next path entry if series was found
            continue;
        }

        if let Some(post) = load_post(path.as_path(), &config) {
            blog.add_post(post);
        }
    }

    print!("\n");
    success_metric(blog.post_count(), "total posts");

    if !blog.is_empty() {
        blog.correlate_posts();
        blog.collate_tags();

        success_metric(blog.category_count(), "post categories");
        success_metric(blog.tag_count(), "unique photo tags");

        blog.sanitize_exif(&config.photo.exif);

        for (_, p) in &blog.posts {
            write_post(root, &config, &blog, &p)
        }
    }

    write_sitemap(root, &config, &blog);
    write_about(root, &config);
}

fn success_metric(count: usize, label: &str) {
    println!("{}", format!("{:>5} {}", count, label).bold().green());
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
                "   {} {}",
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

    if let Some(series_config) = SeriesConfig::load(path) {
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

/// Create post that is part of a series. This differs from non-series post
/// creation with the addition of several fields that identify how the post
/// relates to the series.
fn load_series_post(
    path: &Path,
    config: &BlogConfig,
    series_config: &SeriesConfig,
) -> Option<Post> {
    PostConfig::load(&path).and_then(|c| {
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
            let categories = c.categories();

            Some(Post {
                path: path_slice(path, 2),
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
    PostConfig::load(&path).and_then(|c| {
        // TODO: load log
        let (photos, happened_on) =
            load_photos(path, &config.photo.capture_index, c.cover_photo_index);

        if photos.is_empty() {
            None
        } else {
            let categories = c.categories();

            Some(Post {
                path: path_slice(path, 1),
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

/// Load information about each post photo
fn load_photos(
    path: &Path,
    re: &Regex,
    cover_photo_index: u8,
) -> (Vec<Photo>, Option<DateTime<FixedOffset>>) {
    let mut photos: Vec<Photo> =
        exif_tool::parse_dir(&path, cover_photo_index, &re);

    if photos.is_empty() {
        println!("   {}", "found no photos".red());

        (photos, None)
    } else {
        identify_outliers(&mut photos);
        let happened_on = earliest_photo_date(&photos);

        PhotoLog::write(path, happened_on, &photos);

        (photos, happened_on)
    }
}
