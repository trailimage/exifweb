//! TOML photo log

use super::load_toml;
use crate::models::Photo;
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// File the stores photo tag information and last process time
static LOG_FILE: &str = "log.toml";

/// Log processed photo information per post folder to determine when
/// re-processing is necessary
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoLog {
    /// Date of first relevant (not an outlier) photo in folder
    pub when: Option<DateTime<FixedOffset>>,
    /// When folder was last processed
    pub processed: DateTime<Local>,
    /// Photo tags
    /// // TODO: I think these need to be tags that map to photos
    pub tags: Vec<String>,
}

impl PhotoLog {
    /// Save information about loaded photos to avoid unecessary re-processing
    pub fn write(
        path: &Path,
        earliest_date: Option<DateTime<FixedOffset>>,
        photos: &Vec<Photo>,
    ) {
        let mut tags: Vec<String> = Vec::new();

        for p in photos.iter() {
            for t in p.tags.iter() {
                if !tags.contains(&t) {
                    tags.push(t.clone())
                }
            }
        }

        tags.sort();

        let log = PhotoLog {
            when: earliest_date,
            tags,
            processed: Local::now(),
        };

        match toml::to_string(&log) {
            Ok(content) => {
                match fs::write(path.join(LOG_FILE), &content) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error writing {:?}", e),
                };
                return;
            }
            Err(e) => eprintln!("Error serializaing {:?}", e),
        }
    }

    pub fn load(path: &Path) -> Option<Self> {
        load_toml(path, LOG_FILE)
    }
}
