#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate regex;
extern crate serde_yaml;
extern crate serde;

#[cfg(test)]
extern crate unindent;

pub mod config;
pub mod definition;

pub use definition::Definitions;
pub use config::Config;
