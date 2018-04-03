#![allow(dead_code)]
extern crate rloganalyzer;

use rloganalyzer::Definitions;

#[test]
fn definitions_from_yaml_file() {
    let defs = Definitions::from_yaml("tests/resources/config/minimal.yaml");
    assert_eq!(defs.len(), 3);
    assert_eq!(defs.get(0).markers[0].pattern.as_str(), "first");
}
