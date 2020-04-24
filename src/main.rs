//#![allow(warnings)]
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod config;
mod deserialize;
mod html;
mod image;
mod io;
mod json_ld;
mod models;
mod tools;

use colored::*;
use config::{BlogConfig, FeaturedPost};
use image::image_magick;
use io::{read, Writer};
use models::{Blog, Photo};
use std::{
    self, env, fs,
    path::{Path, PathBuf},
};
use tools::folder_name;

// TODO: generate mini maps
// TODO: read and process GPX files

fn main() {
    // GitHub pages feature requires root at / or /docs
    let root = Path::new("./docs/");
    let mut config = match BlogConfig::load(root) {
        Some(config) => config,
        _ => {
            println!("{}", "Missing root configuration file".red());
            return;
        }
    };

    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        _ => {
            println!(
                "{} {}",
                "Failed to open root directory".red(),
                folder_name(root).red()
            );
            return;
        }
    };

    let args: Vec<String> = env::args().collect();
    let mut blog = Blog::default();

    config.force_rerender = args.contains(&"force".to_owned());

    println!(
        "{}",
        format!("\nForce re-render: {}", config.force_rerender)
            .cyan()
            .bold()
    );

    // iterate over every file or subdirectory within root
    for entry in entries {
        let path: PathBuf = entry.unwrap().path();
        let dir_name: &str = folder_name(&path);

        if !path.is_dir()
            || config.ignore_folders.contains(&dir_name.to_string())
        {
            // ignore root files and specified folders
            continue;
        }

        println!("\n{} └ {}", "Found root directory".bold(), dir_name.bold());

        if let Some(posts) = read::series(&path, &config) {
            println!("   Series of {} posts:", posts.len());
            for p in posts {
                println!(
                    "{:6}{} ({} photos)",
                    "",
                    p.sub_title.yellow(),
                    p.photo_count
                );
                blog.add_post(p);
            }
            // skip to next path entry if series was found
            continue;
        }

        if let Some(post) = read::post(path.as_path(), &config) {
            println!(
                "   {} ({} photos)",
                post.title.yellow(),
                post.photo_count
            );
            blog.add_post(post);
        }
    }

    print!("\n");
    success_metric(blog.post_count(), "total posts");

    if blog.is_empty() {
        return;
    }

    let sequence_changed = blog.correlate_posts();

    // Previously loaded posts that haven't changed but have different previous
    // or next posts need to be re-rendered to update navigation HTML
    read::post_photos(root, &config, &mut blog, &sequence_changed);

    blog.build_photo_urls(&config.photo);

    let render_count = blog.needs_render_count();

    success_metric(render_count, "posts need rendered");

    if render_count > 0 {
        blog.collate_tags();
        blog.sanitize_exif(&config.photo.exif);

        if let Some(p) = config
            .featured_post
            .and_then(|f| blog.get_featured(&f.path))
        {
            config.featured_post = Some(FeaturedPost {
                path: p.path.clone(),
                title: p.title.clone(),
            });
            success_metric(1, "featured post");
        } else {
            config.featured_post = None;
            success_metric(0, "featured posts");
        }

        success_metric(blog.category_count(), "post categories");
        success_metric(blog.tag_count(), "unique photo tags");

        let write = Writer::new(root, &config, &blog);

        write.posts();
        write.home_page();
        write.about_page();
        write.sitemap();
        write.category_menu();
        write.mobile_menu();
        write.photo_tags();
        write.categories();

        for (path, post) in blog.posts {
            let last_render = post.history.as_of;

            for p in post.photos {
                let full_path = &root.join(&path).to_string_lossy().to_string();

                if p.file.created > last_render {
                    image_magick::create_sizes(&full_path, &p, &config.photo)
                }
            }
        }
    }
}

fn success_metric(count: usize, label: &str) {
    println!("{}", format!("{:>5} {}", count, label).bold().green());
}
