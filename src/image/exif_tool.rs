//! Use ExifTool to extract photo metadata

use crate::{path_name, pos_from_name, tab, Photo};
use colored::*;
use regex::Regex;
use serde::Deserialize;
use serde_json;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
pub struct ExifToolInfo {
    #[serde(rename = "FileName")]
    file_name: String,

    #[serde(rename = "Title")] // or ObjectName
    title: String,

    #[serde(rename = "Description")] // or Caption-Abstract or ImageDescription
    caption: String,

    // https://stackoverflow.com/questions/41151080/deserialize-a-json-string-or-array-of-strings-into-a-vec
    #[serde(rename = "Keywords")] // or Subject
    tags: Vec<String>,

    #[serde(rename = "City")]
    city: Option<String>,

    #[serde(rename = "State")] // or Province-State
    state: String,

    #[serde(rename = "Copyright")] // or Rights or CopyrightNotice
    copyright: String,

    #[serde(rename = "UsageTerms")]
    usage_terms: String,

    #[serde(rename = "Software")] // or CreatorTool
    software: String,

    #[serde(rename = "Aperture")] // or FNumber
    aperture: f32,

    #[serde(rename = "ISO")]
    iso: u16,

    #[serde(rename = "ShutterSpeed")] // ShutterSpeedValue
    shuter_speed: String,

    // #[serde(rename = "ExposureCompensation")]
    // exposure_compensation: String,
    #[serde(rename = "FocalLength")]
    focal_length: u16,

    #[serde(rename = "MaxApertureValue")]
    max_aperture: f32,

    #[serde(rename = "Make")]
    camera_make: String,

    #[serde(rename = "Model")]
    camera_model: String,

    #[serde(rename = "DateTimeCreated")] // or DateTimeOriginal
    taken_on: String,

    #[serde(rename = "GPSLatitude")]
    latitude: Option<f32>,

    #[serde(rename = "GPSLongitude")]
    longitude: Option<f32>,

    #[serde(rename = "ProfileDescription")]
    color_profile: String,

    #[serde(rename = "ColorTemperature")]
    color_temperature: u16,

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

            Some(Photo {
                name: i.file_name.to_owned(),
                index,
                primary: index == cover_index,
                ..Photo::default()
            })
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect()
}

pub fn read_dir(path: &Path) -> Vec<ExifToolInfo> {
    // exiftool *.tif -json -quiet -coordFormat %.6f
    // exiftool 002.tif -json -quiet -coordFormat %.6f
    // exiftool *.tif -json -quiet -Aperture# -ColorTemperature# -DateTimeCreated -FocalLength# -FOV# -Keywords# -ShutterSpeed

    // suffix field name with # to disable ExifTool formatting
    let output = match Command::new("exiftool")
        .current_dir(path.to_string_lossy().to_string())
        .arg("*.tif")
        .arg("-json")
        .arg("-quiet")
        .arg("-Aperture#")
        .arg("-City")
        .arg("-ColorTemperature#")
        .arg("-Copyright")
        .arg("-DateTimeCreated")
        .arg("-Description")
        .arg("-ExposureCompensation")
        .arg("-FileName")
        .arg("-FocalLength#")
        .arg("-FOV#")
        .arg("-GPSLatitude#")
        .arg("-GPSLongitude#")
        .arg("-ImageHeight")
        .arg("-ImageWidth")
        .arg("-ISO")
        .arg("-Keywords")
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

    match serde_json::from_str::<Vec<ExifToolInfo>>(&text) {
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
