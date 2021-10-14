use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("rget")
        .version(env!("CARGO_PKG_VERSION"))
        .author("legendary-cookie <github.com@xolley.de>")
        .arg(
            Arg::with_name("threads")
                .help("How many threads to use")
                .required(false)
                .default_value("1")
                .takes_value(true)
                .short("t")
                .long("threads"),
        )
        .arg(
            Arg::with_name("noprog")
                .help("Disable progress bar")
                .required(false)
                .short("p")
                .long("noprog"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
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
