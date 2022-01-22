use serde::{Deserialize, Serialize};

// not implemented
#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    id: String,
    name: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestObj {
    origin: String,
}

pub async fn test_http() -> Result<TestObj, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<TestObj>()
        .await?; 
    
    Ok(resp)
}
