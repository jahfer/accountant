extern crate quick_csv;
extern crate rustc_serialize;

use std::path::Path;

pub fn read<'a, T: 'a>(file_path: &'a Path, header: bool) -> Vec<Option<T>>
    where T: rustc_serialize::Decodable {
    let csv = quick_csv::Csv::from_file(file_path).unwrap();
    csv.has_header(header).map(|row| {
        match row {
            Ok(row) => Some(row.decode::<T>().unwrap()),
            Err(_) => None
        }
    }).collect()
}
