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

use std::io;
use std::fs;
use std::path::{PathBuf, Path};
use toml;
use fs::DirEntry;

fn main() {
    let root = Path::new("./public/");
    let mut blog = Blog {
        posts: Vec::new(),
        categories: Vec::new(),
    };
    let contents = fs::read_to_string(root.join("config.toml")).unwrap();
    let config: BlogConfig = toml::from_str(&contents).unwrap();

    //  for name in config.categories {
    //      blog.categories.push(Category {
    //          name,
    //          posts: Vec::new(),
    //      });
    //  }

    for entry in fs::read_dir(root.join("img/")).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        if !path.is_dir() {
            continue;
        }

        let series_posts: Vec<Post> = fs::read_dir(&path).unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.is_dir())
            .map(load_post)
            .collect();

        println!("{} series posts", series_posts.len());

        if !series_posts.is_empty() {
            for p in series_posts {
               blog.posts.push(p);
            }
            continue;
        }

        blog.posts.push(load_post(path));
    }

    println!("{} posts", blog.posts.len());
}




fn load_post<'a>(path: PathBuf) -> Post<'a> {
   Post {
      path,
      chronological: true,
      ..Default::default()
  }
}
