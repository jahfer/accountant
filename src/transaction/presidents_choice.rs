use std::path::Path;
use super::csv_import;
use super::{Transaction, ImportableTransaction, TransactionSource, Format, to_hash};
use ::money::Money;
use std::hash::{Hash, Hasher};
use chrono::{TimeZone, DateTime, Local};
use regex::Regex;

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

impl Csv {
    fn date_as_datetime(&self) -> DateTime<Local> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap();
        }

        let captures = RE.captures(&self.transaction_date).unwrap();
        let (month, day, year) = (captures.at(1), captures.at(2), captures.at(3));

        Local.ymd(
            year.unwrap().parse::<i32>().unwrap(),
            month.unwrap().parse::<u32>().unwrap(),
            day.unwrap().parse::<u32>().unwrap()
        ).and_hms(0,0,0)
    }

    // PC uses + amounts for credits, () amounts for debits
    fn formatted_amount(&self) -> String {
        let debit_re = Regex::new(r"\((.*)\)").unwrap();
        let amount_re = Regex::new(r"([\d\.,]+)").unwrap();
        let is_debit = debit_re.is_match(&self.amount);

        match amount_re.captures(&self.amount) {
            Some(captures) => {
                let amount = captures.at(1).unwrap();
                if !is_debit {
                    String::from("-") + amount
                } else {
                    amount.to_string()
                }
            },
            None => panic!("Unable to parse CSV field as money")
        }
    }
}

impl Hash for Csv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state)
    }
}

impl ImportableTransaction for Csv {
    fn import(file_path: &Path) -> Vec<Transaction> {
        csv_import::read::<Csv>(file_path, true)
            .into_iter()
            .map(|tx| Transaction {
                source: TransactionSource::PresidentsChoice(Format::CSV),
                identifier: to_hash(&tx),
                date: tx.date_as_datetime(),
                description: None,
                note: None,
                amount: Money::parse(&tx.formatted_amount()),
                merchant: tx.merchant
            }).collect()
    }
}
