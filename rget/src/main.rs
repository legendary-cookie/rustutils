mod cli;

use std::cmp::min;
use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* ARG PARSING */
    let matches = cli::build_cli().get_matches();
    let url;
    let path;
    if let Some(u) = matches.value_of("URL") {
        if u.starts_with("http://") || u.starts_with("https://") {
            url = u;
        } else {
            println!("You have to supply an url starting with either http:// or https://");
            std::process::exit(-1);
        }
    } else {
        std::process::exit(-1);
    }
    if let Some(p) = matches.value_of("PATH") {
        path = p;
    } else {
        // Use the filename from the url
        // e.g: http://www.africau.edu/images/default/sample.pdf will turn into sample.pdf
        let split = url.split("/");
        let vec: Vec<&str> = split.collect();
        path = vec[vec.len() - 1];
    }
    /* REST OF THE STUFF */
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let redirect_policy = reqwest::redirect::Policy::custom(|attempt| {
        println!("DEBUG; REDIRECT!");
        attempt.follow()
    });
    let client = reqwest::Client::builder()
        .redirect(redirect_policy)
        .user_agent(APP_USER_AGENT)
        .build()?;
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total;
    total = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", &url))?;
    println!("Total size: {:?} MB", total / 1024 / 1024);
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));
    // Download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total);
        downloaded = new;
        pb.set_position(new);
    }
    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
}
