#![feature(never_type)]

mod cli;
extern crate utils;

use crossterm::execute;
use pbr::MultiBar;
use std::fs::File;
use utils::download::download;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => cleanup(0),
        Err(err) => {
            println!("{}", err);
            cleanup(-1)
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Hide cursor
    execute!(std::io::stdout(), crossterm::cursor::Hide)?;
    /* ARG PARSING */
    let matches = cli::build_cli().get_matches();
    let url;
    let path;
    let mut progb = true;
    let threads = clap::value_t!(matches.value_of("threads"), u64).unwrap_or_else(|e| e.exit());
    if matches.is_present("noprog") {
        progb = false;
    }
    if let Some(u) = matches.value_of("URL") {
        if u.starts_with("http://") || u.starts_with("https://") {
            url = u;
        } else {
            println!("You have to supply an url starting with either http:// or https://");
            return Ok(());
        }
    } else {
        return Ok(());
    }
    if let Some(p) = matches.value_of("PATH") {
        if !std::path::Path::new(p).is_dir() {
            path = p;
        } else {
            let split = url.split('/');
            let vec: Vec<&str> = split.collect();
            let tpath = vec[vec.len() - 1];
            std::env::set_current_dir(p)?;
            path = tpath;
        }
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
        return Ok(());
    }
    let total = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", &url))?;
    println!("Total size: {}", common::byteconvert::convert(total as f64));
    let headers = res.headers();
    let f = File::create(path).map_err(|_| format!("Failed to create file '{}'", path))?;
    f.set_len(total)?;
    if threads > 1 {
        if headers.contains_key(reqwest::header::ACCEPT_RANGES) {
            let mb = MultiBar::new();
            let mut threadmap: Vec<Option<tokio::task::JoinHandle<()>>> = Vec::new();
            let single = total / threads;
            let mut i = 0;
            let mut last = 0;
            while i < threads + 1 {
                if i == 0 {
                    // we dont want to do *0
                    i += 1
                } else {
                    let range = utils::download::DownloadRange {
                        start: last,
                        end: single * i,
                    };
                    let localpath = <&str>::clone(&path).to_string();
                    let localurl = <&str>::clone(&url).to_string();
                    //println!("{} - {}",common::byteconvert::convert(last as f64),common::byteconvert::convert((single * i) as f64));
                    let p = mb.create_bar(single);
                    let handle = tokio::spawn(async move {
                        utils::download::download_range(&localurl, &localpath, range, p)
                            .await
                            .unwrap();
                    });
                    threadmap.push(Some(handle));
                    last = single * i + 1;
                    i += 1;
                }
            }
            let time_begin = std::time::SystemTime::now();
            mb.listen();
            for t in threadmap.iter_mut() {
                if let Some(handle) = t.take() {
                    handle.await.expect("Something wrent wrong in a task");
                }
            }
            match time_begin.elapsed() {
                Ok(elapsed) => {
                    println!("Downloading took {} seconds", elapsed.as_secs());
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            return Ok(());
        } else {
            println!(
                "WARNING: The server doesn't support ranges. 
                We will download with a single thread to support this server."
            );
        }
    }
    // Download for single threaded stuff / fallback
    download(path, progb, total, res).await?;
    Ok(())
}

fn cleanup(ret: i32) -> ! {
    execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
    std::process::exit(ret)
}
