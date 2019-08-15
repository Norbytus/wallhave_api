#[macro_use]
extern crate clap;

use clap::App;
use serde_derive::{Deserialize, Serialize};

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let yml = load_yaml!("../arg.yml");
    let matchs = App::from_yaml(yml).get_matches();

    let config = Config::from_file(matchs.value_of("config").unwrap_or(""))?;

    let wallhaven = WallHavenApi::init_from_config(&config);

    let search = WallHaveSearch(wallhaven);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    wallhaven_search_config: WallHavenSearchConfig,
}

trait GetToken {
    fn get_token(&self) -> &str;
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).write(false).open(path)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;

        match toml::from_str(&buffer) {
            Ok(config) => Ok(config),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn get_token(&self) -> &str {
        &self.token
    }
}

impl GetToken for Config {
    fn get_token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug)]
struct WallHavenApi {
    token: String,
}

impl WallHavenApi {
    fn get_base_url() -> &'static str {
        "https://wallhaven.cc/api/v1/"
    }

    fn init_from_config<T: GetToken>(config: &T) -> Self {
        Self {
            token: config.get_token().into()
        }
    }
}

#[derive(Debug)]
struct WallHaveSearch(WallHavenApi);

#[derive(Deserialize, Debug)]
struct WallHavenSearchConfig {
    sorting: String,
    resolutions: String,
}

impl WallHaveSearch {
    fn get_end_point() -> &'static str {
        "search?"
    }

    fn prepare_url(search_config: &WallHavenSearchConfig) {
    }
}

type SearchResultList = Vec<SearchResult>;

#[derive(Deserialize, Serialize, Debug)]
struct SearchResult {
    path: String,
}
