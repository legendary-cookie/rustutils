extern crate clap;

use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    app.gen_completions(
        "rget",      // We need to specify the bin name manually
        Shell::Bash, // Then say which shell to build completions for
        outdir,
    ); // Then say where write the completions to
}
