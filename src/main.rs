mod blog;
//mod category;
mod config;
//mod exif;
mod photo;
mod post;

use crate::blog::Blog;
use crate::post::Post;
use std::fs::{self, DirEntry, ReadDir};
//use std::io;
use std::path::{Path, PathBuf};
use toml;

fn main() {
    let root = Path::new("./public/");
    let path = Path::new("./public/config.toml");
    let mut blog = Blog { posts: Vec::new() };
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let c: config::BlogConfig = toml::from_str(&contents).unwrap();
    // let entries = fs::read_dir(root)?
    //     .map(|res: ReadDir| res.filter(|e| e.path().is_dir()).map(|e| e.path()))
    //     .collect::<Result<Vec<PathBuf>, io::Error>>()?;

    //  match fs::read_dir(root) {
    //     Some<
    //  }

    match fs::read_dir(root) {
        Err(e) => println!("Unable to read root folder: {:?}", e),
        Ok(d) => {
            for entry in d {
                match entry {
                    Err(_) => println!("Unable to read file"),
                    Ok(f) => {
                        let path = f.path();
                        if path.is_dir() {
                            let p = Post {
                                title: String::new(),
                                summary: String::new(),
                                path: path.as_path(),
                                photos: Vec::new(),
                                next: Option::None,
                                prev: Option::None,
                            };
                            blog.posts.push(&p);
                        }
                    }
                }
            }
        }
    }

    println!("{}", c.what[0]);
}
