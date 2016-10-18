extern crate rustc_serialize;
extern crate chrono;
extern crate regex;
extern crate num;
#[macro_use] extern crate lazy_static;

use std::fs;
use std::path::{Path, PathBuf};
use chrono::TimeZone;

mod transaction;
use transaction::{Transaction, TransactionSource, ImportableTransaction, Format};

pub mod money;
use money::Money;

fn import<T>(file_path: &Path) -> Vec<Transaction> where T: ImportableTransaction {
    T::import(file_path)
}

fn import_transactions(files: Vec<(TransactionSource, PathBuf)>) -> Vec<Transaction> {
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
    }).collect()
}

fn spending_by_month(transactions: &Vec<Transaction>, month: u32) -> Money {
    transactions
        .clone()
        .iter()
        .filter(|tx| tx.date.date() >= chrono::UTC.ymd(2016, month % 12, 01))
        .filter(|tx| tx.date.date() <  chrono::UTC.ymd(2016, (month + 1) % 12, 01))
        .map(|tx| &tx.amount).sum::<Money>()
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
                if let Some(_) = path.extension() {
                    acc.push((format.clone(), path));
                }
            }
        }
        
        acc
    }).collect::<Vec<_>>();

    let transactions = import_transactions(files);

    for &(month, num) in [
        ("May",       5u32),
        ("June",      6u32),
        ("July",      7u32),
        ("August",    8u32),
        ("September", 9u32),
        ("October",  10u32)
    ].into_iter() {
        println!("{}: {}", month, spending_by_month(&transactions, num));
    }
}
