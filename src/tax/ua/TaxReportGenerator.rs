use rust_decimal::Decimal;
use chrono::NaiveDate;
use super::NationalBank::NationalBank;
use super::TaxPolicy::*;
use super::BrokerReportProvider::*;
use crate::tax::CurrencyConvertor::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::*;
use tabled::{Tabled, Table};

#[derive(Debug)]
pub struct UaTax {
    pub personal_income_tax: Decimal,
    pub military_tax: Decimal,
}

impl UaTax {
    pub fn round_dp(&self, dp: u32) -> UaTax {
        UaTax {
            personal_income_tax: self.personal_income_tax.round_dp(dp),
            military_tax: self.military_tax.round_dp(dp),
        }
    }
}

impl UaTax {
    pub fn new(personal_income_tax: Decimal, military_tax: Decimal) -> UaTax {
        UaTax {
            personal_income_tax,
            military_tax,
        }
    }

    pub fn total(&self) -> Decimal {
        self.personal_income_tax + self.military_tax
    }
}

impl Add for UaTax {
    type Output = UaTax;

    fn add(self, other: UaTax) -> UaTax {
        UaTax {
            personal_income_tax: self.personal_income_tax + other.personal_income_tax,
            military_tax: self.military_tax + other.military_tax,
        }
    }
}

#[derive(Debug)]
pub struct UaInvestmentTaxReport {
    pub dividend_ops: UaTaxDividend,
    pub investment_ops: UaTaxInvestmentOps,
}

#[derive(Debug)]
pub struct UaTaxDividend {
    pub income_total: Decimal,
    pub total_tax: UaTax,
}

impl std::fmt::Display for UaTaxDividend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Income({}), Tax({})", self.income_total, self.total_tax.total())
    }
}

#[derive(Debug)]
pub struct UaTaxInvestmentOps {
    pub trades: Vec<UaTradeReport>,
    pub total_fin_result: Decimal,
    pub total_tax: UaTax,
}

#[derive(Debug)]
pub struct UaTradeReport {
    pub buy_date: NaiveDate,
    pub sell_date: NaiveDate,
    pub name: String,
    pub buy_price_uah: Decimal,
    pub sell_price_uah: Decimal,
    pub fin_result_uah: Decimal,
}

#[derive(Debug)]
pub enum UaTaxReportGeneratorError {
    BrokerError,
    BrokerReportDatesRandeError(String),
    TaxPolicyError,
    CurrencyConvertorError(String),
}

pub struct UaTaxReportGenerator {
    tax_policy: TaxPolicy,
    broker_report_provider: BrokerReportProviderBox,
    national_bank_rate_provider: NationalBank,
}

impl UaTaxReportGenerator {
    pub fn new(tax_policy: TaxPolicy, broker_report_provider: BrokerReportProviderBox) -> UaTaxReportGenerator {    
        UaTaxReportGenerator {
            tax_policy,
            broker_report_provider,
            national_bank_rate_provider: NationalBank{},
        }
    }

    // unformal tax report is a report that can't be used for tax declaration
    pub fn get_unformal_tax_report(&self) -> Result<UaInvestmentTaxReport, UaTaxReportGeneratorError> {    
        let broker_report = self.broker_report_provider.get_broker_report();

        let mut investment_tax_report = UaInvestmentTaxReport {
            dividend_ops: UaTaxDividend {
                income_total: Decimal::new(0, 0),
                total_tax: UaTax::new(Decimal::new(0, 0), Decimal::new(0, 0)),
            },
            investment_ops: UaTaxInvestmentOps {
                trades: Vec::new(),
                total_fin_result: Decimal::new(0, 0),
                total_tax: UaTax::new(Decimal::new(0, 0), Decimal::new(0, 0)),
            },
        };

        let (earliest_date, latest_date) = self.get_total_range_by_report(&broker_report)?;
        let currencies = broker_report.trades.iter().map(|trade| { trade.currency }).collect::<HashSet<Currency>>();

        let preserved_rated_by_currency = self.preserve_convert_rate_by_currencies(currencies, earliest_date, latest_date)?;

        for trade in &broker_report.trades {
            let buy_price = trade.buy_price;
            let sell_price = trade.sell_price;
            let currency = trade.currency;

            let buy_price_uah = self.convert_to_uah_from_preserved_by_currency(buy_price, currency, trade.buy_date, &preserved_rated_by_currency)?.round_dp(2);
            let sell_price_uah = self.convert_to_uah_from_preserved_by_currency(sell_price, currency, trade.sell_date, &preserved_rated_by_currency)?.round_dp(2);

            let fin_result_uah = sell_price_uah - buy_price_uah;

            investment_tax_report.investment_ops.trades.push(UaTradeReport {
                buy_date: trade.buy_date,
                sell_date: trade.sell_date,
                name: trade.name.clone(),
                buy_price_uah,
                sell_price_uah,
                fin_result_uah,
            });

            investment_tax_report.investment_ops.total_fin_result += fin_result_uah;
            
            // tax calculation
            let trade_date = trade.sell_date;
            let tax_spec = self.tax_policy_by_date(trade_date)?;
            let income_tax = fin_result_uah * tax_spec.personal_income_tax;
            let military_tax = fin_result_uah * tax_spec.military_tax;
            investment_tax_report.investment_ops.total_tax =
                investment_tax_report.investment_ops.total_tax + UaTax::new(income_tax, military_tax);
        }
        investment_tax_report.investment_ops.total_tax = investment_tax_report.investment_ops.total_tax.round_dp(2);

        for dividend in broker_report.dividends {
            

            let amount = dividend.amount;
            let currency = dividend.currency;
            let amount_uah = self.convert_to_uah_from_preserved_by_currency(amount, currency, dividend.date, &preserved_rated_by_currency)?.round_dp(2);
            investment_tax_report.dividend_ops.income_total += amount_uah;

            // tax calculation
            let tax_spec = self.tax_policy_by_date(dividend.date)?;
            let income_tax = amount_uah * tax_spec.personal_income_tax;
            let military_tax = amount_uah * tax_spec.military_tax;
            investment_tax_report.dividend_ops.total_tax = investment_tax_report.dividend_ops.total_tax + UaTax::new(income_tax, military_tax);
        }
        investment_tax_report.dividend_ops.total_tax = investment_tax_report.dividend_ops.total_tax.round_dp(2);

        investment_tax_report.dividend_ops.income_total = investment_tax_report.dividend_ops.income_total.round_dp(2);

        Ok(investment_tax_report)
    }

