use lazy_static::lazy_static;
use crate::data::airtable;
use crate::data::twitter;

pub async fn find_user(name: &str) -> Result<twitter::TwitterUser, reqwest::Error> {    
    let full_url = format!("{}/users/by/username/{}", *airtable::URL, name);

    let client = build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<twitter::UserResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn create_user(user: twitter::TwitterUser) -> Result<(), reqwest::Error> {

    let fields = vec![airtable::Fields{ id: user.id, name: user.name, username: user.username }];


    let records = airtable::Records { fields };

    let record = airtable::CreateTwitterUserRequest { records };

    println!("Record = {:?}", &record);

    let full_url = format!("{}/Twitter%20User", *airtable::URL);

    let client = build_client().unwrap();
    let resp = client.post(full_url)
        .json(&record)
        .send()
        .await?
        .text()
        //.json::<airtable::CreateResponse>()
        .await?;

    println!("CreateResponse = {:?}", resp);

    Ok(())
}

fn build_client() -> Result<reqwest::Client, reqwest::Error> {
    let bearer = format!("Bearer {}", *airtable::BEARER);
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
