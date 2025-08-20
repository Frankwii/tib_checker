mod find_schedule_url;
mod utils;

pub use find_schedule_url::find_schedule_url;

use reqwest::blocking::Client;

use crate::get_schedule::{find_schedule_url::FindUrlError, utils::{curl_bytes}};

fn get_client() -> Client {
    reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .build().expect("Build client")
}

pub fn get_schedule_pdf() -> Result<(Vec<u8>, String), FindUrlError> {
    let client = get_client();

    let schedule_url = find_schedule_url(&client)?;
    Ok((curl_bytes(&client, &schedule_url)?, schedule_url))
}
