use crate::data::twitter;

pub async fn find_user(name: &str) -> Result<twitter::TwitterUser, reqwest::Error> {    
    let full_url = format!("{}/users/by/username/{}", *twitter::URL, name);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<twitter::UserResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn get_followers(id: &str) -> Result<Vec<twitter::TwitterUser>, reqwest::Error> {
    let full_url = format!("{}/users/{}/followers", *twitter::URL, &id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<twitter::FollowResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn get_following(id: &str) -> Result<Vec<twitter::TwitterUser>, reqwest::Error> {
    let full_url = format!("{}/users/{}/following", *twitter::URL, id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<twitter::FollowResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn get_recent_tweets(id: &str) -> Result<Vec<twitter::Tweet>, reqwest::Error> {
    let full_url = format!("{}/users/{}/tweets", *twitter::URL, id);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<twitter::TweetResponse>()
        .await?;

    Ok(resp.data)
}

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *twitter::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
