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
use config::{BlogConfig, PostConfig, PostLog, SeriesConfig, CONFIG_FILE};
use image::exif_tool;
use models::{Blog, Category, CategoryKind, Photo, Post};
use std::{
    env,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
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
        let dir_name: &str = path_name(&path);

        if !path.is_dir()
            || config.ignore_folders.contains(&dir_name.to_string())
        {
            // ignore root files and configured folders
            continue;
        }

        println!("\n{} └ {}", "Found root directory".bold(), dir_name.bold());

        if let Some(posts) = load_series(&path, &config) {
            println!("   Series of {} posts:", posts.len());
            for p in posts {
                println!(
                    "{:6}{} ({} photos)",
                    "",
                    p.sub_title.yellow(),
                    p.photo_count
                );
                blog.add_post(p);
            }
            // skip to next path entry if series was found
            continue;
        }

        if let Some(post) = load_post(path.as_path(), &config) {
            println!(
                "   {} ({} photos)",
                post.title.yellow(),
                post.photo_count
            );
            blog.add_post(post);
        }
    }

    let changed_count = blog.changed_count();

    print!("\n");
    success_metric(blog.post_count(), "total posts");
    success_metric(changed_count, "changed posts");

    if !blog.is_empty() && changed_count > 0 {
        blog.correlate_posts();
        blog.collate_tags();

        success_metric(blog.category_count(), "post categories");
        success_metric(blog.tag_count(), "unique photo tags");

        blog.sanitize_exif(&config.photo.exif);

        for (_, p) in &blog.posts {
            if p.changed {
                write_post(root, &config, &blog, &p)
            }
        }
        write_sitemap(root, &config, &blog);
        write_about(root, &config);
    }
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
    let part = pos_from_path(&config.capture_series_index, &path).unwrap_or(0);

    if part == 0 {
        return None;
    }

    PostConfig::load(&path).and_then(|post_config| {
        post_from_log_or_photos(path, config, &post_config).and_then(|p| {
            Some(Post {
                categories: post_config.categories(),
                path: path_slice(path, 1),
                part,
                total_parts: series_config.parts,
                is_partial: true,
                prev_is_part: part > 1,
                next_is_part: part < series_config.parts,
                title: series_config.title.clone(),
                sub_title: post_config.title,
                summary: post_config.summary,
                ..p
            })
        })
    })
}

/// Create post that is not part of a series
fn load_post(path: &Path, config: &BlogConfig) -> Option<Post> {
    PostConfig::load(&path).and_then(|post_config| {
        post_from_log_or_photos(path, config, &post_config).and_then(|p| {
            Some(Post {
                categories: post_config.categories(),
                path: path_slice(path, 1),
                title: post_config.title,
                summary: post_config.summary,
                ..p
            })
        })
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

        PostLog::write(path, happened_on, &photos);

        (photos, happened_on)
    }
}

/// Load basic post from previous render log or by reading photo files
fn post_from_log_or_photos(
    path: &Path,
    config: &BlogConfig,
    post_config: &PostConfig,
) -> Option<Post> {
    match load_post_log(path) {
        Some(log) => Some(Post {
            happened_on: log.when,
            photo_count: log.photo_count,
            changed: false, // unchanged if log is valid
            ..Post::default()
        }),
        _ => {
            let (photos, happened_on) = load_photos(
                path,
                &config.photo.capture_index,
                post_config.cover_photo_index,
            );

            if photos.is_empty() {
                None
            } else {
                Some(Post {
                    happened_on,
                    photo_count: photos.len(),
                    photos,
                    ..Post::default()
                })
            }
        }
    }
}

/// Load post log if source files are still valid (no additions or deletions and
/// unchanged)
fn load_post_log(path: &Path) -> Option<PostLog> {
    PostLog::load(path).and_then(|log| {
        match is_modified(
            path,
            log.processed.timestamp(),
            log.photo_count + 1, // photos plus configuration file
            |name: &str| name.ends_with(".tif") || name == CONFIG_FILE,
        ) {
            Ok(modified) => {
                if modified {
                    None // do not return log if post files have changed
                } else {
                    Some(log)
                }
            }
            Err(e) => {
                println!(
                    "Failed to check {} for change {:?}",
                    path_name(path),
                    e
                );
                None
            }
        }
    })
}

/// Whether `path` contains any `allow_name` files modified after `threshold`
/// timestamp
fn is_modified(
    path: &Path,
    threshold: i64,
    file_count: usize,
    allow_name: fn(name: &str) -> bool,
) -> io::Result<bool> {
    if !path.is_dir() {
        return Ok(true);
    }
    let mut count: usize = 0;

    for entry in fs::read_dir(path)? {
        let entry: DirEntry = entry?;
        let os_name = entry.file_name();
        let name: &str = os_name.to_str().unwrap();

        if !allow_name(name) {
            continue;
        }
        count += 1;

        if count > file_count {
            // more than expected files
            return Ok(true);
        }

        let modified: i64 = entry
            .metadata()?
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if threshold < modified {
            // file modified more recently than threshold
            return Ok(true);
        }
    }

    // path is modified if it has a different file count
    Ok(file_count != count)
}
