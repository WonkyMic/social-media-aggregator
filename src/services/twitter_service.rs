use std::io::Error;
use crate::clients::twitter_client;

#[derive(Debug)]
pub struct TwitterProfile {
    pub user: twitter_client::TwitterUser,
    pub followers: Vec<twitter_client::TwitterUser>,
    pub following: Vec<twitter_client::TwitterUser>,
}

pub async fn get_user_content() -> Result<TwitterProfile, Error>{
    let user = twitter_client::find_user("ThisWeekInRust")
        .await.expect("Failed find_user in service.");
    println!("{:?}", user);

    let followers = twitter_client::get_followers(&user.id)
        .await.expect(format!("Failed to get_followers for username={}", &user.username).as_str());
    println!("{:?}", followers);

    let following = twitter_client::get_following(&user.id)
        .await.expect(format!("Failed to get_following for username={}", &user.username).as_str());
    println!("{:?}", following);

    let profile = TwitterProfile { user, followers,following };

    Ok(profile)
}
