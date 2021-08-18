pub fn build_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let redirect_policy = reqwest::redirect::Policy::custom(|attempt| {
        //println!("DEBUG; REDIRECT!");
        attempt.follow()
    });
    let client = reqwest::Client::builder()
        .redirect(redirect_policy)
        .user_agent(APP_USER_AGENT)
        .build()?;
    Ok(client)
}
