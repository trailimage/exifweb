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
pub use photo::{Camera, ExposureMode, Location, Photo};
pub use post::Post;
pub use tools::{has_ext, min_date, os_text, replace_pairs, slugify, Pairs};
pub use xmp::*;

use chrono::{DateTime, Local, TimeZone};
use exif::{Context, Exif, In, Tag, Value};
use lazy_static::*;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::{error, fmt, fs};

use toml;

static CONFIG_FILE: &str = "config.toml";

/// Patterns to extract position information from the names of photos and post
/// folders in a series
struct Match {
    series_index: Regex,
    photo_index: Regex,
}

// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug, Clone)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn main() {
    // GitHub pages feature requires root at / or /docs
    let root = Path::new("./docs/");
    let config = match load_config::<BlogConfig>(root) {
        Ok(config) => config,
        Err(e) => {
            println!("Missing root configuration file");
            return;
        }
    };

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Unable to read root directory {:?}", e);
            return;
        }
    };

    let mut blog = Blog::default();
    // It is apparently tricky to have Serde automatically deserialize these
    // to Regex instances so instead do it manually
    let matcher = Match {
        series_index: Regex::new(&config.series_index_pattern).unwrap(),
        photo_index: Regex::new(&config.photo.index_pattern).unwrap(),
    };

    // iterate over every file or subdirectory
    for entry in entries {
        let path: PathBuf = entry.unwrap().path();

        if !path.is_dir() {
            // ignore root files
            continue;
        }

        let sub_entries = match (fs::read_dir(path)) {
            Ok(entries) => entries,
            Err(e) => {
                // log error and loop to next path entry
                println!("Unable to read subdirectory {:?}", e);
                continue;
            }
        };

        for entry in sub_entries {
            let path: PathBuf = entry.unwrap().path();

            if let Some(posts) = load_series(&path, &matcher) {
                println!(
                    "Found {} series posts in {}",
                    posts.len(),
                    os_text(path.file_name())
                );
                for p in posts {
                    println!("   {} ({} photos)", &p.key, &p.photos.len());
                    blog.add_post(p);
                }
                continue;
            }

            match load_post(path.as_path(), &matcher) {
                Ok(post) => blog.add_post(post),
                Err(e) => println!("{}", e),
            }

            // if let Ok(post) = load_post(path.as_path(), &matcher) {
            //     blog.add_post(post)
            // }
        }
    }

    println!("Found {} total posts", blog.posts.len());

    blog.correlate_posts();
    blog.sanitize_exif(&config.photo.exif);
}

/// Attempt to load path contents as if it contains a post series.
fn load_series(path: &Path, re: &Match) -> Option<Vec<Post>> {
    let sub_dirs: Vec<PathBuf> = fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();

    if sub_dirs.is_empty() {
        return None;
    }

    if let Ok(config) = load_config::<SeriesConfig>(path) {
        let series_posts: Vec<Post> = sub_dirs
            .iter()
            .map(|p| load_series_post(p.as_path(), &config, re))
            .filter(|p| p.is_ok())
            // .map(|p| p.unwrap())
            .collect();

        return Some(series_posts);
    }

    match path.to_str() {
        Some(dir) => println!("{} has no {}", dir, CONFIG_FILE),
        None => println!("Path has no {}", CONFIG_FILE),
    }
    None
}

/// Create post that is part of a series. This differs from non-series post
/// creation with the addition of several fields that identify how the post
/// relates to the series.
fn load_series_post(
    path: &Path,
    series_config: &SeriesConfig,
    re: &Match,
) -> Result<Post, LoadError> {
    load_config::<PostConfig>(&path).map(|c| {
        // name of series sub-folder used to infer position of post in series
        let dir = os_text(path.file_name());
        let caps = re.series_index.captures(dir).unwrap();
        let part: u8 = caps[1].parse().unwrap();

        Post {
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
            photos: load_photos(path, re, c.cover_photo_index),
            ..Post::default()
        }
    })
}

/// Create post that is not part of a series.
fn load_post(path: &Path, re: &Match) -> Result<Post, LoadError> {
    load_config::<PostConfig>(&path).map(|c| Post {
        key: slugify(&c.title),
        title: c.title,
        summary: c.summary,
        photos: load_photos(path, re, c.cover_photo_index),
        ..Post::default()
    })
}

