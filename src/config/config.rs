
#![cfg_attr(feature = "cargo-clippy", warn(cyclomatic_complexity))]


#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum Command {
    ConfigCheck,
    Unknown,
}

pub struct Config {
    pub def_file: String,
    pub command: Command,
}

impl Config {

    fn new(args: &[String]) -> Config {
        let command =
            {
                if args[1] == "cfgcheck" {
                    Command::ConfigCheck
                } else {
                    Command::Unknown
                }
            };
        let def_file = args[2].clone();
        Config { def_file, command }
    }
}


#[cfg(test)]
mod tests {
    use super::{Config, Command};

    #[test]
    fn new_config() {
        let args = vec!["ranalyzer".to_string(), "cfgcheck".to_string(), "path".to_string()];
        let cfg = Config::new(&args);
        assert_eq!(cfg.command, Command::ConfigCheck);
        assert_eq!(cfg.def_file, "path".to_string());
    }
}
