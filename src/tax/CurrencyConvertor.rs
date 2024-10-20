use core::fmt;

use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum CurrencyConvertorError {
    CurrencyNotSupported {details: String},
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum Currency {
    UAH,
    USD,
    EUR,

    // add if you need
}

impl Currency {
    pub fn from_str(s: &str) -> Result<Currency, CurrencyConvertorError> {
        match s {
            "UAH" => Ok(Currency::UAH),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            _ => Err(CurrencyConvertorError::CurrencyNotSupported {details: s.to_string()}),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrencyRate {
    pub from: Currency,
    pub to: Currency,
    pub rate: Decimal,
    pub date: NaiveDate,
}

// received rate must represent how much of "to" currency is needed to buy 1 "from" currency
pub trait CurrencyRateProvider {

    // single date
    fn convert(&self, from: Currency, to: Currency, date: NaiveDate) -> Result<CurrencyRate, CurrencyConvertorError>;
    
    // dates range
    fn convert_range(&self, from: Currency, to: Currency, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<CurrencyRate>, CurrencyConvertorError>;
}

mod tests {
    use super::Currency;

    #[test]
    fn test_currency_enum_to_string() {
        assert_eq!(format!("{}", Currency::UAH), "UAH");
        assert_eq!(format!("{}", Currency::USD), "USD");
        assert_eq!(format!("{}", Currency::EUR), "EUR");
    }
}
