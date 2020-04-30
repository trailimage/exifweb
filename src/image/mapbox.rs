use crate::config::BlogConfig;
use crate::models::Post;
use reqwest;
use std::{fs::File, io, path::Path};

pub struct MapBox<'a> {
    root: &'a Path,
    config: &'a BlogConfig,
    pin_url: &'a str,
}
impl<'a> MapBox<'a> {
    /// Retrieve and save MapBox static map image for post photo locations
    pub fn save_static(post: &'a Post, root: &'a Path, config: &'a BlogConfig) {
        let mapbox = MapBox {
            root,
            config,
            pin_url: &format!(
                "{}/{}",
                config.site.url, config.mapbox.pin_image
            ),
        };

        mapbox.download_maps(post);
    }

    fn pin_list(&self, post: &Post) -> Vec<String> {
        post.photo_locations
            .iter()
            .map(|(lon, lat)| format!("url-{}({}{})", self.pin_url, lat, lon))
            .collect()
    }

    fn download_maps(&self, post: &'a Post) {
        let pins = self.pin_list(post);

        self.download_static_map(
            post,
            "map.png",
            &pins,
            self.config.style.content_width,
            self.config.style.inline_map_height,
        );

        self.download_static_map(
            post,
            "map_small.png",
            &pins,
            self.width_by_cover(post),
            self.height_by_cover(post),
        );
    }

    /// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
    fn download_static_map(
        &self,
        post: &Post,
        name: &str,
        pins: &Vec<String>,
        width: u16,
        height: u16,
    ) {
        let url = format!("https://api.mapbox.com/styles/v1/{}/static/{}/auto/{}x{}?access_token={}&attribution=false&logo=false",
            self.config.mapbox.style.r#static,
            pins.join(","),
            width,
            height,
            self.config.mapbox.access_token);

        match reqwest::blocking::get(&url) {
            Ok(mut res) => {
                match File::create(self.root.join(&post.path).join(name)) {
                    Ok(mut dest) => match io::copy(&mut res, &mut dest) {
                        Ok(_bytes_copied) => (),
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }

    /// Height of map image next to cover photo computed so map matches height
    /// of landscape images but is less than height of portrait images
    fn height_by_cover(&self, post: &Post) -> u16 {
        let max_height = self.config.style.inline_map_height;

        post.cover_photo().map_or(max_height, |p| {
            if p.size.small.height > p.size.small.width {
                // limit height next to portrait images
                max_height
            } else {
                p.size.small.height
            }
        })
    }

    /// Width of map image next to cover computed so that side-by-side they fill
    /// the `content_width`
    fn width_by_cover(&self, post: &Post) -> u16 {
        let width = self.config.style.content_width;

        post.cover_photo()
            .map_or(width, |p| width - p.size.small.width)
    }
}
