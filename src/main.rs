extern crate rustc_serialize;
#[macro_use] extern crate prettytable;

use std::path::Path;
use prettytable::Table;

mod transaction;
use transaction::{Transaction, TransactionSource, ImportableTransaction};

fn import<T>(file_path: &'static Path) -> Vec<Transaction> where T: ImportableTransaction {
    T::import(file_path)
}

fn read_csv_files(files: &[(transaction::TransactionSource, &'static str)]) -> Vec<Transaction> {
    files.iter().flat_map(|&(ref tx_type, file)| {
        let path = Path::new(file);
        match *tx_type {
            TransactionSource::Scotiabank => {
                import::<transaction::scotiabank::Csv>(path)
            },
            TransactionSource::PresidentsChoice => {
                import::<transaction::presidents_choice::Csv>(path)
            },
        }
    }).collect()
}

fn main() {
    let csv_files = [
        (TransactionSource::Scotiabank, "./data/tx/scotiabank.csv"),
        (TransactionSource::PresidentsChoice, "./data/tx/pcfinancial.csv")
    ];
    let rows = read_csv_files(&csv_files);

    let mut table = Table::new();
    table.add_row(row!["ID", "DATE", "OWNER", "AMOUNT", "SOURCE"]);

    for tx in rows {
        table.add_row(row![tx.identifier, tx.date, tx.merchant, tx.amount, tx.source]);
    }

    table.printstd();
}
