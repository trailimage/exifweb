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

use colored::*;
use config::{
    BlogConfig, PhotoConfig, PostConfig, PostLog, SeriesConfig, CONFIG_FILE,
};
use image::exif_tool;
use models::{collate_tags, Blog, Photo, Post};
use std::{
    env,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use template::Writer;
use tools::{
    earliest_photo_date, final_path_name, identify_outliers, path_slice,
    pos_from_path,
};

// TODO: generate mini maps
// TODO: read and process GPX files

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
                final_path_name(root).red()
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
        let dir_name: &str = final_path_name(&path);

        if !path.is_dir()
            || config.ignore_folders.contains(&dir_name.to_string())
        {
            // ignore root files and specified folders
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

    print!("\n");
    success_metric(blog.post_count(), "total posts");

    if blog.is_empty() {
        return;
    }

    let sequence_changed = blog.correlate_posts();

    // Previously loaded posts that haven't changed but have different previous
    // or next posts need to be re-rendered to update navigation HTML
    load_post_photos(root, &config, &mut blog, &sequence_changed);

    let render_count = blog.needs_render_count();

    success_metric(render_count, "posts need rendered");

    if render_count > 0 {
        blog.correlate_posts();
        blog.collate_tags();

        success_metric(blog.category_count(), "post categories");
        success_metric(blog.tag_count(), "unique photo tags");

        blog.sanitize_exif(&config.photo.exif);

        let write = Writer::new(root, &config, &blog);

        for (_, p) in &blog.posts {
            if p.needs_render {
                write.post(&p);
                // TODO: spawn thread to write log
                PostLog::write(root, p);
            }
        }
        write.home_page();
        write.about_page();
        write.sitemap();
        write.category_menu();
        write.mobile_menu();

        for (_, list) in &blog.categories {
            for c in list {
                write.category(&c);
            }
        }
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
            .map(|e: io::Result<DirEntry>| e.unwrap().path())
            .filter(|p: &'_ PathBuf| p.is_dir())
            .collect(),
        _ => {
            println!(
                "   {} {}",
                "Failed to open subdirectory".red(),
                final_path_name(&path).red().bold()
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
                .filter_map(|p| {
                    load_series_post(p.as_path(), config, &series_config)
                })
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
        create_post(path, true, config, &post_config).and_then(|p| {
            Some(Post {
                categories: post_config.categories(),
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
        create_post(path, false, config, &post_config).and_then(|p| {
            Some(Post {
                categories: post_config.categories(),
                title: post_config.title,
                summary: post_config.summary,
                ..p
            })
        })
    })
}

/// Load photos for all posts with given `paths`. This may be used to populate
/// posts initially created from a log file but then found to have changed
/// sequence (different next or previous post), requiring a re-render which
/// needs complete photo information.
fn load_post_photos(
    root: &Path,
    config: &BlogConfig,
    blog: &mut Blog,
    paths: &Vec<String>,
) {
    for path in paths.iter() {
        println!(" Attempting to add photos to {}", path);

        let mut photos = load_photos(&root.join(path), &config.photo);

        blog.add_post_photos(path, &mut photos)
    }
}

/// Load information about each photo in `path`
///
/// - `capture_photo_index` Photos will be sorted with the index captured with
///    this pattern
///
fn load_photos(path: &Path, config: &PhotoConfig) -> Vec<Photo> {
    let mut photos: Vec<Photo> = exif_tool::parse_dir(&path, config);

    if photos.is_empty() {
        println!("   {}", "found no photos".red());
    } else {
        identify_outliers(&mut photos);
        photos.sort();
    }
    photos
}

/// Load basic post data from previous render log or by reading photo files
fn create_post(
    path: &Path,
    is_series: bool,
    config: &BlogConfig,
    post_config: &PostConfig,
) -> Option<Post> {
    // path to series post includes parent
    let post_path = path_slice(path, if is_series { 2 } else { 1 });

    match load_post_log(path, config) {
        Some(log) => Some(Post {
            path: post_path,
            happened_on: log.happened_on,
            photo_count: log.photo_count,
            needs_render: false,
            tags: log.tags.clone(),
            history: Some(log),
            cover_photo_index: post_config.cover_photo_index,
            ..Post::default()
        }),
        _ => {
            let photos = load_photos(path, &config.photo);

            if photos.is_empty() {
                None
            } else {
                Some(Post {
                    tags: collate_tags(&photos),
                    path: post_path,
                    happened_on: earliest_photo_date(&photos),
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
fn load_post_log(path: &Path, config: &BlogConfig) -> Option<PostLog> {
    if config.force_rerender {
        return None;
    }

    PostLog::load(path).and_then(|log| {
        match is_modified(
            path,
            log.as_of.timestamp(),
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
                    "   Failed to check {} for change {:?}",
                    final_path_name(path),
                    e
                );
                None
            }
        }
    })
}

// TODO: retrieve per-photo modified date to know they need to have their sizes
// regenerated (independent of re-rendering post)

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
        let name = os_name.to_str();

        if name.is_none() || !allow_name(name.unwrap()) {
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

    // consider path modified if it has a different file count
    Ok(file_count != count)
}
