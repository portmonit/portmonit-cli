use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::vec;

pub struct TaxPolicy {
    pub sub_policies: Vec<TaxSpecByDate>,
}

#[derive(Debug, Clone)]
pub struct TaxSpecByDate {
    pub personal_income_tax: Decimal,
    pub military_tax: Decimal,
    pub start_date: NaiveDate,
}

pub fn default_tax_policy() -> TaxPolicy {
    TaxPolicy {
        sub_policies: vec![
            TaxSpecByDate {
                personal_income_tax: dec!(0.18),
                military_tax: dec!(0.015),

                // not sure it is needed to be added policy before this date
                start_date: NaiveDate::from_ymd(2020, 1, 1),
            },
            TaxSpecByDate {
                personal_income_tax: dec!(0.18),
                military_tax: dec!(0.05),
                start_date: NaiveDate::from_ymd(2024, 10, 1),
            },
        ],
    }
}
