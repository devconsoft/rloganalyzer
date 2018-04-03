#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate serde_yaml;

#[cfg(test)]
extern crate unindent;

pub mod config;
pub mod definition;

pub use definition::definitions::Definitions;
