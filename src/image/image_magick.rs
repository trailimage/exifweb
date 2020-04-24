use crate::{
    config::PhotoConfig,
    models::{suffix, Photo},
};
use colored::*;
use std::process::Command;

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
        format!(
            "( +clone -resize {}x -write {} +delete )",
            size,
            file_name(suffix)
        )
    };

    match Command::new("magick")
        .current_dir(path)
        .arg("convert")
        .arg(&photo.file.name)
        .arg("-quiet")
        .arg(sub_command(config.size.large, suffix::LARGE))
        .arg(sub_command(config.size.medium, suffix::MEDIUM))
        .arg(sub_command(config.size.small, suffix::SMALL))
        .arg(format!("-resize {px}x{px}^", px = config.size.thumb))
        .arg("-gravity center")
        .arg(format!("-extent {px}x{px}", px = config.size.thumb))
        .arg(file_name(suffix::THUMB))
        .output()
    {
        Ok(out) => {
            let text = match String::from_utf8(out.stdout) {
                Ok(text) => text,
                Err(e) => {
                    println!(
                        "   {}: {:?}",
                        "Failed to read resize output".red(),
                        e
                    );
                    String::new()
                }
            };
            if !text.is_empty() {
                println!("{}", text);
            }
        }
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
