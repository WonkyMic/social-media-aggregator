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
#[derive(Debug, Deserialize)] struct FollowResponse { data: Vec<TwitterUser> } 

// Tweets
#[derive(Debug, Deserialize)] 
struct TweetResponse { 
    data: Vec<Tweet>,
    meta: TweetLookupMetadata
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    id: String,
    text: String
}

#[derive(Debug, Deserialize)]
struct TweetLookupMetadata {
    oldest_id: String,
    newest_id: String,
    result_count: u8,
    next_token: String
}

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

pub async fn get_followers(id: &str) -> Result<Vec<TwitterUser>, reqwest::Error> {
    let full_url = format!("{}/users/{}/followers", *TWITTER_URL, &id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<FollowResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn get_following(id: &str) -> Result<Vec<TwitterUser>, reqwest::Error> {
    let full_url = format!("{}/users/{}/following", *TWITTER_URL, id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<FollowResponse>()
        .await?;

    Ok(resp.data)
}


pub async fn get_recent_tweets(id: &str) -> Result<Vec<Tweet>, reqwest::Error> {
    let full_url = format!("{}/users/{}/tweets", *TWITTER_URL, id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<TweetResponse>()
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
