use std::io::Error;
use crate::clients::airtable_client;
use crate::clients::twitter_client;
use crate::data::twitter;

#[derive(Debug)]
pub struct TwitterProfile {
    pub user: twitter::TwitterUser,
    pub followers: Vec<twitter::TwitterUser>,
    pub following: Vec<twitter::TwitterUser>,
    pub recent: Vec<twitter::Tweet>,
}

pub async fn get_user_content(name: &str) -> Result<(), Error> {     
    let user = twitter_client::find_user(name)
        .await.expect("Failed find_user in service.");
    println!("{:?}", user); 

    let followers = twitter_client::get_followers(&user.id)
        .await.expect(format!("Failed to get_followers for username={}", &user.username).as_str());
    
    let following = twitter_client::get_following(&user.id)
        .await.expect(format!("Failed to get_following for username={}", &user.username).as_str());
    
    let recent = twitter_client::get_recent_tweets(&user.id)
        .await.expect(format!("Failed to get_recent_tweets for username={}", &user.username).as_str()); 
    
    let save_user = airtable_client::create_user(user).await.expect("Failed to create_user");

    //let profile = TwitterProfile { user, followers, following, recent }; 

    Ok(())
}
