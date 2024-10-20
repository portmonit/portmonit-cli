use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::tax::CurrencyConvertor::Currency;

pub struct BrokerReport {
    pub trades: Vec<BrokerTrade>,
    pub dividends: Vec<BrokerDividend>,
}

pub struct BrokerTrade {
    pub buy_date: NaiveDate,
    pub sell_date: NaiveDate,
    pub name: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub currency: Currency,
}

pub struct BrokerDividend {
    pub date: NaiveDate,
    pub name: String,
    pub amount: Decimal,
    pub currency: Currency,
}

pub trait BrokerReportProvider {
    fn get_broker_report(&self) -> BrokerReport;
}

pub type BrokerReportProviderBox = Box<dyn BrokerReportProvider>;

