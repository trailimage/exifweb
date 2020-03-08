mod blog;
mod caption;
mod category;
mod config;
mod image;
mod photo;
mod post;
mod tools;
mod xmp;

pub use blog::Blog;
pub use caption::Caption;
pub use category::Category;
pub use config::*;
use image::read_dir_exif;
pub use photo::{Camera, ExposureMode, Location, Photo};
pub use post::Post;
pub use tools::{
    has_ext, min_date, path_name, pos_from_name, pos_from_path, replace_pairs,
    slugify, tab, LoadError, Pairs,
};
pub use xmp::*;

use chrono::{DateTime, Local, TimeZone};
use colored::*;
use exif::{Context, Exif, In, Tag, Value};
use lazy_static::*;
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

/// Load configuration from file in given path.
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

/// Load information about each post photo.
fn load_photos(path: &Path, re: &Match, cover_photo_index: u8) -> Vec<Photo> {
    let photos: Vec<Photo> = match read_dir_exif(&path) {
        Some(info) => info
            .iter()
            .map(|i| {
                let index =
                    pos_from_name(&re.photo, &i.image.file_name).unwrap_or(0);

                if index == 0 {
                    println!(
                        "{:tab$}{} {}",
                        "",
                        "failed to infer index of".red(),
                        i.image.file_name.red(),
                        tab = tab(1)
                    );
                    return None;
                }

                Some(Photo {
                    name: i.image.file_name.to_owned(),
                    //title: exif_text(&exif, Tag(Context::Tiff, 0x9c9b)),
                    //         caption: exif_text(&exif, Tag::ImageDescription),
                    //         camera: exif_data,
                    index,
                    primary: index == cover_photo_index,
                    //         location: Location {
                    //             longitude: exif_f64(&exif, Tag::GPSLongitude),
                    //             latitude: exif_f64(&exif, Tag::GPSLatitude),
                    //         },
                    //         tags,
                    //         date_taken: exif_date(&exif, Tag::DateTimeOriginal),
                    ..Photo::default()
                })
            })
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect(),
        _ => return Vec::new(),
    };

    if photos.is_empty() {
        println!("{:tab$}{}", "", "found no photos".red(), tab = tab(1));
    }
    photos
}

// fn load_photo(path: &Path, re: &Match, cover_photo_index: u8) -> Option<Photo> {
//     let file = match fs::File::open(path) {
//         Ok(f) => f,
//         _ => {
//             println!(
//                 "{:tab$}{} {}",
//                 "failed to open".red(),
//                 path_name(&path).red(),
//                 tab = tab(2)
//             );
//             return None;
//         }
//     };

//     let index = pos_from_path(&re.photo, &path).unwrap_or(0);

//     if index == 0 {
//         println!(
//             "{:tab$}{} {}",
//             "failed to infer index of".red(),
//             path_name(&path).red(),
//             tab = tab(2)
//         );
//         return None;
//     }

//     let mut bufreader = std::io::BufReader::new(&file);
//     let exifreader = exif::Reader::new();
//     let exif = exifreader.read_from_container(&mut bufreader).unwrap();

//     let tags: Vec<String> = Vec::new();

//     // https://docs.rs/kamadak-exif/0.5.1/exif/struct.Tag.html
//     // https://exiftool.org/TagNames/EXIF.html
//     let exif_data = Camera {
//         //artist: exif_text(&exif, Tag::Artist),
//         name: format!(
//             "{} {}",
//             exif_text(&exif, Tag::Make),
//             exif_text(&exif, Tag::Model)
//         )
//         .trim()
//         .to_owned(),
//         compensation: exif_text(&exif, Tag::ExposureBiasValue),
//         shutter_speed: exif_text(&exif, Tag::ExposureTime),
//         aperture: exif_text(&exif, Tag::FNumber),
//         focal_length: exif_f64(&exif, Tag::FocalLength),
//         iso: exif_uint(&exif, Tag::PhotographicSensitivity),
//         mode: match exif_uint(&exif, Tag::ExposureProgram) {
//             1 => ExposureMode::Manual,
//             2 => ExposureMode::ProgramAE,
//             3 => ExposureMode::AperturePriority,
//             4 => ExposureMode::ShutterPriority,
//             5 => ExposureMode::Creative,
//             6 => ExposureMode::Action,
//             7 => ExposureMode::Portrait,
//             8 => ExposureMode::Landscape,
//             9 => ExposureMode::Bulb,
//             _ => ExposureMode::Undefined,
//         },
//         lens: format!(
//             "{} {}",
//             exif_text(&exif, Tag::LensMake),
//             exif_text(&exif, Tag::LensModel)
//         )
//         .trim()
//         .to_owned(),
//         //software: exif_text(&exif, Tag::Software),
//         //sanitized: false,
//     };

//     //println!("{:?}", exif_data);
//     println!("{}", exif_text(&exif, Tag::ImageDescription));

//     // for f in exif.fields() {
//     //     println!(
//     //         "  {}/{}: {}",
//     //         f.ifd_num.index(),
//     //         f.tag,
//     //         f.display_value().with_unit(&exif)
//     //     );
//     //     //println!("      {:?}", f.value);
//     // }

//     Photo {
//         name: path_name(&path).to_owned(),
//         title: exif_text(&exif, Tag(Context::Tiff, 0x9c9b)),
//         caption: exif_text(&exif, Tag::ImageDescription),
//         camera: exif_data,
//         index,
//         primary: index == cover_photo_index,
//         location: Location {
//             longitude: exif_f64(&exif, Tag::GPSLongitude),
//             latitude: exif_f64(&exif, Tag::GPSLatitude),
//         },
//         tags,
//         date_taken: exif_date(&exif, Tag::DateTimeOriginal),
//         ..Photo::default()
//     }
// }

// // https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L64
// fn exif_f64(exif: &Exif, tag: Tag) -> f64 {
//     if let Some(field) = exif.get_field(tag, In::PRIMARY) {
//         return match field.value {
//             Value::Rational(ref vec) if !vec.is_empty() => vec[0].to_f64(),
//             _ => 0.0,
//         };
//     }
//     0.0
// }

// // https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L56
// fn exif_uint(exif: &Exif, tag: Tag) -> u32 {
//     if let Some(field) = exif.get_field(tag, In::PRIMARY) {
//         if let Some(value) = field.value.get_uint(0) {
//             return value;
//         }
//     }
//     0
// }

// // https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L73
// fn exif_date(exif: &Exif, tag: Tag) -> DateTime<Local> {
//     if let Some(field) = exif.get_field(tag, In::PRIMARY) {
//         return match field.value {
//             Value::Ascii(ref vec) if !vec.is_empty() => {
//                 if let Ok(dt) = exif::DateTime::from_ascii(&vec[0]) {
//                     Local
//                         .ymd(dt.year as i32, dt.month as u32, dt.day as u32)
//                         .and_hms(
//                             dt.hour as u32,
//                             dt.minute as u32,
//                             dt.second as u32,
//                         )
//                 } else {
//                     min_date()
//                 }
//             }
//             _ => min_date(),
//         };
//     }
//     min_date()
// }

// fn exif_text(exif: &Exif, tag: Tag) -> String {
//     lazy_static! {
//         static ref QUOTES: Regex =
//             Regex::new(r#"(^\s*"\s*|\s*"\s*$)"#).unwrap();
//     }

//     if let Some(field) = exif.get_field(tag, In::PRIMARY) {
//         return QUOTES
//             .replace_all(&field.display_value().with_unit(exif).to_string(), "")
//             .into_owned();
//     }
//     String::new()
// }
