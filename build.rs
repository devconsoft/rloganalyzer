#[macro_use]
extern crate clap;

use clap::{App, Shell};

fn main() {
    let yaml = load_yaml!("src/cli.yaml");
    let mut app = App::from_yaml(yaml);
    let outdir = std::env::var("OUT_DIR").unwrap();
    app.gen_completions(
        crate_name!(),
        Shell::Bash,
        &outdir,
    );
    app.gen_completions(
        crate_name!(),
        Shell::Bash,
        "target/debug",
    );
}
