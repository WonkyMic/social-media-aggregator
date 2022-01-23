#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod clients;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Application Start --");

    let twit_user = clients::twitter_client::find_user("wonkymic").await.expect("Failed find_user in service.");
    println!("{:?}", twit_user);

    Ok(())
}
