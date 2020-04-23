use crate::{config::PhotoConfig, models::Photo};
use colored::*;
use std::process::Command;

// magick convert *.tif -quiet ( +clone -resize 2048x -write *-large.webp +delete ) ( +clone -resize 1024x -write *-regular.webp +delete ) ( +clone -resize 320x -write *-small.webp +delete ) -resize 256x256^ -gravity center -extent 256x256 001-thumb.webp

// -thumbnail 100x100^ -gravity center -extent 100x100

// magick convert *.tif -define tiff:ignore-tags=42033 -resize 256x256^ -gravity center -extent 256x256 *-thumb.webp

// convert -resize 256x256^ -extent 256x256 in.png out.png

// -define tiff:ignore-tags=42033

/// https://imagemagick.org/script/webp.php
pub fn create_sizes(path: &str, photo: &Photo, config: &PhotoConfig) {
    // magick convert 001.tif -quiet -resize 2048x2048 -define webp:lossless=false -define webp:thread-level=1 001.webp
    match Command::new("magick")
        .current_dir(path)
        .arg("convert")
        .arg(&photo.file.name)
        .arg("-quiet")
        .arg("-resize")
        .arg(format!("{px}x{px}", px = config.size.large))
        .arg(format!("{:03}_l{}", photo.index, config.output_ext))
        .output()
    {
        Ok(out) => (),
        Err(e) => {
            println!(
                "   {} {} {:?}",
                "Failed to generate sizes for".red(),
                path.magenta(),
                e
            );
        }
    };
}
