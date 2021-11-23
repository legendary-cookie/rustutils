extern crate reqwest;

use crate::factory;
use futures_util::StreamExt;
use pbr::{Pipe, ProgressBar, Units};
use reqwest::header::RANGE;
use std::cmp::min;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

pub struct DownloadRange {
    pub start: u64,
    pub end: u64,
}

impl DownloadRange {
    pub fn get_size(&self) -> u64 {
        self.end - self.start
    }
}

pub async fn download_range(
    url: &str,
    path: &str,
    range: DownloadRange,
    mut pbar: ProgressBar<Pipe>,
) -> Result<(), Box<dyn std::error::Error>> {
    pbar.set_units(Units::Bytes);
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
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        // Write chunk
        let chunk = item.map_err(|_| "Error while downloading file".to_string())?;
        file.write(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
        // Update progress bar
        let new = min(downloaded + (chunk.len() as u64), range.get_size());
        downloaded = new;
        pbar.set(new);
    }
    pbar.finish();
    Ok(())
}

pub async fn download_from_url(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = factory::build_client()?;
    let res = client.get(url).send().await?;
    let mut file =
        std::fs::File::create(path).map_err(|_| format!("Failed to create file '{}'", path))?;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|_| "Error while downloading file".to_string())?;
        file.write(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
    }
    Ok(())
}

pub async fn download(
    path: &str,
    progb: bool,
    total: u64,
    res: reqwest::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pb = ProgressBar::new(total);
    pb.set_units(Units::Bytes);
    let mut file =
        std::fs::File::create(path).map_err(|_| format!("Failed to create file '{}'", path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|_| "Error while downloading file".to_string())?;
        file.write(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
        if progb && total >= 1000000 {
            let new = min(downloaded + (chunk.len() as u64), total);
            downloaded = new;
            pb.set(new);
        }
    }
    if progb {
        pb.finish_print("Finished with downloading!");
    }
    Ok(())
}
