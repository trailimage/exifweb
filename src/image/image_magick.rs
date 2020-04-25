use crate::{
    config::PhotoConfig,
    html,
    models::{
        suffix, Camera, ExposureMode, Location, Photo, PhotoFile,
        SizeCollection,
    },
    tools::{folder_name, pos_from_name},
};
use chrono::{DateTime, FixedOffset};
use colored::*;
use encoding::{all::*, DecoderTrap, Encoding};
use serde::Deserialize;
use std::{
    mem,
    path::Path,
    process::{self, Command},
};

/// Create vector of owned strings
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// magick convert *.tif -quiet
//  ( +clone -resize 2048x -write *-large.webp +delete )
//  ( +clone -resize 1024x -write *-regular.webp +delete )
//  ( +clone -resize 320x -write *-small.webp +delete )
//  -resize 256x256^ -gravity center -extent 256x256 001-thumb.webp

/// https://imagemagick.org/script/webp.php
pub fn create_sizes(path: &str, photo: &Photo, config: &PhotoConfig) {
    // Generate file name from photo index and configured extenstion
    let file_name = |suffix: &'static str| {
        format!("{:03}_{}{}", photo.index, suffix, config.output_ext)
    };

    let sub_command = |size: u16, suffix: &'static str| {
        string_vec![
            "(",
            "+clone",
            "-resize",
            format!("{}x", size),
            "-write",
            file_name(suffix),
            "+delete",
            ")"
        ]
    };

    let thumb_command = string_vec![
        "-resize",
        format!("{px}x{px}^", px = config.size.thumb),
        "-gravity",
        "center",
        "-extent",
        format!("{px}x{px}", px = config.size.thumb),
        file_name(suffix::THUMB)
    ];

    match Command::new("magick")
        .current_dir(path)
        .arg("convert")
        .arg(&photo.file.name)
        .arg("-quiet")
        .args(sub_command(config.size.large, suffix::LARGE))
        .args(sub_command(config.size.medium, suffix::MEDIUM))
        .args(sub_command(config.size.small, suffix::SMALL))
        .args(&thumb_command)
        .output()
    {
        Ok(out) => {
            if let Ok(err) = String::from_utf8(out.stderr) {
                if !err.is_empty() {
                    println!(
                        "Running\nmagick convert {} -quiet {} {} {} {}\n",
                        &photo.file.name,
                        sub_command(config.size.large, suffix::LARGE).join(" "),
                        sub_command(config.size.medium, suffix::MEDIUM)
                            .join(" "),
                        sub_command(config.size.small, suffix::SMALL).join(" "),
                        thumb_command.join(" ")
                    );
                    println!("{}", err.red());
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("   {:?}", e);
            process::exit(1);
        }
    };
}

// https://www.awaresystems.be/imaging/tiff/tifftags/private.html
#[derive(Deserialize, Debug)]
struct ImageProperties {
    #[serde(rename = "exif:ApertureValue")] // or exif:FNumber
    aperture: String,

    #[serde(rename = "exif:artist")]
    artist: Option<String>,

    #[serde(rename = "exif:Make")]
    camera_make: Option<String>,

    #[serde(rename = "exif:model")]
    camera_model: Option<String>,

    #[serde(rename = "exif:ImageDescription")]
    caption: Option<String>,

    //city: Option<String>,
    #[serde(rename = "icc:description")]
    color_profile: Option<String>,

    color_temperature: Option<String>,

    #[serde(rename = "exif:copyright")]
    copyright: String,

    /// When the *photo*, not the file, was created
    #[serde(rename = "date:create")]
    created_on: Option<DateTime<FixedOffset>>,

    #[serde(rename = "exif:ExposureBiasValue")]
    exposure_compensation: Option<String>,

    #[serde(rename = "exif:FocalLengthIn35mmFilm")]
    field_of_view: Option<f32>,

    /// When the *file*, not the photo, was created
    file_created_on: Option<DateTime<FixedOffset>>,

    file_name: String,

    #[serde(rename = "exif:FocalLength")]
    focal_length: Option<f32>,

    #[serde(rename = "exif:LensModel")]
    iso: Option<u16>,

    #[serde(rename = "exif:GPSLatitude")]
    latitude: Option<f32>,

    #[serde(rename = "exif:PhotographicSensitivity")]
    lens: Option<String>,

    #[serde(rename = "exif:GPSLongitude")]
    longitude: Option<f32>,

    #[serde(rename = "exif:MaxApertureValue")]
    max_aperture: Option<f32>,

    #[serde(rename = "exif:Software")]
    software: String,

    // state: Option<String>,

    //tags: Vec<String>,

    //title: Option<String>,
    usage_terms: Option<String>,

    #[serde(rename = "exif:ExposureTime")] // or ShutterSpeedValue
    shutter_speed: Option<String>,

    #[serde(rename = "exif:DateTimeOriginal")]
    taken_on: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize, Debug)]
