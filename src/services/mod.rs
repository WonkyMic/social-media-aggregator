mod twitter;

pub async fn create(name: &str, social: &str) {
    let twitter_profile = twitter::test(name)
        .await.expect("Error attempting to create Twitter Profile.");
}
