use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};

pub struct GadgetLoader {}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Gadget {
    MassStorage(ImageInfo),
    Serial,
    Ethernet,
}

impl Gadget {
    pub fn module_name(&self) -> String {
        match self {
            Gadget::MassStorage(_) => String::from("g_mass_storage"),
            Gadget::Serial => String::from("g_serial"),
            Gadget::Ethernet => String::from("g_ether"),
        }
    }

    pub fn module_arguments(&self) -> Vec<String> {
        match self {
            Gadget::MassStorage(image) => {
                let mut arguments = vec![format!("file={}", image.path)];

                if image.as_cdrom.unwrap_or(false) {
                    arguments.push(String::from("cdrom=y"));
                }

                arguments.push(format!(
                    "ro={}",
                    if image.read_only.unwrap_or(false) {
                        "1"
                    } else {
                        "0"
                    }
                ));

                arguments
            }
            Gadget::Serial => vec![],
            Gadget::Ethernet => vec![],
        }
    }
}

impl GadgetLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self) -> Vec<Gadget> {
        let files_list = std::env::var("CONFIG_PATH").unwrap_or_else(|_| {
            std::env::current_dir()
                .map(|path| {
                    path.as_os_str()
                        .to_str()
                        .map(|path| format!("{}/gadgets.yaml", path))
                        .unwrap_or_else(|| {
                            eprintln!("Could not build path to gadgets.yaml");
                            Default::default()
                        })
                })
                .unwrap_or_else(|err| {
                    println!("Could not get current-dir: {}", err);

                    Default::default()
                })
        });

        match OpenOptions::new().read(true).write(false).open(&files_list) {
            Ok(files) => serde_yaml::from_reader(files).unwrap_or_else(|err| {
                eprintln!("Could not parse gadgets.yaml: {}", err);

                Default::default()
            }),
            Err(err) => {
                eprintln!("Could not open gadgets file ({}): {}", &files_list, err);

                Default::default()
            }
        }
    }
}
