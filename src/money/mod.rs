use num::{BigInt, Zero};
use regex::Regex;
use std::str::FromStr;
use std::fmt;
use std::iter::Sum;

use std::ops::{Add};

#[derive(Debug)]
pub struct Money {
    cents: BigInt
}

impl Money {
    pub fn new(input: BigInt) -> Money {
        Money { cents: input }
    }

    pub fn parse(input: &str) -> Money {
        Money { cents: Money::extract_money(input) }
    }

    pub fn empty() -> Money {
        Money { cents: BigInt::zero() }
    }

    fn extract_money(input: &str) -> BigInt {
        if input.len() == 0 { return BigInt::zero() }

        lazy_static! {
            static ref RE1: Regex = Regex::new(r"(-?[\d\.,]+)").unwrap();
            static ref RE2: Regex = Regex::new(r"^(?P<sign>-)?(?P<trimmed_decimals>0[,.]\d\d)\d+$").unwrap();
            static ref RE3: Regex = Regex::new(r"^(.*?)(?:[.,](\d{1,2}))?$").unwrap();
        }

        match RE1.captures(&input).unwrap().at(1) {
            Some(amount) => {
                // Convert 0.123 or 0,123 into what will be parsed as a decimal amount 0.12 or 0.13
                let formatted = RE2.replace_all(amount, "$sign$rounded_decimals");
                let segments = RE3.captures(&formatted).unwrap();
                if segments.is_empty() { return BigInt::zero() }
                let str_amount = String::from(segments.at(1).unwrap()) + segments.at(2).unwrap();
                BigInt::from_str(&str_amount).unwrap()
            },
            None => {
                println!("NaN");
                BigInt::zero()
            }
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_re = Regex::new(r"^(?P<sign>-?)(?P<integer>[\d]+)(?P<scale>\d{2})$").unwrap();
        let formatted = format_re.replace(self.cents.to_string().as_str(), "$sign$integer.$scale");
        write!(f, "{}", formatted)
    }
}

impl Sum<Money> for Money {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item=Money> {
        iter.fold(Money::empty(), |acc, item| acc + item)
    }
}

impl<'a> Sum<&'a Money> for Money {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item=&'a Money> {
        iter.fold(Money::empty(), |acc, item| acc + item)
    }
}

impl<'a, 'b> Add<&'b Money> for &'a Money {
    type Output = Money;

    #[inline]
    fn add(self, other: &Money) -> Money {
        Money::new(&self.cents + &other.cents)
    }
}

impl<'a> Add<Money> for &'a Money {
    type Output = Money;

    #[inline]
    fn add(self, other: Money) -> Money {
        Money::new(&self.cents + other.cents)
    }
}

impl<'a> Add<&'a Money> for Money {
    type Output = Money;

    #[inline]
    fn add(self, other: &Money) -> Money {
        Money::new(self.cents + &other.cents)
    }
}

impl Add<Money> for Money {
    type Output = Money;

    #[inline]
    fn add(self, other: Money) -> Money {
        Money::new(self.cents + other.cents)
    }
}
