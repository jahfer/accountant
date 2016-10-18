use std::hash::{Hash, SipHasher, Hasher};
use std::path::Path;
use std::fmt::{self, Debug};
use chrono::datetime::{DateTime};
use chrono::UTC;
use ::money::Money;

pub mod csv_import;
pub mod scotiabank;
pub mod presidents_choice;

#[derive(Debug, Clone)]
pub enum Format { CSV }

#[derive(Debug, Clone)]
pub enum TransactionSource {
    Scotiabank(Format),
    PresidentsChoice(Format)
}

impl fmt::Display for TransactionSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub identifier: u64,
    pub source: TransactionSource,
    pub date: DateTime<UTC>,
    pub amount: Money,
    pub merchant: String,
    pub description: Option<String>,
    pub note: Option<String>
}

pub trait ImportableTransaction {
    fn import(file_path: &Path) -> Vec<Transaction>;
}

fn to_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = SipHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
