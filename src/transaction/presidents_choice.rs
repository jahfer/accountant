use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, TransactionSource, to_hash};
use std::hash::{Hash, Hasher};

#[derive(RustcDecodable,Clone)]
pub struct Csv {
    transaction_date: String,
    _posting_date: String,
    amount: f32,
    merchant: String,
    _merchant_city: String,
    _merchant_state: String,
    _merchant_zip: String,
    reference: String,
    _direction: char,
    _sicmcc_code: u16
}

impl Hash for Csv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state)
    }
}

impl ImportableTransaction for Csv {
    fn import(file_path:&'static Path) -> Vec<Option<Transaction>> {
        csv_import::read::<Csv>(file_path, true)
            .into_iter()
            .map(|scotia_tx| {
                match scotia_tx {
                    Some(tx) => Some(Transaction {
                        source: TransactionSource::PresidentsChoice,
                        identifier: to_hash(&tx),
                        date: tx.transaction_date,
                        amount: tx.amount,
                        merchant: tx.merchant,
                        description: None,
                        note: None
                    }),
                    None => None
                }
            }).collect()
    }
}
