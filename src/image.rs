use crate::{path_name, tab, XmpMeta};
use colored::*;
use encoding::all::*;
use encoding::{DecoderTrap, Encoding};
use serde::Deserialize;
use serde_json;
use serde_xml_rs;
use std::path::Path;
use std::process::Command;

// https://www.awaresystems.be/imaging/tiff/tifftags/private.html
#[derive(Deserialize, Debug)]
pub struct ImageProperties {
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
pub struct ImageFields {
    #[serde(rename = "baseName")]
    pub file_name: String,
    pub format: String,
    pub properties: ImageProperties,
}
#[derive(Deserialize, Debug)]
pub struct ImageInfo {
    pub image: ImageFields,
}

pub fn read_dir_exif(path: &Path) -> Option<Vec<ImageInfo>> {
    // magick convert -quiet *.tif json:
    let output = match Command::new("magick")
        .current_dir(path.to_string_lossy().to_string())
        .arg("convert")
        .arg("-quiet")
        .arg("*.tif")
        .arg("json:")
        .output()
    {
        Ok(out) => out,
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "Failed to generate EXIF for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            return None;
        }
    };

    let text = match ISO_8859_1.decode(&output.stdout[..], DecoderTrap::Ignore)
    {
        Ok(text) => text,
        _ => {
            println!(
                "{:tab$}{} {}",
                "",
                "Failed to convert EXIF output to UTF-8 for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            return None;
        }
    };

    if text.is_empty() {
        println!(
            "{} {}",
            "EXIF JSON is empty for".red(),
            path_name(&path).magenta()
        );
        return None;
    }

    match serde_json::from_str::<Vec<ImageInfo>>(&text) {
        Ok(info) => Some(info),
        Err(e) => {
            println!(
                "{:tab$}{} {}",
                "",
                "Unable to parse EXIF JSON for".red(),
                path_name(&path).magenta(),
                tab = tab(1)
            );
            println!("{}", text);
            println!("{:?}", e);
            None
        }
    }
}

// we’re cold
// https://www.imagemagick.org/discourse-server/viewtopic.php?t=30987
// https://users.rust-lang.org/t/reading-latin1-ascii-chars-from-a-binary-file/25849/2
// https://www.imagemagick.org/discourse-server/viewtopic.php?t=16586
pub fn exif() -> Vec<ImageInfo> {
    let output = Command::new("magick")
        .current_dir("./src/fixtures")
        .arg("convert")
        .arg("img_006-of-021.jpg[1x1+0+0]")
        .arg("json:")
        .output()
        .unwrap();

    let text = ISO_8859_1
        .decode(&output.stdout[..], DecoderTrap::Ignore)
        .unwrap();

    //println!("{}", text);

    serde_json::from_str(&text).unwrap()
}

pub fn xmp() -> XmpMeta {
    let output = Command::new("magick")
        .current_dir("./src/fixtures")
        .arg("convert")
        .arg("img_006-of-021.jpg")
        .arg("xmp:")
        .output()
        .unwrap();

    let text = String::from_utf8(output.stdout).unwrap();

    serde_xml_rs::from_str(&text).unwrap()
}

// convert image.jpg[1x1+0+0] json:
// https://imagemagick.org/script/convert.php

// magick convert br08-party-on-crater-peak_011-of-020.jpg[1x1+0+0] json:
// magick convert img_006-of-021.jpg[1x1+0+0] json:out.json
// magick convert img_006-of-021.jpg -format "%[IPTC:*]" info:
// magick convert -ping img_006-of-021.jpg xmp:
// magick convert -quiet 001.tif json:
// magick convert 001.tif xmp:
// magick convert atlanta-loop_001-of-038.tif info:
// magick convert -quiet *.png json:
// magick convert -quiet *.tif xmp:
// magick identify -verbose atlanta-loop_006-of-038.png
// magick identify -verbose 001.tif

#[cfg(test)]
mod tests {
    use super::{exif, xmp};
    use crate::XmpMeta;

    #[test]
    fn exif_test() {
        let got = exif();
        assert_eq!(got.len(), 1);
    }

    #[test]
    fn xmp_test() {
        let got: XmpMeta = xmp();
        println!("{}", got.rdf.description.description.item.value);
        assert_eq!(got.rdf.description.title.item.value, "Time to move on");
    }
}
