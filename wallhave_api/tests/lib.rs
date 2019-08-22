#![feature(async_await)]
use wallhave_api::{Client, Search, search};

#[test]
fn t() {
    let client = Client::new("<API_KEY>");
    let conf = Search {
        ratio: "16x10".to_string(),
        resolutions: "1920x1080".to_string(),
        order: "asc".to_string(),
        sorting: "date_added".to_string(),
        purity: "nsfw".to_string(),
        categories: "latest".to_string(),
    };
    assert_eq!(true, search(&conf, &client).is_ok());
}
