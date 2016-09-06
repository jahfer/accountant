use std::hash::{Hash, SipHasher, Hasher};
use std::path::Path;

pub mod csv_import;
pub mod scotiabank;

pub struct Transaction {
    pub identifier: u64,
    pub date: String,
    pub amount: f32,
    pub source: String,
    pub description: String,
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
