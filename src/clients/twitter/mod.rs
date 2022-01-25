use crate::data::twitter;

pub mod tweets;
pub mod user;

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *twitter::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
