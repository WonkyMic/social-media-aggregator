use crate::data::twitter;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref BEARER: &'static str = dotenv!("AIRTABLE_BEARER");
}

lazy_static! {
    pub static ref URL: &'static str = "https://api.airtable.com/v0";
}

#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub records: Vec<TwitterUserRecord>
}
#[derive(Debug, Deserialize)] 
pub struct TwitterUserRecord { 
    pub id: String,
    pub fields: twitter::TwitterUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub id: String,
    pub name: String,
    pub username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Records {
    pub fields: Vec<Fields>
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct CreateTwitterUserRequest { 
    pub records: Records
}
