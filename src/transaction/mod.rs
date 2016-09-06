use std::hash::{Hash, SipHasher, Hasher};
use std::path::Path;
mod csv_import;

fn to_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = SipHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

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

#[derive(RustcDecodable)]
pub struct ScotiabankTxCsv {
    date: String,
    amount: f32,
    _unknown: Option<String>,
    description: String,
    owner: String,
}

impl Hash for ScotiabankTxCsv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.amount.to_string().hash(state);
        self.description.hash(state);
    }
}

impl ImportableTransaction for ScotiabankTxCsv {
    fn import(file_path:&'static Path) -> Vec<Transaction> {
        csv_import::read::<ScotiabankTxCsv>(file_path)
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