/// Load configuration from file in given path.
///
/// *See* https://gitter.im/rust-lang/rust/archives/2018/09/07
fn load_config<D: DeserializeOwned>(path: &Path) -> Result<D, LoadError> {
    fs::read_to_string(path.join(CONFIG_FILE))
        .map_err(|_| LoadError)
        .and_then(|s| toml::from_str::<D>(&s).map_err(|_| LoadError))
}

/// Load information about each post photo.
fn load_photos(path: &Path, re: &Match, cover_photo_index: u8) -> Vec<Photo> {
    fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| !p.is_dir() && has_ext(p, "png"))
        .map(|p| load_photo(&p, re, cover_photo_index))
        .collect()
}

fn load_photo(path: &Path, re: &Match, cover_photo_index: u8) -> Photo {
    let file = fs::File::open(path).unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let caps = re.photo_index.captures(file_name).unwrap();
    let index: u8 = caps[1].parse().unwrap();

    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();

    let tags: Vec<String> = Vec::new();

    // https://docs.rs/kamadak-exif/0.5.1/exif/struct.Tag.html
    // https://exiftool.org/TagNames/EXIF.html
    let exif_data = Camera {
        //artist: exif_text(&exif, Tag::Artist),
        name: format!(
            "{} {}",
            exif_text(&exif, Tag::Make),
            exif_text(&exif, Tag::Model)
        )
        .trim()
        .to_owned(),
        compensation: exif_text(&exif, Tag::ExposureBiasValue),
        shutter_speed: exif_text(&exif, Tag::ExposureTime),
        aperture: exif_text(&exif, Tag::FNumber),
        focal_length: exif_f64(&exif, Tag::FocalLength),
        iso: exif_uint(&exif, Tag::PhotographicSensitivity),
        mode: match exif_uint(&exif, Tag::ExposureProgram) {
            1 => ExposureMode::Manual,
            2 => ExposureMode::ProgramAE,
            3 => ExposureMode::AperturePriority,
            4 => ExposureMode::ShutterPriority,
            5 => ExposureMode::Creative,
            6 => ExposureMode::Action,
            7 => ExposureMode::Portrait,
            8 => ExposureMode::Landscape,
            9 => ExposureMode::Bulb,
            _ => ExposureMode::Undefined,
        },
        lens: format!(
            "{} {}",
            exif_text(&exif, Tag::LensMake),
            exif_text(&exif, Tag::LensModel)
        )
        .trim()
        .to_owned(),
        //software: exif_text(&exif, Tag::Software),
        //sanitized: false,
    };

    //println!("{:?}", exif_data);
    println!("{}", exif_text(&exif, Tag::ImageDescription));

    // for f in exif.fields() {
    //     println!(
    //         "  {}/{}: {}",
    //         f.ifd_num.index(),
    //         f.tag,
    //         f.display_value().with_unit(&exif)
    //     );
    //     //println!("      {:?}", f.value);
    // }

    Photo {
        name: file_name.to_owned(),
        title: exif_text(&exif, Tag(Context::Tiff, 0x9c9b)),
        caption: exif_text(&exif, Tag::ImageDescription),
        camera: exif_data,
        index,
        primary: index == cover_photo_index,
        location: Location {
            longitude: exif_f64(&exif, Tag::GPSLongitude),
            latitude: exif_f64(&exif, Tag::GPSLatitude),
        },
        tags,
        date_taken: exif_date(&exif, Tag::DateTimeOriginal),
        ..Photo::default()
    }
}

// https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L64
fn exif_f64(exif: &Exif, tag: Tag) -> f64 {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        return match field.value {
            Value::Rational(ref vec) if !vec.is_empty() => vec[0].to_f64(),
            _ => 0.0,
        };
    }
    0.0
}

// https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L56
fn exif_uint(exif: &Exif, tag: Tag) -> u32 {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        if let Some(value) = field.value.get_uint(0) {
            return value;
        }
    }
    0
}

// https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs#L73
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
    lazy_static! {
        static ref QUOTES: Regex =
            Regex::new(r#"(^\s*"\s*|\s*"\s*$)"#).unwrap();
    }

    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        return QUOTES
            .replace_all(&field.display_value().with_unit(exif).to_string(), "")
            .into_owned();
    }
    String::new()
}
