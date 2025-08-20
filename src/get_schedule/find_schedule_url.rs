use reqwest::blocking::Client;
use soup::{NodeExt, QueryBuilderExt};
const TIB_USER_PAGE_URL: &str = "https://www.tib.org/en/linies-i-horaris/tren/-/linia/T1";

use std::fmt;

use crate::get_schedule::utils::curl_text;

pub enum FindUrlError {
    RequestError(reqwest::Error),
    ParseError(String),
}

impl From<reqwest::Error> for FindUrlError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl fmt::Display for FindUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FindUrlError::RequestError(err) => {
                write!(f, "Request error: {err}")
            }
            FindUrlError::ParseError(msg) => {
                write!(f, "Parse error: {msg}")
            }
        }
    }
}

impl From<FindUrlError> for anyhow::Error {
    fn from(value: FindUrlError) -> Self {
        Self::msg(value.to_string())
    }
}


fn curl_tib_user_page(client: &Client) -> Result<String, reqwest::Error> {
    curl_text(client, TIB_USER_PAGE_URL)
}

fn find_schedule_url_in_page(raw_page: &str) -> Result<String, String> {
    let page_contents = soup::Soup::new(raw_page);
    let candidates = page_contents.class("type1").attr_name("href").find_all();

    let current_timetable = "Current timetable";

    let first_match = candidates
        .filter_map(|candidate| {
            candidate.tag("span").find().iter().next().map(|span_node| {
                if span_node.text().contains(current_timetable) {
                    Some(candidate)
                } else {
                    None
                }
            })
        })
        .next()
        .ok_or(format!("No candidates containing {current_timetable}"))
        .map(|node| node.ok_or("No candidates of type1 with href attribute"))??;

    Ok(first_match
        .attrs()
        .get("href")
        .expect("Already querying for href before. Shouldn't get here.")
        .into())
}

pub fn find_schedule_url(client: &Client) -> Result<String, FindUrlError> {
    let raw_page = curl_tib_user_page(client)?;

    find_schedule_url_in_page(&raw_page).map_err(FindUrlError::ParseError)
}
