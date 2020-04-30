// choco install webp
// brew install webp
use crate::{
    config::PhotoConfig,
    models::{suffix, Photo},
};
use colored::*;
use std::process::{self, Command};

/// Create vector of owned strings
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

//cwebp -near_lossless 0 -mt -m 6 -noalpha -sharp_yuv -metadata icc 028.tif -o 028_test.webp

/// https://developers.google.com/speed/webp/docs/cwebp
pub fn create_sizes(path: &str, p: &Photo, c: &PhotoConfig) {
    create_size(path, p, c, c.size.large, suffix::LARGE, false);
    create_size(path, p, c, c.size.medium, suffix::MEDIUM, false);
    create_size(path, p, c, c.size.small, suffix::SMALL, false);
    create_size(path, p, c, c.size.thumb, suffix::THUMB, true);
}

fn create_size(
    path: &str,
    photo: &Photo,
    config: &PhotoConfig,
    size: u16,
    suffix: &'static str,
    make_square: bool,
) {
    let setting =
        |key: &str, value: &str| string_vec![format!("-{}", key), value];

    let file_name =
        || format!("{:03}_{}{}", photo.index, suffix, config.output_ext);

    // The cwebp parameter is `-resize width height`. Leaving either width or
    // height `0` causes it to be computed to preserve the aspect ratio.
    let resize = |max: u16| {
        if config.source_size == max {
            Vec::new()
        } else if make_square {
            // crop always occurs before resize
            let (x, y, size) = photo.size.original.center_square();

            string_vec!["-crop", x, y, size, size, "-resize", max, 0]
        } else {
            if photo.size.is_landscape() {
                string_vec!["-resize", max, 0]
            } else {
                string_vec!["-resize", 0, max]
            }
        }
    };

    match Command::new("cwebp")
        .current_dir(path)
        .arg("-mt")                         // enable multi-threading
        .arg("-noalpha")                    // drop any alpha channel
        .arg("-quiet")
        .args(setting("near_lossless", "0"))
        .args(setting("m", "6"))            // best quality method
        .args(setting("metadata", "icc"))   // retain profile but drop EXIF
        .args(resize(size))
        .arg(&photo.file.name)
        .args(setting("o", &file_name()))   // output file
        .output()
    {
        Ok(out) => {
            if let Ok(err) = String::from_utf8(out.stderr) {
                if !err.is_empty() {
                    println!("   {}", err.red());
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
