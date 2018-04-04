
#[macro_use]
extern crate clap;
extern crate rloganalyzer;

use clap::{App, ArgMatches};
use rloganalyzer::Config;
use rloganalyzer::config::Command;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let cfg = get_config(&matches);
    print_config(&cfg);
}

fn get_config(matches: &ArgMatches) -> Config {
    let def_file = matches.value_of("config").unwrap();
    let command = Command::Scan;
    let in_file = matches.value_of("input").unwrap_or("-");
    let out_file = matches.value_of("output").unwrap_or("-");
    let summary_file = matches.value_of("summary").unwrap_or("-");
    let verbosity = matches.occurrences_of("verbose") as u8;
    Config::new(def_file, command, in_file, out_file, summary_file, verbosity)
}

fn print_config(cfg: &Config) {
    if cfg.verbosity > 0 {
        println!("def      : {}", cfg.def_file);
        println!("command  : {:?}", cfg.command);
        println!("in       : {}", cfg.in_file);
        println!("out      : {}", cfg.out_file);
        println!("summary  : {}", cfg.summary_file);
        println!("verbosity: {}", cfg.verbosity);
    }
}
