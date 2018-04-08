#![allow(dead_code)]
extern crate rloganalyzer;

use rloganalyzer::datasource::TextFileDataSource;
use rloganalyzer::datasource::Data;

#[test]
fn read_data_from_textfile_datasource() {
    let ds = TextFileDataSource::new("tests/resources/data/input.txt");
    let mut iter = ds.into_iter();

    assert_eq!(iter.next(), Some(Data::new(1, "a".to_string())));
    assert_eq!(iter.next(), Some(Data::new(2, "b".to_string())));
    assert_eq!(iter.next(), Some(Data::new(3, "c".to_string())));
    assert_eq!(iter.next(), None);
}
