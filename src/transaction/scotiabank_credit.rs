use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, TransactionSource, Format, to_hash};
use ::money::Money;
use chrono::{TimeZone, DateTime, UTC};
use std::hash::{Hash, Hasher};
use regex::Regex;

#[derive(RustcDecodable)]
pub struct Csv {
    date: String,
    description: String,
    amount: String
}

impl Csv {
    fn date_as_datetime(&self) -> DateTime<UTC> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{1,2})$").unwrap();
        }

        let captures = RE.captures(&self.date).unwrap();
        let (month, day, year) = (captures.at(1), captures.at(2), captures.at(3));

        UTC.ymd(
            (year.unwrap().parse::<i32>().unwrap() + 2000),
            month.unwrap().parse::<u32>().unwrap(),
            day.unwrap().parse::<u32>().unwrap()
        ).and_hms(0,0,0)
    }
}

impl Hash for Csv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.amount.to_string().hash(state);
        self.description.hash(state);
    }
}

impl ImportableTransaction for Csv {
    fn import(file_path: &Path) -> Vec<Transaction> {
        csv_import::read::<Csv>(file_path, false)
            .into_iter()
            .map(|tx| Transaction {
                source: TransactionSource::ScotiabankCredit(Format::CSV),
                identifier: to_hash(&tx),
                date: tx.date_as_datetime(),
                amount: Money::parse(&tx.amount),
                merchant: tx.description.clone(),
                description: Some(tx.description),
                note: None
            }).collect()
    }
}
