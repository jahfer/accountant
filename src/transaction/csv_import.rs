extern crate quick_csv;
extern crate rustc_serialize;

use std::path::Path;

pub fn read<'a, T: 'a>(file_path: &'a Path) -> Box<Iterator<Item=T> + 'a>
    where T: rustc_serialize::Decodable {
    let csv = quick_csv::Csv::from_file(file_path).unwrap();
    let iter = csv.into_iter().map(|row| row.unwrap().decode::<T>().unwrap());
    Box::new(iter)
}
