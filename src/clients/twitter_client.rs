use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref TWITTER_BEARER: &'static str = dotenv!("TWITTER_BEARER");
}

lazy_static! {
    static ref TWITTER_URL: &'static str = "https://api.twitter.com/2";
}

// User Lookup
#[derive(Debug, Deserialize)] struct UserResponse { data: TwitterUser }
#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

// Followers Lookup
#[derive(Debug, Deserialize)] pub struct FollowersResponse { data: Vec<TwitterUser> } 

pub async fn find_user(name: &str) -> Result<TwitterUser, reqwest::Error> {    
    let full_url = format!("{}/users/by/username/{}", *TWITTER_URL, name);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<UserResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn get_followers(id: &str) -> Result<FollowersResponse, reqwest::Error> {
    let full_url = format!("{}/users/{}/followers", *TWITTER_URL, &id);

        let client = build_client().unwrap();
        let resp = client.get(full_url)
            .send()
            .await?
            .json::<FollowersResponse>()
            .await?;

    Ok(resp)
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
