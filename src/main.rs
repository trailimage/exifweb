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
use config::{BlogConfig, BlogLog, FeaturedPost};
use image::cwebp;
use io::{read, Writer};
use models::{Blog, Photo};
use std::{
    self, env, fs,
    path::{Path, PathBuf},
    process,
};
use tools::folder_name;

// TODO: read and process GPX files
// TODO: compile TypeScript
// TODO: compile LESS

fn main() {
    // GitHub pages feature requires root at / or /docs
    let root = Path::new("./docs/");
    let entries = load_root_directory(&root);
    let mut config = load_config(&root);
    let mut blog = Blog::default();

    blog.history = BlogLog::load(&root).unwrap_or(BlogLog::empty());

    // iterate over every file or directory within root
    for entry in entries {
        post_from_entry(&mut blog, entry.unwrap(), &config);
    }

    print!("\n");
    success_metric(blog.post_count(), "total posts");

    if blog.is_empty() {
        return;
    }

    blog.correlate_posts();

    for (_, p) in blog
        .posts
        .iter_mut()
        .filter(|(_, p)| p.photos.is_empty() && p.sequence_changed())
    {
        // posts that changed order need to be re-rendered which requires all
        // their photo data to be loaded
        p.add_photos(read::load_photos(&root.join(&p.path), &config.photo));
    }

    blog.build_photo_urls(&config);

    let render_count = blog.needs_render_count();
    let render_html =
        render_count > 0 || config.force.html || config.force.tags;

    success_metric(render_count, "posts need rendered");

    if render_html || config.force.maps || config.force.photos {
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
        BlogLog::write(root, &blog);

        let write = Writer::new(root, &config, &blog);

        if render_html {
            write.posts();
            write.home_page();
            write.sitemap();
            write.category_menu();
            write.mobile_menu();
            write.photo_tags();
            write.about_page();
            write.error_pages();
            write.categories();
        }

        write.post_maps();

        for (path, post) in blog.posts {
            let last_render = post.history.as_of;
            let full_path = root.join(&path).to_string_lossy().to_string();
            let mut count: usize = 0;

            println!(
                "\nExamining {} photos in {}",
                post.title.yellow(),
                full_path.cyan()
            );

            for p in post.photos {
                if p.file.created > last_render {
                    count = count + 1;
                    cwebp::create_sizes(&full_path, &p, &config.photo);
                }
            }
            if count > 0 {
                println!("   Resized {} photo(s)", count);
            } else {
                println!("   All photos are current");
            }
        }
    }
}

/// Load all entries (files and directories) from the root directory
fn load_root_directory(root: &Path) -> fs::ReadDir {
    match fs::read_dir(root) {
        Ok(entries) => entries,
        _ => {
            println!(
                "{} {}",
                "Failed to open root directory".red(),
                folder_name(root).red()
            );
            process::exit(1)
        }
    }
}

/// Load configuration file and apply command line arguments and environment
/// variables
fn load_config(root: &Path) -> BlogConfig {
    let mut config: BlogConfig = match BlogConfig::load(root) {
        Some(config) => config,
        _ => {
            println!("{}", "Missing root configuration file".red());
            process::exit(1)
        }
    };
    let args: Vec<String> = env::args().collect();
    let has_arg = |arg: &str| args.contains(&arg.to_owned());
    let notify = |label: &str, force: bool| {
        println!("{}", format!("Force {}: {}", label, force).cyan().bold())
    };

    config.force.html = has_arg("force_html");
    config.force.maps = has_arg("force_maps");
    config.force.photos = has_arg("force_photos");
    config.force.tags = has_arg("force_tags");

    println!("");
    notify("HTML re-render", config.force.html);
    notify("static map re-download", config.force.maps);
    notify("photo resizing", config.force.photos);
    notify("tag page re-render", config.force.tags);

    config
}

/// Create post(s) from a directory entry. The number of posts created for the
/// blog may be one, several or none depending on whether the entry is a
/// post-containing directory, a series-containing directory or neither,
/// respectively.
fn post_from_entry(blog: &mut Blog, entry: fs::DirEntry, config: &BlogConfig) {
    let path: PathBuf = entry.path();
    let dir_name: &str = folder_name(&path);

    if !path.is_dir() || config.ignore_folders.contains(&dir_name.to_string()) {
        // ignore root files and specified folders
        return;
    }

    println!("\n{} └ {}", "Found root directory".bold(), dir_name.bold());

    if let Some(posts) = read::series(&path, &config) {
        println!("   Series of {} posts:", posts.len());
        for p in posts {
            println!("{:6}{} ({} photos)", "", p.title.yellow(), p.photo_count);
            blog.add_post(p);
        }
        // skip to next path entry if series was found
        return;
    }

    if let Some(post) = read::post(path.as_path(), &config) {
        println!("   {} ({} photos)", post.title.yellow(), post.photo_count);
        blog.add_post(post);
    }
}

fn success_metric(count: usize, label: &str) {
    println!("{}", format!("{:>5} {}", count, label).bold().green());
}
