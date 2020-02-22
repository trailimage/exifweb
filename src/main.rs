mod blog;
mod caption;
mod category;
mod config;
mod exif;
mod photo;
mod post;

pub use blog::Blog;
pub use caption::Caption;
pub use category::Category;
pub use config::*;
pub use exif::EXIF;
pub use photo::Photo;
pub use post::Post;

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

    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
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
