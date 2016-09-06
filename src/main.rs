extern crate quick_csv;
extern crate rustc_serialize;
#[macro_use] extern crate prettytable;

use std::path::Path;
use prettytable::Table;
mod transaction;

fn import<T>(file_path: &'static Path) -> Vec<transaction::Transaction> where T: transaction::ImportableTransaction {
    T::import(file_path)
}

fn main() {
    let rows = import::<transaction::ScotiabankTxCsv>(Path::new("./data/tx/pcbanking.csv"));

    let mut table = Table::new();
    table.add_row(row!["ID", "DATE", "OWNER", "AMOUNT", "DESCRIPTION"]);
    for tx in rows {
        table.add_row(row![tx.identifier, tx.date, tx.source, tx.amount, tx.description]);
    }

    table.printstd();
}
