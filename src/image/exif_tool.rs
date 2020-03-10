//! Use ExifTool to extract photo metadata

use crate::image::deserialize::{string_number, string_sequence};
use crate::photo::{Camera, ExposureMode, Photo};
use crate::{path_name, pos_from_name, tab};
use colored::*;
use regex::Regex;
use serde::Deserialize;
use serde_json;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
pub struct ExifToolOutput {
    #[serde(rename = "FileName")]
    file_name: String,

    #[serde(rename = "Artist")]
    artist: String,

    #[serde(rename = "Title")] // or ObjectName
    title: Option<String>,

    #[serde(rename = "Description")] // or Caption-Abstract or ImageDescription
    caption: Option<String>,

    // or Subject
    #[serde(
        default,
        rename = "Keywords",
        deserialize_with = "string_sequence"
    )]
    tags: Vec<String>,

    #[serde(rename = "City")]
    city: Option<String>,

    #[serde(rename = "State")] // or Province-State
    state: Option<String>,

    #[serde(rename = "Copyright")] // or Rights or CopyrightNotice
    copyright: String,

    #[serde(rename = "UsageTerms")]
    usage_terms: String,

    #[serde(rename = "Software")] // or CreatorTool
    software: String,

    #[serde(rename = "Aperture")] // or FNumber
    aperture: Option<f32>,

    #[serde(rename = "ISO")]
    iso: Option<u16>,

    // or ShutterSpeedValue
    #[serde(rename = "ShutterSpeed", deserialize_with = "string_number")]
    shutter_speed: String,

    #[serde(
        rename = "ExposureCompensation",
        deserialize_with = "string_number"
    )]
    exposure_compensation: String,

    #[serde(default, rename = "ExposureProgram")]
    exposure_mode: ExposureMode,

    #[serde(rename = "FocalLength")]
    focal_length: Option<u16>,

    #[serde(rename = "MaxApertureValue")]
    max_aperture: Option<f32>,

    #[serde(rename = "Lens")]
    lens: Option<String>,

    #[serde(rename = "Make")]
    camera_make: Option<String>,

    #[serde(default, rename = "Model")]
    camera_model: Option<String>,

    // TODO: convert to date
    #[serde(default, rename = "DateTimeCreated")] // or DateTimeOriginal
    taken_on: String,

    #[serde(rename = "GPSLatitude")]
    latitude: Option<f32>,

    #[serde(rename = "GPSLongitude")]
    longitude: Option<f32>,

    #[serde(default, rename = "ProfileDescription")]
    color_profile: Option<String>,

    #[serde(rename = "ColorTemperature")]
    color_temperature: Option<u16>,

    #[serde(rename = "FOV")]
    field_of_view: Option<f32>,

    #[serde(rename = "ImageWidth")]
    width: u16,

    #[serde(rename = "ImageHeight")]
    height: u16,
}

pub fn parse_dir(
    path: &Path,
    cover_index: u8,
    infer_pos: &Regex,
) -> Vec<Photo> {
    read_dir(&path)
        .iter()
        .map(|i| {
            let index = pos_from_name(&infer_pos, &i.file_name).unwrap_or(0);

            if index == 0 {
                println!(
                    "{:tab$}{} {}",
                    "",
                    "failed to infer index of".red(),
                    i.file_name.red(),
                    tab = tab(1)
                );
                return None;
            }

            let mut photo = Photo {
                name: i.file_name.to_owned(),
                title: i.title.to_owned(),
                artist: i.artist.to_owned(),
                caption: i.caption.to_owned(),
                software: i.software.to_owned(),
                tags: i.tags.to_owned(),
                index,
                primary: index == cover_index,
                ..Photo::default()
            };

            if let Some(make) = &i.camera_make {
                let name = match &i.camera_model {
                    Some(model) => format!("{} {}", make, model),
                    _ => make.clone(),
                };

                let camera = Camera {
                    name,
                    // TODO: allow this to be optional by updating custom deserializer
                    // https://users.rust-lang.org/t/serde-handling-null-in-custom-deserializer/18191/2
                    compensation: Some(i.exposure_compensation.to_owned()),
                    // also this
                    shutter_speed: Some(i.shutter_speed.to_owned()),
                    mode: i.exposure_mode,
                    aperture: i.aperture,
                    focal_length: i.focal_length,
                    iso: i.iso,
                    lens: i.lens.to_owned(),
                    ..Camera::default()
                };

                photo.camera = Some(camera);
            }

            Some(photo)
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect()
}

pub fn read_dir(path: &Path) -> Vec<ExifToolOutput> {
    // exiftool *.tif -json -quiet -coordFormat %.6f
    // exiftool 002.tif -json -quiet -coordFormat %.6f -ExposureProgram#
    // exiftool *.tif -json -quiet -Aperture# -ColorTemperature# -DateTimeCreated -FocalLength# -FOV# -Keywords# -ShutterSpeed

    // suffix field name with # to disable ExifTool formatting
    let output = match Command::new("exiftool")
        .current_dir(path.to_string_lossy().to_string())
        .arg("*.tif")
        .arg("-json")
        .arg("-quiet")
        .arg("-Aperture#")
        .arg("-Artist")
        .arg("-City")
        .arg("-ColorTemperature#")
        .arg("-Copyright")
        .arg("-DateTimeCreated")
        .arg("-Description")
        .arg("-ExposureCompensation")
        .arg("-ExposureProgram#")
        .arg("-FileName")
        .arg("-FocalLength#")
        .arg("-FOV#")
        .arg("-GPSLatitude#")
        .arg("-GPSLongitude#")
        .arg("-ImageHeight")
        .arg("-ImageWidth")
        .arg("-ISO")
        .arg("-Keywords")
        .arg("-Lens")
        .arg("-Make")
        .arg("-MaxApertureValue")
        .arg("-Model")
        .arg("-ProfileDescription")
        .arg("-ShutterSpeed")
        .arg("-Software")
        .arg("-State")
        .arg("-Title")
        .arg("-UsageTerms")
        .output()
    {
        Ok(out) => out,
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "failed to retrieve EXIF for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            return Vec::new();
        }
    };

    let text = match String::from_utf8(output.stdout) {
        Ok(text) => text,
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "Failed to convert EXIF output to UTF-8 for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            return Vec::new();
        }
    };

    if text.is_empty() {
        println!(
            "{} {}",
            "EXIF JSON is empty for".red(),
            path_name(&path).magenta()
        );
        return Vec::new();
    }

    match serde_json::from_str::<Vec<ExifToolOutput>>(&text) {
        Ok(info) => info,
        Err(e) => {
            println!(
                "{:tab$}{} {}",
                "",
                "unable to parse EXIF JSON for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            //println!("{}", text);
            println!("—\n{:?}\n—", e);
            Vec::new()
        }
    }
}
