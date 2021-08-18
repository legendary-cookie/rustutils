extern crate indicatif;
extern crate reqwest;

pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

pub async fn download_range(_path: String, _range: DownloadRange) {}
