use crate::{
    config::{
        BlogConfig, PhotoConfig, PostConfig, PostLog, SeriesConfig, CONFIG_FILE,
    },
    image::exif_tool,
    models::{collate_tags, Blog, Photo, Post, PostSeries},
    tools::{earliest_photo_date, folder_name, identify_outliers, path_slice},
};
use colored::*;
use std::{
    self,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    process,
    time::UNIX_EPOCH,
};

/// Create post that is not part of a series
pub fn post(path: &Path, config: &BlogConfig) -> Option<Post> {
    PostConfig::load(&path)
        .and_then(|post_config| create_post(path, false, config, post_config))
}

/// Attempt to load path entries as if they constitute a post series. `None` is
/// returned if there are no subdirectories or they don't contain valid posts.
pub fn series(path: &Path, config: &BlogConfig) -> Option<Vec<Post>> {
    let sub_dirs: Vec<PathBuf> = match fs::read_dir(&path) {
        Ok(entries) => entries
            .map(|e: std::io::Result<DirEntry>| e.unwrap().path())
            .filter(|p| p.is_dir())
            .collect(),
        _ => {
            println!(
                "   {} {}",
                "Failed to open subdirectory".red(),
                folder_name(&path).red().bold()
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
                    series_post(p.as_path(), config, &series_config)
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
fn series_post(
    path: &Path,
    config: &BlogConfig,
    series_config: &SeriesConfig,
) -> Option<Post> {
    PostConfig::load(&path).and_then(|post_config| {
        let part = post_config.part;

        if part == 0 {
            return None;
        } else if part > series_config.parts {
            println!(
                "Post {} part {} is more than should be in series ({})",
                folder_name(path),
                part,
                series_config.parts
            );
            process::exit(1);
        }

        create_post(path, true, config, post_config).and_then(|mut p| {
            p.series = Some(PostSeries {
                part,
                title: series_config.title.clone(),
                path: series_config.path.clone(),
                part_path: path_slice(path, 1),
                total_parts: series_config.parts,
                prev_is_part: part > 1,
                next_is_part: part < series_config.parts,
            });

            Some(p)
        })
    })
}

/// Load photos for all posts with given `paths`. This may be used to populate
/// posts initially created from a log file but then found to have changed
/// sequence (different next or previous post), requiring a re-render which
/// depends on complete photo information.
pub fn post_photos(
    root: &Path,
    config: &BlogConfig,
    blog: &mut Blog,
    paths: &Vec<String>,
) {
    for path in paths.iter() {
        println!("   Attempting to add photos to {}", path);

        let mut photos = load_photos(&root.join(path), &config.photo);

        blog.add_post_photos(path, &mut photos)
    }
}

/// Load information about each photo in `path`
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

/// Load basic post data from previous render log or by reading photo files.
///
/// If the post is loaded from the log, that implies there were no changes and
/// its photos won't be loaded, leaving the `photos` field will be empty.
///
/// If there is no log or photos, `None` will be returned.
fn create_post(
    path: &Path,
    is_series: bool,
    config: &BlogConfig,
    post_config: PostConfig,
) -> Option<Post> {
    // path to series post includes parent
    let post_path = path_slice(path, if is_series { 2 } else { 1 });
    let log = load_post_log(path, config);

    if !(log.files_have_changed || config.force_rerender) {
        // no files have changed and re-render NOT forced
        assert_index(
            post_config.cover_photo_index,
            log.photo_count,
            &post_config.title,
        );

        Some(Post {
            path: post_path,
            happened_on: log.happened_on,
            photo_count: log.photo_count,
            photo_locations: log.photo_locations.clone(),
            needs_render: false,
            tags: log.tags.clone(),
            ..Post::from_config(post_config, log)
        })
    } else {
        let photos = load_photos(path, &config.photo);

        if photos.is_empty() {
            None
        } else {
            assert_index(
                post_config.cover_photo_index,
                photos.len(),
                &post_config.title,
            );

            let mut photo_locations: Vec<(f32, f32)> = Vec::new();

            for p in &photos {
                if let Some(l) = &p.location {
                    photo_locations.push(l.as_tuple());
                }
            }

            Some(Post {
                path: post_path,
                tags: collate_tags(&photos),
                happened_on: if post_config.chronological {
                    earliest_photo_date(&photos)
                } else {
                    None
                },
                photo_count: photos.len(),
                photo_locations,
                photos,
                ..Post::from_config(post_config, log)
            })
        }
    }
}

/// - `index` One-based
fn assert_index(index: usize, length: usize, label: &str) {
    if index > length {
        println!(
            "\n   {}",
            format!("Index {} exceeded {} for {}", index, length, label).red()
        );

        process::exit(1);
    }
}

/// Load post log. If there is no file then return a log with
/// `files_have_changed = true`.
fn load_post_log(path: &Path, config: &BlogConfig) -> PostLog {
    PostLog::load(path).map_or(PostLog::empty(), |mut log| {
        match is_modified(
            path,
            log.as_of,
            log.photo_count + 1, // photos plus configuration file
            config,
        ) {
            Ok(modified) => {
                log.files_have_changed = modified;
                return log;
            }
            Err(e) => {
                println!(
                    "   Failed to check {} for change {:?}",
                    folder_name(path),
                    e
                );
                PostLog::empty()
            }
        }
    })
}

/// Whether `path` contains any pertinent files modified after `threshold`
/// timestamp.
///
/// If `true` then photos will be loaded and a subsequent check will determine
/// whether particular photos have been modified since the `threshold`.
fn is_modified(
    path: &Path,
    threshold: i64,
    file_count: usize,
    config: &BlogConfig,
) -> std::io::Result<bool> {
    if !path.is_dir() {
        return Ok(true);
    }
    let mut count: usize = 0;

    let allow_name = |name: &str| {
        name.ends_with(&config.photo.source_ext) || name == CONFIG_FILE
    };

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

    // consider path to be modified if it has a different file count
    Ok(file_count != count)
}
