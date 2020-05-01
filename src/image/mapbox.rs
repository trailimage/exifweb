//! Methods to interact with MapBox services

use crate::config::BlogConfig;
use crate::models::Post;
use reqwest;
use std::{fs::File, io, path::Path};
use url::form_urlencoded::byte_serialize;

pub struct MapBox<'a> {
    root: &'a Path,
    config: &'a BlogConfig,
}
impl<'a> MapBox<'a> {
    /// Retrieve and save MapBox static map image for post photo locations
    ///
    /// https://docs.mapbox.com/help/how-mapbox-works/static-maps/
    /// https://docs.mapbox.com/api/maps/#static-images
    ///
    pub fn save_static(post: &'a Post, root: &'a Path, config: &'a BlogConfig) {
        let mapbox = MapBox { root, config };

        mapbox.download_maps(post);
    }

    /// Generate markers as `url-{url}({lon},{lat})`
    ///
    /// https://docs.mapbox.com/api/maps/#marker
    ///
    fn pin_list(&self, post: &Post) -> Vec<String> {
        post.photo_locations
            .iter()
            .map(|(lon, lat)| {
                format!("url-{}({},{})", self.config.mapbox.pin_image, lon, lat)
            })
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
            post.cover_map_size.0,
            post.cover_map_size.1,
        );
    }

    fn download_static_map(
        &self,
        post: &Post,
        name: &str,
        pins: &Vec<String>,
        width: u16,
        height: u16,
    ) {
        let pins: String = byte_serialize(pins.join(",").as_bytes()).collect();

        let url = format!("https://api.mapbox.com/styles/v1/{}/static/{}/auto/{}x{}@2x?access_token={}&attribution=false&logo=false",
            self.config.mapbox.style.r#static,
            pins,
            width,
            height,
            self.config.mapbox.access_token);

        // GOAL: refactor match nesting
        match reqwest::blocking::get(&url) {
            Ok(mut res) => {
                if res.status().is_success() {
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
                } else {
                    println!(
                        "Failed to download {} {}",
                        url,
                        res.text().unwrap_or(String::new())
                    );
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }
}
