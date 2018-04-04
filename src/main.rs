
#[macro_use]
extern crate clap;
extern crate rloganalyzer;

use clap::App;
use rloganalyzer::Config;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let _matches = App::from_yaml(yaml).get_matches();
    //let cfg = Config::new(matches);
}
