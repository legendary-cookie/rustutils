extern crate indicatif;
extern crate reqwest;

use crate::factory;

pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

pub async fn download_range(
    _path: String,
    _range: DownloadRange,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = &factory::build_client()?;
    Ok(())
}
