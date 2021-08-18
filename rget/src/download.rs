extern crate indicatif;
extern crate reqwest;

use indicatif::ProgressBar;
use reqwest::Client;
use reqwest::Response;

pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

pub async fn download_range(path: String, range: DownloadRange) {}
