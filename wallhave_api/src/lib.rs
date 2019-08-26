#![feature(async_await)]
use std::convert::Into;
use std::error::Error;
use std::collections::HashSet;
use serde_derive::{Deserialize, Serialize};

const BASE_URL: &'static str = "https://wallhaven.cc/api/v1/";

pub struct Client {
    pub token: String,
}

impl Client {
    pub fn new<T: Into<String>>(token: T) -> Self {
        Self {
            token: token.into(),
        }
    }
}

fn build_url() -> String {
    String::new()
}


#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Data {
    data: Vec<ImageInfo>,
    meta: Meta,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Color (String);

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Thumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ImageInfo {
    id: String,
    url: String,
    short_url: String,
    views: u32,
    favorites: u32,
    source: String,
    purity: String,
    category: String,
    dimension_x: u64,
    dimension_y: u64,
    resolution: String,
    ratio: String,
    file_size: u64,
    file_type: String,
    created_at: String,
    colors: Vec<Color>,
    path: String,
    thumbs: Thumbs,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Meta {
    current_page: u32,
    last_page: u32,
    per_page: String,
    total: u32,
    seed: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Search {
    pub ratio: String,
    pub resolutions: String,
    pub order: String,
    pub sorting: String,
    pub purity: String,
    pub categories: String,
}

struct QuerySearch {
    incluide_tags: Tags,
}

struct Tags(HashSet<String>);

impl Tags {
    fn new() -> Self {
        Self (HashSet::new())
    }

    fn add_tag<T: Into::<String>>(&mut self, tag: T) {
        self.0.insert(tag.into());
    }

    fn remove_tag<T: Into::<String>>(&mut self, tag: T) {
        self.0.remove(&tag.into());
    }
}

impl QuerySearch {
    fn new() -> Self {
        Self {
            incluide_tags: Tags::new(),
        }
    }
}

impl Search {
    fn to_query(&self) -> String {
        format!("ratio={}&resolutions={}&order={}&sorting={}&purity={}&categories={}",
            self.ratio,
            self.resolutions,
            self.order,
            self.sorting,
            self.purity,
            self.categories
        )
    }
}

pub fn search(search_param: &Search, client: &Client) -> Result<Data, Box<dyn Error>> {
    let url = format!("{}search?apikey={}&{}", BASE_URL, client.token, search_param.to_query());
    let result = reqwest::get(&url)?.text()?;

    match serde_json::from_str::<Data>(&result) {
        Ok(result) => {
            Ok(result)
        },
        Err(e) => {
            Err(Box::new(e))
        }
    }
}
