use crate::XmpMeta;
use encoding::all::*;
use encoding::{DecoderTrap, Encoding};
use serde::Deserialize;
use serde_json;
use serde_xml_rs;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct IPTC {
    #[serde(rename = "Keyword[2,25]")]
    keywords: Vec<String>,

    #[serde(rename = "Image Name[2,5]")]
    title: [String; 1],
}

#[derive(Deserialize, Debug)]
struct Profiles {
    iptc: IPTC,
}

#[derive(Deserialize, Debug)]
struct EXIF {
    #[serde(rename = "exif:Copyright")]
    copyright: String,

    #[serde(rename = "exif:ApertureValue")]
    aperture: String,

    #[serde(rename = "exif:Artist")]
    artist: String,

    #[serde(rename = "exif:Software")]
    software: String,

    #[serde(rename = "exif:ImageDescription")]
    caption: String,
}

#[derive(Deserialize, Debug)]
pub struct ImageProperties {
    #[serde(rename = "baseName")]
    file_name: String,
    format: String,
    #[serde(rename = "properties")]
    exif: EXIF,
    profiles: Profiles,
}
#[derive(Deserialize, Debug)]
pub struct ImageInfo {
    image: ImageProperties,
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

    //println!("{}", text);

    serde_xml_rs::from_str(&text).unwrap()
}

// convert image.jpg[1x1+0+0] json:
// https://imagemagick.org/script/convert.php

// magick convert br08-party-on-crater-peak_011-of-020.jpg[1x1+0+0] json:
// magick convert img_006-of-021.jpg[1x1+0+0] json:out.json
// magick convert img_006-of-021.jpg -format "%[IPTC:*]" info:
// magick convert -ping img_006-of-021.jpg xmp:

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
