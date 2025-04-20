// TODO: here adapt Ibkr for UA tax report generator

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::broker;
use crate::tax::ua::BrokerReportProvider::*;
use crate::broker::ibkr::ReportParser::*;
use crate::tax::CurrencyConvertor::Currency;

pub struct IbkrAdapter {
    ibkr_report_parser: IbkrReportParser,
}

impl IbkrAdapter {
    pub fn new(input_file: String) -> IbkrAdapter {
        IbkrAdapter {
            ibkr_report_parser: IbkrReportParser::new(input_file),
        }
    }
}

impl BrokerReportProvider for IbkrAdapter {
    fn get_broker_report(&self) -> BrokerReport {
        let xml_parsed = self.ibkr_report_parser.parse();
        match xml_parsed {
            Ok(report) => {
                let mut broker_report = BrokerReport {
                    trades: Vec::new(),
                    dividends: Vec::new(),
                };

                for statement in report.flex_statements.flex_statement {
                    for trade in statement.trades.lots {


                        let buy_date = NaiveDate::parse_from_str(&trade.open_date_time.split(';').nth(0).unwrap(), "%Y%m%d").unwrap();
                        let sell_date = NaiveDate::parse_from_str(&trade.trade_date, "%Y%m%d").unwrap();                        
                        let cost = Decimal::from_str_exact(&trade.cost).unwrap();
                        let fifo_pnl_realized = Decimal::from_str_exact(&trade.fifo_pnl_realized).unwrap();

                        let buy_price = cost.round_dp(2);
                        let sell_price = (cost + fifo_pnl_realized).round_dp(2);

                        // name consist of assert category and description
                        // assert category is 'Акції' for 'STK' and 'ETF' for 'ETF'
                        let active_type : Option<String> = if trade.asset_category == "STK" {
                            Some("Акції".to_string())
                        } else if trade.asset_category == "ETF" {
                            Some("ETF".to_string())
                        } else {
                            None
                        };

                        let name : String;
                        match active_type {
                            Some(active_type) => {
                                name = format!("{} {}", active_type, trade.description);
                            },
                            None => {
                                name = trade.description;
                            }
                        }

                        let currency = Currency::from_str(&trade.currency).unwrap();

                        broker_report.trades.push(
                            BrokerTrade {
                                buy_date,
                                sell_date,
                                name,
                                buy_price,
                                sell_price,
                                currency,
                            }
                        );
                    }

                    for dividend in statement.cash_transactions.cash_transactions {
                        let date = NaiveDate::parse_from_str(&dividend.settle_date, "%Y%m%d").unwrap();
                        let amount = Decimal::from_str_exact(&dividend.amount).unwrap().round_dp(2);
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

                return broker_report;
            },
            Err(e) => {
                panic!("Error parsing report: {:?}", e);
            }
            
        }
    }
}

