#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod clients;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Application Start --");

    // Twitter
    let twitter_profile = services::twitter_service::get_user_content("ThisWeekInRust")
        .await.expect("Error attempting to generate Twitter proflie.");
    println!("Twitter Profile = {:?}", twitter_profile);
    
    Ok(())
}
