mod cli;
extern crate utils;

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
    let mut multiple = false;
    let threads = clap::value_t!(matches.value_of("threads"), u64).unwrap_or_else(|e| e.exit());
    if matches.value_of("multiple").is_some() {
        multiple = true;
    }
    if let Some(u) = matches.value_of("URL") {
        if multiple {
            let _urls = u.split(',');
            // TODO: implement multiple urls
            std::process::exit(-1);
        } else if u.starts_with("http://") || u.starts_with("https://") {
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
        let split = url.split('/');
        let vec: Vec<&str> = split.collect();
        path = vec[vec.len() - 1];
    }
    /* REST OF THE STUFF */
    let client = &utils::factory::build_client()?;
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|_| format!("Failed to GET from '{}'", &url))?;
    if res.status() != 200 && res.status() != 206 {
        println!("Got Status {}", res.status());
        std::process::exit(-1);
    }
    let total = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", &url))?;
    println!("Total size: {}", common::byteconvert::convert(total as f64));
    let headers = res.headers();
    if threads > 1 {
        if headers.contains_key(reqwest::header::ACCEPT_RANGES) {
            let mut threadmap: Vec<Option<tokio::task::JoinHandle<()>>> = Vec::new();
            let single = total / threads;
            let mut i = 0;
            let mut last = 0;
            while i < threads + 1 {
                if i == 0 {
                    i += 1
                } else {
                    let range = utils::download::DownloadRange {
                        start: last,
                        end: single * i,
                    };
                    let localpath = format!("{}~{}", <&str>::clone(&path), i);
                    let localurl = <&str>::clone(&url).to_string();
                    /*
                    println!(
                        "{} - {}",
                        common::byteconvert::convert(last as f64),
                        common::byteconvert::convert((single * i) as f64)
                    );
                    */
                    let handle = tokio::spawn(async move {
                        // println!("CHUNK TO {}", localpath);
                        utils::download::download_range(&localurl, &localpath, range)
                            .await
                            .unwrap();
                    });
                    threadmap.push(Some(handle));
                    last = single * i + 1;
                    i += 1;
                }
            }
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .map_err(|_| format!("Failed to create file '{}'", path))?;
            let mut a = 1;
            for t in threadmap.iter_mut() {
                if let Some(handle) = t.take() {
                    handle.await.expect("Something wrent wrong in a task");
                    let fname = format!("{}~{}", path, a);
                    let mut reader = my_reader::BufReader::open(fname.clone())?;
                    let mut buffer = String::new();
                    while let Some(line) = reader.read_line(&mut buffer) {
                        f.write_all(line?.as_bytes())?;
                    }
                    std::fs::remove_file(fname)?;
                }
                a += 1;
            }
            std::process::exit(0);
        } else {
            println!(
                "The server doesn't support ranges. 
            Specify only one thread to download from here."
            );
            std::process::exit(-1);
        }
    }
    // Download for single threaded stuff
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));
    let mut file = File::create(path).map_err(|_| format!("Failed to create file '{}'", path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or_else(|_| Err("Error while downloading file".to_string()))?;
        file.write(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
        let new = min(downloaded + (chunk.len() as u64), total);
        downloaded = new;
        pb.set_position(new);
    }
    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
