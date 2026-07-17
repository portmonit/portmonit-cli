// Implementation of adapter for IBKR reports

use chrono::NaiveDate;
use quick_xml::de::from_str;
use rust_decimal::Decimal;

use super::broker_adapter::BrokerAdapter;
use crate::broker::common::broker_report_parser_factory::BrokerType;
use crate::broker::ibkr::report;
use crate::tax::currency_convertor::Currency;
use crate::tax::ua::broker_report_provider::*;

pub struct IbkrAdapter {
    adapter: BrokerAdapter,
}

impl IbkrAdapter {
    pub fn new(input_file: String) -> IbkrAdapter {
        IbkrAdapter {
            adapter: BrokerAdapter::new(BrokerType::IBKR, input_file),
        }
    }

    // Alternative constructor with auto-detection
    pub fn from_file(input_file: String) -> Result<IbkrAdapter, String> {
        match BrokerAdapter::from_file(input_file) {
            Ok(adapter) => {
                if *adapter.get_broker_type() != BrokerType::IBKR {
                    return Err(format!(
                        "Expected IBKR broker type, but detected {:?}",
                        adapter.get_broker_type()
                    ));
                }
                Ok(IbkrAdapter { adapter })
            }
            Err(e) => Err(e),
        }
    }

    // Convert from generic XML to IBKR specific structure
    fn parse_ibkr_xml(&self, xml_content: &str) -> Result<report::FlexQueryResponse, String> {
        match from_str(xml_content) {
            Ok(report) => Ok(report),
            Err(e) => Err(format!("Failed to parse IBKR report: {}", e)),
        }
    }
}

impl BrokerReportProvider for IbkrAdapter {
    fn get_broker_report(&self) -> BrokerReport {
        // Get raw report using the abstract adapter
        let raw_report = match self.adapter.parse_report() {
            Ok(report) => report,
            Err(e) => panic!("Error parsing report: {}", e), // Keep the panic for compatibility
        };

        // Convert raw report to IBKR specific format
        let ibkr_report = match self.parse_ibkr_xml(&raw_report.content) {
            Ok(report) => report,
            Err(e) => panic!("Error converting to IBKR format: {}", e),
        };

        // Process IBKR report data
        let mut broker_report = BrokerReport {
            trades: Vec::new(),
            dividends: Vec::new(),
        };

        for statement in ibkr_report.flex_statements.flex_statement {
            // Process trades
            for trade in statement.trades.lots {
                let buy_date = NaiveDate::parse_from_str(
                    trade.open_date_time.split(';').nth(0).unwrap(),
                    "%Y%m%d",
                )
                .unwrap();
                let sell_date = NaiveDate::parse_from_str(&trade.trade_date, "%Y%m%d").unwrap();
                let cost = Decimal::from_str_exact(&trade.cost).unwrap();
                let fifo_pnl_realized = Decimal::from_str_exact(&trade.fifo_pnl_realized)
                    .unwrap()
                    .round_dp(2);

                let buy_price = cost.round_dp(2);
                let sell_price = (cost + fifo_pnl_realized).round_dp(2);

                // name consist of assert category and description
                // assert category is 'Акції' for 'STK' and 'ETF' for 'ETF'
                let active_type: Option<String> = if trade.asset_category == "STK" {
                    Some("Акції".to_string())
                } else if trade.asset_category == "ETF" {
                    Some("ETF".to_string())
                } else {
                    None
                };

                let name: String;
                match active_type {
                    Some(active_type) => {
                        name = format!("{} {}", active_type, trade.description);
                    }
                    None => {
                        name = trade.description;
                    }
                }

                let currency = Currency::from_str(&trade.currency).unwrap();

                broker_report.trades.push(BrokerTrade {
                    buy_date,
                    sell_date,
                    name,
                    buy_price,
                    sell_price,
                    currency,
                });
            }

            // Process dividends
            for dividend in statement.cash_transactions.cash_transactions {
                let date = NaiveDate::parse_from_str(&dividend.settle_date, "%Y%m%d").unwrap();
                let amount = Decimal::from_str_exact(&dividend.amount)
                    .unwrap()
                    .round_dp(2);
                if amount.is_sign_negative() {
                    // negative amount means tax
                    continue;
                }

                let currency = Currency::from_str(&dividend.currency).unwrap();

                // it is not used in UA tax report, so format is not important
                let name = dividend.description;

                broker_report.dividends.push(BrokerDividend {
                    date,
                    name,
                    amount,
                    currency,
                });
            }
        }

        broker_report
    }
}
