//! Use ExifTool to extract photo metadata

use crate::config::PhotoConfig;
use crate::deserialize::{date_time_string, string_number, string_sequence};
use crate::html;
use crate::models::{Camera, ExposureMode, Location, Photo, SizeCollection};
use crate::tools::pos_from_name;
use chrono::{DateTime, FixedOffset};
use colored::*;
use lazy_static::*;
use regex::Regex;
use serde::Deserialize;
use serde_json;
use std::{mem, path::Path, process::Command};

#[derive(Deserialize, Debug)]
pub struct ExifToolOutput {
    #[serde(rename = "FileName")]
    file_name: String,

    #[serde(default, rename = "Artist")]
    artist: Option<String>,

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

    #[serde(default, rename = "Copyright")] // or Rights or CopyrightNotice
    copyright: Option<String>,

    #[serde(default, rename = "UsageTerms")]
    usage_terms: Option<String>,

    #[serde(rename = "Software")] // or CreatorTool
    software: String,

    #[serde(rename = "Aperture")] // or FNumber
    aperture: Option<f32>,

    #[serde(rename = "ISO")]
    iso: Option<u16>,

    // or ShutterSpeedValue
    #[serde(
        default,
        rename = "ShutterSpeed",
        deserialize_with = "string_number"
    )]
    shutter_speed: Option<String>,

    #[serde(
        default,
        rename = "ExposureCompensation",
        deserialize_with = "string_number"
    )]
    exposure_compensation: Option<String>,

    #[serde(default, rename = "ExposureProgram")]
    exposure_mode: ExposureMode,

    #[serde(rename = "FocalLength")]
    focal_length: Option<f32>,

    #[serde(rename = "MaxApertureValue")]
    max_aperture: Option<f32>,

    #[serde(rename = "Lens")]
    lens: Option<String>,

    #[serde(rename = "Make")]
    camera_make: Option<String>,

    #[serde(default, rename = "Model")]
    camera_model: Option<String>,

    #[serde(
        default,
        rename = "DateTimeCreated",
        deserialize_with = "date_time_string"
    )]
    // or DateTimeOriginal or DateTimeCreated
    taken_on: Option<DateTime<FixedOffset>>,

    #[serde(
        default,
        rename = "CreateDate",
        deserialize_with = "date_time_string"
    )]
    created_on: Option<DateTime<FixedOffset>>,

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

impl PartialEq for ExifToolOutput {
    fn eq(&self, other: &Self) -> bool {
        self.file_name == other.file_name
            && self.artist == other.artist
            && self.title == other.title
            && self.caption == other.caption
            && self.tags == other.tags
            && self.city == other.city
            && self.state == other.state
            && self.copyright == other.copyright
            && self.usage_terms == other.usage_terms
            && self.software == other.software
            && self.aperture == other.aperture
            && self.iso == other.iso
            && self.shutter_speed == other.shutter_speed
            && self.exposure_compensation == other.exposure_compensation
            && self.exposure_mode == other.exposure_mode
            && self.focal_length == other.focal_length
            && self.max_aperture == other.max_aperture
            && self.lens == other.lens
            && self.camera_make == other.camera_make
            && self.camera_model == other.camera_model
            && self.taken_on == other.taken_on
            && self.created_on == other.created_on
            && self.latitude == other.latitude
            && self.longitude == other.longitude
            && self.color_profile == other.color_profile
            && self.color_temperature == other.color_temperature
            && self.field_of_view == other.field_of_view
            && self.width == other.width
            && self.height == other.height
    }
}

impl Eq for ExifToolOutput {}

/// Execute exif_tool for each image file in given `path` and capture output as
/// `Photo` structs
pub fn parse_dir(path: &Path, config: &PhotoConfig) -> Vec<Photo> {
    lazy_static! {
        /// Period followed by two or more non-periods at end of text
        static ref FILE_EXT: Regex = Regex::new(r"\.[^\.]{2,}$").unwrap();
    }

    read_dir(&path)
        .iter_mut()
        .filter_map(|i: &mut ExifToolOutput| {
            // Photo index based on its file name pattern
            let index =
                pos_from_name(&config.capture_index, &i.file_name).unwrap_or(0);

            if index == 0 {
                println!(
                    "   {} {}",
                    "failed to infer index of".red(),
                    i.file_name.red()
                );
                return None;
            }

            let mut photo = Photo {
                name: FILE_EXT.replace(&i.file_name, "").to_string(),
                title: mem::replace(&mut i.title, None),
                artist: mem::replace(&mut i.artist, None),
                caption: mem::replace(&mut i.caption, None)
                    .map(|s| html::caption(&s)),
                software: mem::replace(&mut i.software, String::new()),
                tags: mem::replace(&mut i.tags, Vec::new()),
                index,
                size: SizeCollection::from(i.width, i.height, &config.size),
                date_taken: i.taken_on.or(i.created_on),
                ..Photo::default()
            };

            if let Some(make) = &i.camera_make {
                let name = match &i.camera_model {
                    Some(model) => format!("{} {}", make, model),
                    _ => make.clone(),
                };

                let camera = Camera {
                    name,
                    compensation: mem::replace(
                        &mut i.exposure_compensation,
                        None,
                    ),
                    shutter_speed: mem::replace(&mut i.shutter_speed, None),
                    mode: i.exposure_mode,
                    aperture: i.aperture,
                    focal_length: i.focal_length,
                    iso: i.iso,
                    lens: mem::replace(&mut i.lens, None),
                };

                photo.camera = Some(camera);
            }

            if i.latitude.is_some() && i.longitude.is_some() {
                let loc = Location {
                    latitude: i.latitude.unwrap(),
                    longitude: i.longitude.unwrap(),
                };

                if loc.is_valid() {
                    photo.location = Some(loc);
                }
            }

            Some(photo)
        })
        .collect()
}

