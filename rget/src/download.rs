extern crate indicatif;
extern crate reqwest;

use indicatif::ProgressBar;
use reqwest::Client;
use reqwest::Response;

pub struct DownloadRange {
    start: u64,
    end: u64,
}

pub async fn download_range(path: &str, range: &DownloadRange) {}
