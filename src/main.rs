extern crate serde;

mod clients;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Application Start --");
    
    let resp = clients::twitter_client::test_http().await.expect("Failed http test.");
    println!("{:#?}", resp);
    Ok(())
}
