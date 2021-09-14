use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};

pub struct ImageLoader {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IconKind {
    File(String),
    Resource(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    pub name: String,
    pub path: String,
    pub icon: IconKind,
    pub read_only: Option<bool>,
    pub as_cdrom: Option<bool>,
}

impl ImageLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_images(&self) -> Vec<ImageInfo> {
        let files_list = std::env::var("CONFIG_PATH").unwrap_or_else(|_| {
            std::env::current_dir()
                .map(|path| {
                    path.as_os_str()
                        .to_str()
                        .map(|path| format!("{}/files.yaml", path))
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        });

        if let Ok(files) = OpenOptions::new().read(true).write(false).open(files_list) {
            serde_yaml::from_reader(files).unwrap_or_default()
        } else {
            vec![]
        }
    }
}
