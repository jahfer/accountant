use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, TransactionSource, to_hash};
use std::hash::{Hash, Hasher};

#[derive(RustcDecodable,Clone)]
pub struct Csv {
    date: String,
    amount: f32,
    _unknown: Option<String>,
    description: String,
    merchant: String,
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
        csv_import::read::<Csv>(file_path, false)
            .into_iter()
            .map(|tx| Transaction {
                source: TransactionSource::Scotiabank,
                identifier: to_hash(&tx),
                date: tx.date,
                amount: tx.amount,
                merchant: tx.merchant,
                description: Some(tx.description),
                note: None
            }).collect()
    }
}
