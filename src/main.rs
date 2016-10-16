extern crate rustc_serialize;
extern crate chrono;
extern crate regex;
extern crate num;
#[macro_use] extern crate prettytable;
#[macro_use] extern crate lazy_static;

use std::fs;
use std::path::{Path, PathBuf};
use prettytable::Table;

mod transaction;
use transaction::{Transaction, TransactionSource, ImportableTransaction, Format};

pub mod money;
use money::Money;

fn import<T>(file_path: &Path) -> Vec<Transaction> where T: ImportableTransaction {
    T::import(file_path)
}

fn import_files(files: Vec<(TransactionSource, PathBuf)>) -> Vec<Transaction> {
    files.into_iter().flat_map(|(tx_type, path_buf)| {
        let path = path_buf.as_path().clone();
        match tx_type {
            TransactionSource::Scotiabank(format) => {
                match format {
                    Format::CSV => import::<transaction::scotiabank::Csv>(path)
                }
            },
            TransactionSource::PresidentsChoice(format) => {
                match format {
                    Format::CSV => import::<transaction::presidents_choice::Csv>(path)
                }
            },
        }
    }).collect::<Vec<Transaction>>()
}

fn main() {
    let paths = [
        (TransactionSource::Scotiabank(Format::CSV), Path::new("./data/tx/scotiabank/")),
        (TransactionSource::PresidentsChoice(Format::CSV), Path::new("./data/tx/pc/"))
    ];

    let files = paths.iter().flat_map(|&(ref format, dir)| {
        let mut acc = Vec::new();
        if dir.is_dir() {
            for file in fs::read_dir(dir).unwrap() {
                let path = file.unwrap().path();
                acc.push((format.clone(), path));
            }
        }
        
        acc
    }).collect::<Vec<_>>();

    let mut rows = import_files(files);
    rows.sort_by_key(|k| k.date);

    let mut table = Table::new();
    table.add_row(row!["ID", "DATE", "MERCHANT", "AMOUNT", "SOURCE"]);

    println!("{}", rows.iter().map(|tx| &tx.amount).sum::<Money>());

    for tx in rows {
        table.add_row(row![tx.identifier, tx.date, tx.merchant, tx.amount, tx.source]);
    }

    table.printstd();
}
