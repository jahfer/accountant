extern crate csv;
extern crate rustc_serialize;

use std::path::Path;

pub fn read<'a, T: 'a>(file_path: &'a Path, header: bool) -> Vec<T>
    where T: rustc_serialize::Decodable {
    let csv = csv::Reader::from_file(file_path).unwrap();
    csv.has_headers(header).decode().collect::<csv::Result<Vec<T>>>().unwrap()
}
