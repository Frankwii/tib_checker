pub fn curl_text(client: &reqwest::blocking::Client, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send()?.text()
}

pub fn curl_bytes(client: &reqwest::blocking::Client, url: &str) -> Result<Vec<u8>, reqwest::Error> {
    Ok(client.get(url).send()?.bytes()?.into_iter().collect())
}
