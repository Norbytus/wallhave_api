#![feature(async_await)]
use std::convert::Into;
use std::marker::PhantomData;
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
pub struct Color {
    hex: String,
}

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
    ratio: f64,
    file_size: u64,
    file_type: String,
    created_at: String,
    colors: Vec<Color>,
    path: String,
    thumbs: Thumbs,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Meta {
    current_path: u32,
    last_page: u32,
    per_page: u32,
    total: u32,
    seed: String,
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

pub async fn search(search_param: &Search, client: &Client) -> Option<Data> {
    let url = format!("{}search?apikey={}&{}", BASE_URL, client.token, search_param.to_query());
    match surf::get(&url).recv_string().await {
        Ok(response) => {
            let data = serde_json::from_str::<Data>(&response);
            match data {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        },
        Err(_) => None
    }
}