struct GeometryFields {
    width: u16,
    height: u16,
}

#[derive(Deserialize, Debug)]
struct ProfileFields {
    iptc: Option<IptcFields>,
}

/// IPTC field values are all string arrays even if they should have only a
/// single element
#[derive(Deserialize, Debug)]
struct IptcFields {
    #[serde(rename = "Caption[2,120]")]
    caption: Vec<String>,

    #[serde(rename = "Keyword[2,25]")]
    tags: Vec<String>,

    #[serde(rename = "Image Name[2,5]")]
    title: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ImageFields {
    #[serde(rename = "baseName")]
    file_name: String,
    format: String,
    properties: ImageProperties,
    geometry: GeometryFields,
    profiles: ProfileFields,
}
#[derive(Deserialize, Debug)]
struct ImageMagickInfo {
    pub image: ImageFields,
}

pub fn parse_dir(path: &Path, config: &PhotoConfig) -> Vec<Photo> {
    let mut photos: Vec<Photo> = Vec::new();

    for i in read_dir(&path) {
        let i = i.image;

        let index =
            pos_from_name(&config.capture_index, &i.file_name).unwrap_or(0);

        if index == 0 {
            println!(
                "   {} {}",
                "failed to infer index of".red(),
                i.file_name.red(),
            );
            continue;
        }

        let p = i.properties;
        let iptc = i.profiles.iptc.unwrap();

        let mut photo = Photo {
            file: PhotoFile {
                name: i.file_name.clone(),
                created: 0, // i.file_created_on.map_or(0, |d| d.timestamp()),
            },
            title: None, // mem::replace(&mut iptc.title, None),
            artist: p.artist,
            caption: p.caption.map(|s| html::caption(&s)),
            software: p.software,
            tags: iptc.tags,
            index,
            size: SizeCollection::from(
                i.geometry.width,
                i.geometry.height,
                &config.size,
            ),
            date_taken: p.taken_on.or(p.created_on),
            ..Photo::default()
        };

        if let Some(make) = &p.camera_make {
            let name = match &p.camera_model {
                Some(model) => format!("{} {}", make, model),
                _ => make.clone(),
            };

            let camera = Camera {
                name,
                compensation: p.exposure_compensation,
                shutter_speed: p.shutter_speed,
                mode: ExposureMode::Manual, // p.exposure_mode,
                aperture: None,             // p.aperture,
                focal_length: p.focal_length,
                iso: p.iso,
                lens: p.lens,
            };

            photo.camera = Some(camera);
        }

        if p.latitude.is_some() && p.longitude.is_some() {
            let loc = Location {
                latitude: p.latitude.unwrap(),
                longitude: p.longitude.unwrap(),
            };

            if loc.is_valid() {
                photo.location = Some(loc);
            }
        }

        photos.push(photo);
    }

    photos
}

fn read_dir(path: &Path) -> Vec<ImageMagickInfo> {
    // magick convert -quiet 001.jpg json:
    // magick convert -quiet *.tif xmp:
    let output = match Command::new("magick")
        .current_dir(path.to_string_lossy().to_string())
        .arg("convert")
        .arg("-quiet")
        .arg("*.jpg")
        .arg("json:")
        .output()
    {
        Ok(out) => out,
        _ => {
            println!(
                "   {} {}",
                "Failed to generate EXIF for".red(),
                folder_name(&path).magenta(),
            );
            return Vec::new();
        }
    };

    let text = match ISO_8859_1.decode(&output.stdout[..], DecoderTrap::Ignore)
    {
        Ok(text) => text,
        _ => {
            println!(
                "   {} {}",
                "Failed to convert EXIF output to UTF-8 for".red(),
                folder_name(&path).magenta(),
            );
            return Vec::new();
        }
    };

    if text.is_empty() {
        println!(
            "   {} {}",
            "EXIF JSON is empty for".red(),
            folder_name(&path).magenta()
        );
        return Vec::new();
    }

    match serde_json::from_str::<Vec<ImageMagickInfo>>(&text) {
        Ok(info) => info,
        Err(e) => {
            println!(
                "   {} {}",
                "Unable to parse EXIF JSON for".red(),
                folder_name(&path).magenta(),
            );
            //println!("{}", text);
            //println!("{:?}", e);
            Vec::new()
        }
    }
}
