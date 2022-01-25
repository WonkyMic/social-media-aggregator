#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod clients;
mod data;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- Application Start --");

    services::create("ThisWeekInRust", "twitter").await;
    
    Ok(())
}
