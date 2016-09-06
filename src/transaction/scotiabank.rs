use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, to_hash};
use std::hash::{Hash, Hasher};

#[derive(RustcDecodable)]
pub struct Csv {
    date: String,
    amount: f32,
    _unknown: Option<String>,
    description: String,
    owner: String,
}

impl Hash for Csv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.amount.to_string().hash(state);
        self.description.hash(state);
    }
}

impl ImportableTransaction for Csv {
    fn import(file_path:&'static Path) -> Vec<Transaction> {
        csv_import::read::<Csv>(file_path)
            .map(|tx| Transaction {
                identifier: to_hash(&tx),
                date: tx.date,
                amount: tx.amount,
                source: tx.owner,
                description: tx.description,
                note: None
            }).collect()
    }
}
