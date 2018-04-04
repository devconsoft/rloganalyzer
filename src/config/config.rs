
#![cfg_attr(feature = "cargo-clippy", warn(cyclomatic_complexity))]

use super::Command;

pub struct Config {
    pub def_file: String,
    pub command: Command,
    pub in_file: String,
    pub out_file: String,
    pub summary_file: String,
    pub verbosity: u8,
}

impl Config {

    pub fn new(def_file: &str,
            command: Command,
            in_file: &str,
            out_file: &str,
            summary_file: &str,
            verbosity: u8,
            ) -> Config {
        let def_file = def_file.to_string();
        let in_file = in_file.to_string();
        let out_file = out_file.to_string();
        let summary_file = summary_file.to_string();
        Config {
            def_file,
            command,
            in_file,
            out_file,
            summary_file,
            verbosity,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Config, Command};

    #[test]
    fn config_new() {
        let cfg = Config::new("def_file", Command::Scan, "in_file", "out_file", "summary_file", 1);
        assert_eq!(cfg.def_file, "def_file");
    }
}
