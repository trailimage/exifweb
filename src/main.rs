mod blog;
mod caption;
mod category;
mod config;
mod photo;
mod post;
mod tools;

pub use blog::Blog;
pub use caption::Caption;
pub use category::Category;
pub use config::*;
pub use photo::{Location, Photo, EXIF};
pub use post::Post;
pub use tools::{min_date, slugify};

use chrono::{DateTime, Local, TimeZone};
use exif::{Exif, In, Tag, Value};
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
                        println!("   {}", &p.key);
                        blog.add_post(p);
                    }
                    continue;
                }
                None => blog.add_post(load_post(path.as_path(), &matcher)),
            }
        }
    }

    println!("Found {} total posts", blog.posts.len());

    blog.correlate_posts()
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
        key: format!(
            "{}/{}",
            slugify(&series_config.title),
            slugify(&post_config.title)
        ),
        title: series_config.title.clone(),
        sub_title: post_config.title,
        summary: post_config.summary,
        part,
        is_partial: true,
        total_parts: series_config.parts,
        prev_is_part: part > 1,
        next_is_part: part < series_config.parts,
        photos: load_photos(path, re),
        ..Post::default()
    }
}

/// Create post that is not part of a series.
fn load_post<'a>(path: &Path, re: &Match) -> Post<'a> {
    let config: PostConfig = load_config(&path);
    Post {
        key: slugify(&config.title),
        title: config.title,
        summary: config.summary,
        photos: load_photos(path, re),
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

/// Load information about each post photo.
fn load_photos(path: &Path, re: &Match) -> Vec<Photo> {
    fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| !p.is_dir() && has_ext(p, "jpg"))
        .map(|p| load_photo(&p, re))
        .collect()
}

fn load_photo(path: &Path, re: &Match) -> Photo {
    let file = fs::File::open(path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let caps = re.photo_index.captures(file_name).unwrap();
    let index: u8 = caps[1].parse().unwrap();

    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();

    let tags: Vec<String> = Vec::new();

    // https://docs.rs/kamadak-exif/0.5.1/exif/struct.Tag.html
    let exif_data = EXIF {
        artist: exif_text(&exif, Tag::Artist),
        camera: format!(
            "{} {}",
            exif_text(&exif, Tag::Make),
            exif_text(&exif, Tag::Model)
        ),
        compensation: exif_text(&exif, Tag::Model),
        exposure: exif_text(&exif, Tag::ExposureTime),
        f_number: exif_uint(&exif, Tag::FNumber),
        focal_length: exif_uint(&exif, Tag::FocalLength),
        iso: exif_uint(&exif, Tag::ISOSpeed),
        lens: format!(
            "{} {}",
            exif_text(&exif, Tag::LensMake),
            exif_text(&exif, Tag::LensModel)
        ),
        software: exif_text(&exif, Tag::Software),
        sanitized: false,
    };

    //exif_data.sanitize(

    Photo {
        name: file_name.to_owned(),
        exif: exif_data,
        index,
        location: Location {
            longitude: exif_f64(&exif, Tag::GPSLongitude),
            latitude: exif_f64(&exif, Tag::GPSLatitude),
        },
        tags,
        date_taken: exif_date(&exif, Tag::DateTimeOriginal),
        ..Photo::default()
    }
}

fn exif_f64(exif: &Exif, tag: Tag) -> f64 {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        return match field.value {
            Value::Rational(ref vec) if !vec.is_empty() => vec[0].to_f64(),
            _ => 0.0,
        };
    }
    0.0
}

fn exif_uint(exif: &Exif, tag: Tag) -> u32 {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        if let Some(value) = field.value.get_uint(0) {
            return value;
        }
    }
    0
}

fn exif_date(exif: &Exif, tag: Tag) -> DateTime<Local> {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        return match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(dt) = exif::DateTime::from_ascii(&vec[0]) {
                    Local
                        .ymd(dt.year as i32, dt.month as u32, dt.day as u32)
                        .and_hms(
                            dt.hour as u32,
                            dt.minute as u32,
                            dt.second as u32,
                        )
                } else {
                    min_date()
                }
            }
            _ => min_date(),
        };
    }
    min_date()
}

fn exif_text(exif: &Exif, tag: Tag) -> String {
    match exif.get_field(tag, In::PRIMARY) {
        Some(f) => f.display_value().to_string(),
        None => String::new(),
    }
}

/// Whether path ends with an extension.
fn has_ext(p: &PathBuf, ext: &str) -> bool {
    p.file_name().unwrap().to_str().unwrap().ends_with(ext)
}

struct Match {
    series_index: Regex,
    photo_index: Regex,
}
