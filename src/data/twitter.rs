use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref BEARER: &'static str = dotenv!("TWITTER_BEARER");
}

lazy_static! {
    pub static ref URL: &'static str = "https://api.twitter.com/2";
}

// User Lookup
#[derive(Debug, Deserialize)] pub struct UserResponse { pub data: TwitterUser }
#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

// Followers Lookup
#[derive(Debug, Deserialize)] pub struct FollowResponse { pub data: Vec<TwitterUser> } 

// Tweets
#[derive(Debug, Deserialize)] 
pub struct TweetResponse { 
    pub data: Vec<Tweet>,
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
