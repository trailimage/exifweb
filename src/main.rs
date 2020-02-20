mod blog;
mod config;
mod post;

use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use toml;

fn main() -> io::Result<()> {
    let root = Path::new("./public/");
    let path = Path::new("./public/config.toml");
    //let blog = Blog();
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let c: config::BlogConfig = toml::from_str(&contents).unwrap();
    // let entries = fs::read_dir(root)?
    //     .map(|res: ReadDir| res.filter(|e| e.path().is_dir()).map(|e| e.path()))
    //     .collect::<Result<Vec<PathBuf>, io::Error>>()?;

    println!("{}", c.what);

    //Ok(())
}

// fn post_paths(root: &Path) -> io::Result<()> {
//     fs::read_dir(root)?
//         .map(|res| res.map(|e: DirEntry| e.path()))
//         .collect::<Result<Vec<_>, io::Error>>();

//     // entries.sort();

//     Ok(())
// }
