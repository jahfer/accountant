use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, TransactionSource, to_hash};
use std::hash::{Hash, Hasher};

#[derive(RustcDecodable,Clone)]
pub struct Csv {
    transaction_date: String,
    _posting_date: String,
    amount: String,
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

// PC uses + amounts for credits, () amounts for debits
fn convert_amount(amount: String) -> f32 {
    let mut trimmed = amount;
    if '(' == trimmed.chars().next().unwrap() {
        let parens: &[char] = &['(', ')'];
        trimmed = trimmed.trim_matches(parens).trim_left_matches('$').to_owned();
    } else {
        trimmed = String::from("-") + trimmed.trim_left_matches('$');
    }

    trimmed.parse::<f32>().unwrap()
}

impl ImportableTransaction for Csv {
    fn import(file_path:&'static Path) -> Vec<Transaction> {
        csv_import::read::<Csv>(file_path, true)
            .into_iter()
            .map(|tx| Transaction {
                source: TransactionSource::PresidentsChoice,
                identifier: to_hash(&tx),
                date: tx.transaction_date,
                amount: convert_amount(tx.amount),
                merchant: tx.merchant,
                description: None,
                note: None
            }).collect()
    }
}
