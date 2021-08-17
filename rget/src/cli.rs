use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("rget")
        .version(env!("CARGO_PKG_VERSION"))
        .author("legendary-cookie <github.com@xolley.de>")
        .arg(
            Arg::with_name("multiple")
                .help("Enable downloading a list of urls")
                .required(false)
                .short("m")
                .long("multiple"),
        )
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