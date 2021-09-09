extern crate indicatif;
extern crate reqwest;

use crate::factory;
use futures_util::StreamExt;
use reqwest::header::RANGE;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

impl DownloadRange {
    pub fn get_size(&self) -> u64 {
        return self.end - self.start;
    }
}

pub async fn download_range(
    url: &str,
    path: &str,
    range: DownloadRange,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = &factory::build_client()?;
    let res = client
        .get(url)
        .header(RANGE, format!("bytes={}-{}", range.start, range.end))
        .send()
        .await
        .map_err(|_| format!("Failed to GET from '{}'", &url))?;
    // If status code is not 206 [=> partial content]
    if res.status() != 206 {
        println!("Got Status {}", res.status());
        std::process::exit(-1);
    }
    // Download
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .append(false)
        .open(path)?;
    file.seek(SeekFrom::Start(range.start))?;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|_| "Error while downloading file".to_string())?;
        file.write(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
    }
    Ok(())
}
