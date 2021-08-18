use indicatif::ProgressBar;
use reqwest::Client;
use reqwest::Response;

pub async fn download_range(client: &Client, res: &Response, url: &str, path: &str, range: &u64) {}
