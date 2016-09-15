extern crate quick_csv;
extern crate rustc_serialize;
#[macro_use] extern crate prettytable;

use std::path::Path;
use prettytable::Table;
mod transaction;

use transaction::{Transaction, TransactionSource, ImportableTransaction};

fn import<T>(file_path: &'static Path) -> Vec<Option<Transaction>> where T: ImportableTransaction {
    T::import(file_path)
}

fn main() {
    let files = [
        (TransactionSource::Scotiabank, "./data/tx/pcbanking.csv"),
        (TransactionSource::PresidentsChoice, "./data/tx/pcfinancial.csv")
    ];

    let rows = files
        .iter()
        .flat_map(|&(ref tx_type, file)| {
            let path = Path::new(file);
            match *tx_type {
                TransactionSource::Scotiabank => {
                    import::<transaction::scotiabank::Csv>(path)
                },
                TransactionSource::PresidentsChoice => {
                    import::<transaction::presidents_choice::Csv>(path)
                },
            }
        });

    let mut table = Table::new();
    table.add_row(row!["ID", "DATE", "OWNER", "AMOUNT", "SOURCE"]);
    for maybe_tx in rows {
        match maybe_tx {
            Some(tx) => { table.add_row(row![tx.identifier, tx.date, tx.merchant, tx.amount, tx.source]); },
            None => { println!("Failed to read row"); }
        }
    }

    table.printstd();
}