    fn get_total_range_by_report(&self, broker_report: &BrokerReport) -> Result<(NaiveDate, NaiveDate), UaTaxReportGeneratorError> {
        let min_date_trades = broker_report.trades.iter().map(|trade| { trade.buy_date }).min();
        let min_date_dividends = broker_report.dividends.iter().map(|dividend| { dividend.date }).min();

        let max_date_trades = broker_report.trades.iter().map(|trade| { trade.sell_date }).max();
        let max_date_dividends = broker_report.dividends.iter().map(|dividend| { dividend.date }).max();

        let earliest_date = min_date_trades
            .into_iter()
            .chain(min_date_dividends)
            .min()
            .ok_or(UaTaxReportGeneratorError::BrokerReportDatesRandeError(format!("Could not get earliest date")))?;

        let latest_date = max_date_trades
            .into_iter()
            .chain(max_date_dividends)
            .max()
            .ok_or(UaTaxReportGeneratorError::BrokerReportDatesRandeError(format!("Could not get latest date")))?;

        Ok((earliest_date, latest_date))
    }

    fn preserve_convert_rate_by_currencies(&self, currencies: HashSet<Currency>, start_date: NaiveDate, end_date: NaiveDate) -> Result<HashMap<Currency, Vec<CurrencyRate>>, UaTaxReportGeneratorError> {
        let mut preserved_rated_by_currency: HashMap<Currency, Vec<CurrencyRate>> = HashMap::new();
        for currency in currencies {
            let preserved_rates = self.preserve_convert_rate(currency, start_date, end_date).unwrap();
            preserved_rated_by_currency.insert(currency, preserved_rates);
        }
        Ok(preserved_rated_by_currency)
    }

    fn preserve_convert_rate(&self, currency: Currency, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<CurrencyRate>, UaTaxReportGeneratorError> {
        let convert_rates = self.national_bank_rate_provider.convert_range(currency, Currency::UAH, start_date, end_date);
        match convert_rates {
            Ok(rates) => {
                Ok(rates)
            },
            Err(e) => {
                println!("Error preserving currency rates: {:?}", e);
                Err(UaTaxReportGeneratorError::CurrencyConvertorError("Error preserving currency rates".to_string()))
            }
        }
    }

    fn convert_to_uah_from_preserved_by_currency(&self, amount: Decimal, currency: Currency, date: NaiveDate, preserved_rates: &HashMap<Currency, Vec<CurrencyRate>>) -> Result<Decimal, UaTaxReportGeneratorError> {
        let rates = preserved_rates.get(&currency);
        match rates {
            Some(rates) => {
                let rate = rates.iter().find(|rate| rate.date == date);
                match rate {
                    Some(rate) => {
                        Ok(amount * rate.rate)
                    },
                    None => {
                        Err(UaTaxReportGeneratorError::CurrencyConvertorError(format!("Could not find rate for currency {} at date {}", currency, date)))
                    }
                }
            },
            None => {
                Err(UaTaxReportGeneratorError::CurrencyConvertorError("Could not get preserved rates".to_string()))
            }
        }
    }

    fn tax_policy_by_date(&self, date: NaiveDate) -> Result<TaxSpecByDate, UaTaxReportGeneratorError> {
        // date must be bigger than the earliest tax policy date
        
        let mut earliest_compatible_policy : Option<TaxSpecByDate> = None;
        for policy in self.tax_policy.sub_policies.iter() {
            if earliest_compatible_policy.is_none() {
                if date >= policy.start_date {
                    earliest_compatible_policy = Some(policy.clone());
                } else {
                    continue;
                }
            }
            if policy.start_date < earliest_compatible_policy.clone().unwrap().start_date && date >= policy.start_date {
                earliest_compatible_policy = Some(policy.clone());
            }
        }

        earliest_compatible_policy.ok_or(UaTaxReportGeneratorError::TaxPolicyError)
    }

}
