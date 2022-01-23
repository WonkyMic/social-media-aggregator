use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref TWITTER_BEARER: &'static str = dotenv!("TWITTER_BEARER");
}

#[derive(Debug, Deserialize)] struct UserResponse { data: TwitterUser }
#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    id: String,
    name: String,
    username: String,
}

pub async fn find_user(name: &str) -> Result<TwitterUser, reqwest::Error> {    
    let full_url = format!("https://api.twitter.com/2/users/by/username/{}", name);

    let client = build_client().expect("Client Builder failure in find_user");
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<UserResponse>()
        .await?;

    Ok(resp.data)
}

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *TWITTER_BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
