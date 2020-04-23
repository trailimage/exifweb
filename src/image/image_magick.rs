use crate::config::PhotoConfig;
use crate::models::Photo;
use crate::tools::{folder_name, pos_from_name};
use colored::*;
use encoding::{all::*, DecoderTrap, Encoding};
use serde::Deserialize;
use serde_json;
use std::{path::Path, process::Command};

// https://www.awaresystems.be/imaging/tiff/tifftags/private.html
#[derive(Deserialize, Debug)]
struct ImageProperties {
    #[serde(rename = "tiff:copyright")]
    pub copyright: String,

    #[serde(rename = "exif:ApertureValue")]
    pub aperture: String,

    #[serde(rename = "exif:MaxApertureValue")]
    pub max_aperture: String,

    #[serde(rename = "exif:FocalLength")]
    pub focal_length: String,

    #[serde(rename = "exif:ShutterSpeedValue")]
    pub shutter_speed: String,

    #[serde(rename = "exif:ISOSpeedRatings")]
    pub iso: String,

    #[serde(rename = "exif:ExposureBiasValue")]
    pub compensation: String,

    #[serde(rename = "tiff:artist")]
    pub artist: String,

    #[serde(rename = "tiff:software")]
    pub software: String,

    #[serde(rename = "tiff:make")]
    pub camera_make: String,

    #[serde(rename = "tiff:model")]
    pub camera_model: String,

    #[serde(rename = "date:create")]
    pub date_taken: String,
}

#[derive(Deserialize, Debug)]
struct ImageFields {
    #[serde(rename = "baseName")]
    pub file_name: String,
    pub format: String,
    pub properties: ImageProperties,
}
#[derive(Deserialize, Debug)]
struct ImageMagickInfo {
    pub image: ImageFields,
}

// magick convert *.tif -quiet ( +clone -resize 2048x -write *-large.webp +delete ) ( +clone -resize 1024x -write *-regular.webp +delete ) ( +clone -resize 320x -write *-small.webp +delete ) -resize 256x256^ -gravity center -extent 256x256 001-thumb.webp

// -thumbnail 100x100^ -gravity center -extent 100x100

// magick convert *.tif -define tiff:ignore-tags=42033 -resize 256x256^ -gravity center -extent 256x256 *-thumb.webp

// convert -resize 256x256^ -extent 256x256 in.png out.png

// -define tiff:ignore-tags=42033

/// https://imagemagick.org/script/webp.php
/// TIFF options
/// http://www.imagemagick.org/script/command-line-options.php#define
pub fn resize(path: &Path, config: &PhotoConfig) {
    // magick convert 001.tif -quiet -resize 2048x2048 -define webp:lossless=false -define webp:thread-level=1 001.webp
    let output = match Command::new("magick")
        .current_dir(path.to_string_lossy().to_string())
        .arg("convert")
        .arg("-quality")
        .arg("50")
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
}

// magick convert 001.tif -quiet ( +clone -resize 2048x -write 001-large.webp +delete ) ( +clone -resize 1024x -write 001-regular.webp +delete ) ( +clone -resize 320x -write 001-small.webp +delete ) -resize 256x256^ -gravity center -extent 256x256 001-thumb.webp

pub fn parse_dir(path: &Path, config: &PhotoConfig) -> Vec<Photo> {
    read_dir(&path)
        .iter()
        .filter_map(|i| {
            let index =
                pos_from_name(&config.capture_index, &i.image.file_name)
                    .unwrap_or(0);

            if index == 0 {
                println!(
                    "   {} {}",
                    "failed to infer index of".red(),
                    i.image.file_name.red(),
                );
                return None;
            }

            Some(Photo {
                name: i.image.file_name.to_owned(),
                //sizes: SizeCollection::from(i.width, i.height, &config.size),
                index,
                ..Photo::default()
            })
        })
        .collect()
}

fn read_dir(path: &Path) -> Vec<ImageMagickInfo> {
    // magick convert -quiet *.tif json:
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
                "{:>3} {}",
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
                "{:>3} {}",
                "Failed to convert EXIF output to UTF-8 for".red(),
                folder_name(&path).magenta(),
            );
            return Vec::new();
        }
    };

    if text.is_empty() {
        println!(
            "{} {}",
            "EXIF JSON is empty for".red(),
            folder_name(&path).magenta()
        );
        return Vec::new();
    }

    match serde_json::from_str::<Vec<ImageMagickInfo>>(&text) {
        Ok(info) => info,
        Err(e) => {
            println!(
                "{:>3} {}",
                "Unable to parse EXIF JSON for".red(),
                folder_name(&path).magenta(),
            );
            println!("{}", text);
            println!("{:?}", e);
            Vec::new()
        }
    }
}
