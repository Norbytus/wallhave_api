use clap::{App, load_yaml};
use serde_derive::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml::de::from_str;
use std::path::{Path, PathBuf};
use std::env::home_dir;
use std::fs::DirBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let yml = load_yaml!("../arg.yml");
    let matchs = App::from_yaml(yml).get_matches();

    let config = Config::from_file(matchs.value_of("config").unwrap_or(""))?;
    init_conf_dir(&config);

    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    wallhaven_search_config: WallHavenConf,
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        match from_str(&buffer) {
            Ok(success) => Ok(success),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn get_folders_name(&self) -> Vec<String> {
        vec![String::from("wallhaven")]
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct WallHavenConf {
    token: String,
    resolution: String,
    sorting: String,
    categoriest: String,
}

fn init_conf_dir(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut home_dir_path: PathBuf = home_dir().expect("Can't find home directory");

    home_dir_path.push(".config/wallhaven");

    if !home_dir_path.exists() {
        DirBuilder::new().recursive(true).create(&home_dir_path)?;
    }

    for file_folder_name in config.get_folders_name() {
        let mut temp = home_dir_path.clone();
        temp.push(file_folder_name);
        DirBuilder::new().recursive(true).create(&temp)?;
    }

    Ok(())
}
