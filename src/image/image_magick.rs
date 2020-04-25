use crate::{
    config::PhotoConfig,
    models::{suffix, Photo},
};
use colored::*;
use std::process::{self, Command};

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
