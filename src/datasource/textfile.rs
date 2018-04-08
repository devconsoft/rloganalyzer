
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use datasource::Data;

pub struct TextFileDataSource {
    reader: BufReader<::std::fs::File>,
}

impl TextFileDataSource {
    pub fn new(path: &str) -> TextFileDataSource {
        let f = File::open(path).unwrap();
        TextFileDataSource {
            reader: BufReader::new(f)
        }
    }
}

impl IntoIterator for TextFileDataSource {
    type Item = Data;
    type IntoIter = ::std::iter::Map<::std::iter::Enumerate<::std::io::Lines<::std::io::BufReader<::std::fs::File>>>,
        fn((usize, Result<String, ::std::io::Error>)) -> Data>;

    fn into_iter(self) -> Self::IntoIter {
        fn convert_to_data(x: (usize, Result<String, ::std::io::Error>)) -> Data {
            Data::new(x.0 + 1, x.1.unwrap())
        }
        self.reader.lines().enumerate().map(convert_to_data)
    }
}
