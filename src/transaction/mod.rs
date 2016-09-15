use std::hash::{Hash, SipHasher, Hasher};
use std::path::Path;
use std::fmt::{self, Debug};

pub mod csv_import;
pub mod scotiabank;
pub mod presidents_choice;

#[derive(Debug)]
pub enum TransactionSource {
    Scotiabank,
    PresidentsChoice
}

impl fmt::Display for TransactionSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub struct Transaction {
    pub identifier: u64,
    pub source: TransactionSource,
    pub date: String,
    pub amount: f32,
    pub merchant: String,
    pub description: Option<String>,
    pub note: Option<String>
}

pub trait ImportableTransaction {
    fn import(file_path: &'static Path) -> Vec<Transaction>;
}

fn to_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = SipHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
