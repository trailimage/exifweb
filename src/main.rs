mod blog;
mod category;
mod config;
mod exif;
mod photo;
mod post;

use blog::Blog;
use category::Category;
use post::Post;
use std::fs;
use std::path::Path;
use toml;

fn main() {
    let root = Path::new("./public/");
    let path = Path::new("./public/config.toml");
    let mut blog = Blog {
        posts: Vec::new(),
        categories: Vec::new(),
    };
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let c: config::BlogConfig = toml::from_str(&contents).unwrap();

    for name in c.categories {
        blog.categories.push(Category {
            name,
            posts: Vec::new(),
        });
    }

    for entry in fs::read_dir(root).expect("Unable to read root directory") {
        let entry = entry.expect("Unable to access directory entry");
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        blog.posts.push(Post {
            title: String::new(),
            summary: String::new(),
            path,
            photos: Vec::new(),
            next: Option::None,
            prev: Option::None,
        });
    }

    println!("{} posts", blog.posts.len());
}
