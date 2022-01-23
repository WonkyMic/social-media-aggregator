#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod clients;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Application Start --");

    // Twitter Domain Object?
    let twit_user = clients::twitter_client::find_user("ThisWeekInRust").await.expect("Failed find_user in service.");
    println!("{:?}", twit_user);

    let twit_user_followers = clients::twitter_client::get_followers(&twit_user.id).await.expect(format!("Failed to get_followers for username={}", &twit_user.username).as_str());
    println!("{:?}", twit_user_followers);

    let twit_user_following = clients::twitter_client::get_following(&twit_user.id).await.expect(format!("Failed to get_following for username={}", &twit_user.username).as_str());
    println!("{:?}", twit_user_following);

    Ok(())
}