pub fn read_dir(path: &Path) -> Vec<ExifToolOutput> {
    // exiftool *.jpg -json -quiet -coordFormat %.6f
    // exiftool 002.jpg -json -quiet -coordFormat %.6f -ExposureProgram#
    // exiftool *.jpg -json -quiet -Aperture# -ColorTemperature# -DateTimeCreated -FocalLength# -FOV# -Keywords# -ShutterSpeed

    // suffix field name with # to disable ExifTool formatting
    let output = match Command::new("exiftool")
        .current_dir(path.to_string_lossy().to_string())
        .arg("*.jpg")
        .arg("-json")
        .arg("-quiet")
        .arg("-Aperture#")
        .arg("-Artist")
        .arg("-City")
        .arg("-ColorTemperature#")
        .arg("-Copyright")
        .arg("-DateTimeCreated")
        .arg("-CreateDate")
        // Offsets seem only to be present for software modified dates
        //.arg("-OffsetTimeOriginal")
        //.arg("-OffsetTimeDigitized")
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
            println!("   {}", "failed to retrieve EXIF".red());
            return Vec::new();
        }
    };

    let text = match String::from_utf8(output.stdout) {
        Ok(text) => text,
        _ => {
            println!("   {}", "Failed to convert EXIF output to UTF-8".red());
            return Vec::new();
        }
    };

    if text.is_empty() {
        println!("   {}", "EXIF JSON is empty".red());
        return Vec::new();
    }

    match serde_json::from_str::<Vec<ExifToolOutput>>(&text) {
        Ok(info) => info,
        Err(e) => {
            println!("   {}", "unable to parse EXIF JSON".red());
            println!("{}", text);
            println!("   —\n   {:?}\n   —", e);
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExifToolOutput;
    use crate::models::ExposureMode;
    use chrono::DateTime;
    use serde_json;

    #[test]
    fn deserialize_test() {
        let json = r#"[{
            "SourceFile": "001.tif",
            "Aperture": 2.2,
            "Artist": "Jason Abbott",
            "ColorTemperature": 5800,
            "Copyright": "© Copyright 2017 Jason Abbott",
            "DateTimeCreated": "2017:08:06 11:25:41",
            "Description": "We worked all day yesterday, and various days before that, to get the bikes in working order. A hot and hazy day isn’t my first choice to ride the Boise Ridge but Nick and I want to put the bikes through their paces before a four-day ride in a few weeks.",
            "ExposureProgram": 2,
            "FileName": "001.tif",
            "FocalLength": 4.15,
            "FOV": 63.6549469203798,
            "GPSLatitude": 43.579192,
            "GPSLongitude": -116.173061,
            "ImageHeight": 75,
            "ImageWidth": 100,
            "ISO": 25,
            "Keywords": ["Gas Station","KTM 500 XC-W","Motorcycle"],
            "Lens": "iPhone 6s back camera 4.15mm f/2.2",
            "Make": "Apple",
            "Model": "iPhone 6s",
            "ProfileDescription": "ProPhoto RGB",
            "ShutterSpeed": "1/500",
            "Software": "Adobe Photoshop Lightroom Classic 9.2 (Windows)",
            "Title": "Fuel stop",
            "UsageTerms": "All Rights Reserved"
          }]"#;

        let target = vec![ExifToolOutput {
            file_name: "001.tif".to_owned(),
            artist: Some("Jason Abbott".to_owned()),
            title: Some("Fuel stop".to_owned()),
            caption: Some("We worked all day yesterday, and various days before that, to get the bikes in working order. A hot and hazy day isn’t my first choice to ride the Boise Ridge but Nick and I want to put the bikes through their paces before a four-day ride in a few weeks.".to_owned()),
            tags: vec!["Gas Station".to_owned(),"KTM 500 XC-W".to_owned(),"Motorcycle".to_owned()],
            city: None,
            state: None,
            copyright: Some("© Copyright 2017 Jason Abbott".to_owned()),
            usage_terms: Some("All Rights Reserved".to_owned()),
            software: "Adobe Photoshop Lightroom Classic 9.2 (Windows)".to_owned(),
            aperture: Some(2.2),
            iso: Some(25),
            shutter_speed: Some("1/500".to_owned()),
            exposure_compensation: None,
            exposure_mode: ExposureMode::ProgramAE,
            focal_length: Some(4.15),
            max_aperture: None,
            lens: Some("iPhone 6s back camera 4.15mm f/2.2".to_owned()),
            camera_make: Some("Apple".to_owned()),
            camera_model: Some("iPhone 6s".to_owned()),
            taken_on: Some(DateTime::parse_from_rfc3339("2017-08-06T11:25:41-06:00").unwrap()),
            created_on: None,
            latitude: Some(43.579192),
            longitude: Some(-116.173061),
            color_profile: Some("ProPhoto RGB".to_owned()),
            color_temperature: Some(5800),
            field_of_view: Some(63.6549469203798),
            width: 100,
            height: 75
        }];

        match serde_json::from_str::<Vec<ExifToolOutput>>(&json) {
            Ok(exif) => assert_eq!(exif, target),
            Err(e) => {
                eprintln!("{:?}", e);
                assert!(false);
            }
        }
    }
}
