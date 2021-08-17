use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("rget")
        .version("0.1.0")
        .author("Vincent S. <github.com@xolley.de>")
        .help("Download files with a nice little progress bar")
        .arg(
            Arg::with_name("URL")
                .help("The url to download the files")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PATH")
                .help("The file location")
                .required(false)
                .index(2),
        )
}
