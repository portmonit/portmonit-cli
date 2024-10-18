use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;
use quick_xml::de::from_str;
use ureq;

use crate::tax::CurrencyConvertor::*;

#[derive(Debug, Deserialize, PartialEq, Default)]
#[serde(default)]
struct XmlExchange {
    #[serde(rename = "currency")]
    currencies: Vec<XmlCurrency>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
struct XmlCurrency {
    exchangedate: String,
    r030: String,
    cc: String,
    txt: String,
    enname: String,
    rate: String,
    units: String,
    rate_per_unit: String,
    group: String,
    calcdate: String,
}

fn fetch_nbu_exchange_rate(currency: Currency, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<XmlExchange, CurrencyConvertorError> {
    let url = format!("https://bank.gov.ua/NBU_Exchange/exchange_site?start={}&end={}&valcode={}&sort=exchangedate&order=desc",
        start_date.format("%Y%m%d"), end_date.format("%Y%m%d"), currency.to_string().to_ascii_lowercase());
    
    let response = ureq::get(url.as_str()).call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("Body: {}", body);

            let exchange: XmlExchange = from_str(body.as_str()).unwrap();
            return Ok(exchange);
        },
        Err(e) => {
            return Err(CurrencyConvertorError::CurrencyNotSupported{details: format!("Error fetching exchange rate: {:?}", e)})
        }
    }
}

pub struct NationalBank {}

impl CurrencyRateProvider for NationalBank {
    fn convert(&self, from: Currency, to: Currency, date: chrono::NaiveDate) -> Result<CurrencyRate, CurrencyConvertorError> {
        const EXPECTED_ELEMENTS: usize = 1;
        let res = self.convert_range(from, to, date, date);
        match res {
            Ok(mut rates) => {
                if rates.len() == EXPECTED_ELEMENTS {
                    return Ok(rates.remove(0));
                } else {
                    return Err(CurrencyConvertorError::CurrencyNotSupported{
                        details: format!("Error converting currency: from {:?} to {:?} on date {:?}, len = {:?}", from, to, date, rates.len())
                    });
                }
            },
            Err(e) => {
                return Err(e);
            }
        }        
    }

    fn convert_range(&self, from: Currency, to: Currency, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<Vec<CurrencyRate>, CurrencyConvertorError> {
        // Ukraine National Bank supports only UAH as base currency
        if to != Currency::UAH {
            return Err(CurrencyConvertorError::CurrencyNotSupported{details: format!("Currency not supported: {:?}", from)});
        }

        if from == Currency::UAH {
            return Err(CurrencyConvertorError::CurrencyNotSupported{details: format!("'From' currency can't be: {:?}", from)});
        }

        let exchange = fetch_nbu_exchange_rate(from, start_date, end_date);
        match exchange {
            Ok(exchange) => {
                let mut rates = Vec::new();
                for currency in exchange.currencies.iter() {
                    let decimal_rate = Decimal::from_str(currency.rate.as_str()).unwrap();
                    let rate = CurrencyRate {
                        from: from,
                        to: to,
                        rate: decimal_rate,
                        date: chrono::NaiveDate::parse_from_str(currency.exchangedate.as_str(), "%d.%m.%Y").unwrap(),
                    };
                    rates.push(rate);
                }
                return Ok(rates);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_xml_exchange_deserialization() {
        let xml = r#"
            <exchange>
                <currency>
                    <exchangedate>01.01.2022</exchangedate>
                    <r030>840</r030>
                    <cc>USD</cc>
                    <txt>Долар США</txt>
                    <enname>US Dollar</enname>
                    <rate>27.2782</rate>
                    <units>1</units>
                    <rate_per_unit>27.2782</rate_per_unit>
                    <group>1</group>
                    <calcdate>30.12.2021</calcdate>
                </currency>
            </exchange>
        "#;

        let result: XmlExchange = from_str(xml).unwrap();

        assert_eq!(result.currencies.len(), 1);
        assert_eq!(result.currencies[0].exchangedate, "01.01.2022");
        assert_eq!(result.currencies[0].r030, "840");
        assert_eq!(result.currencies[0].cc, "USD");
        assert_eq!(result.currencies[0].txt, "Долар США");
        assert_eq!(result.currencies[0].enname, "US Dollar");
        assert_eq!(result.currencies[0].rate, "27.2782");
        assert_eq!(result.currencies[0].units, "1");
        assert_eq!(result.currencies[0].rate_per_unit, "27.2782");
        assert_eq!(result.currencies[0].group, "1");
        assert_eq!(result.currencies[0].calcdate, "30.12.2021");
    }

    #[test]
    fn test_xml_exchange_deserialization_multiple_dates() {
        let xml = r#"
            <exchange>
                <currency>
                    <exchangedate>01.01.2022</exchangedate>
                    <r030>840</r030>
                    <cc>USD</cc>
                    <txt>Долар США</txt>
                    <enname>US Dollar</enname>
                    <rate>27.2782</rate>
                    <units>1</units>
                    <rate_per_unit>27.2782</rate_per_unit>
                    <group>1</group>
                    <calcdate>30.12.2021</calcdate>
                </currency>
                <currency>
                    <exchangedate>01.01.2022</exchangedate>
                    <r030>840</r030>
                    <cc>USD</cc>
                    <txt>Долар США</txt>
                    <enname>US Dollar</enname>
                    <rate>27.2782</rate>
                    <units>1</units>
                    <rate_per_unit>27.2782</rate_per_unit>
                    <group>1</group>
                    <calcdate>30.12.2021</calcdate>
                </currency>
            </exchange>
        "#;

        let result: XmlExchange = from_str(xml).unwrap();

        assert_eq!(result.currencies.len(), 2);
        for currency in result.currencies.iter() {
            assert_eq!(currency.exchangedate, "01.01.2022");
            assert_eq!(currency.r030, "840");
            assert_eq!(currency.cc, "USD");
            assert_eq!(currency.txt, "Долар США");
            assert_eq!(currency.enname, "US Dollar");
            assert_eq!(currency.rate, "27.2782");
            assert_eq!(currency.units, "1");
            assert_eq!(currency.rate_per_unit, "27.2782");
            assert_eq!(currency.group, "1");
            assert_eq!(currency.calcdate, "30.12.2021");
        }
    }

    // this test requires internet connection and works only with real data
    #[test]
    fn test_fetch_nbu_exchange_rate() {
        let result = fetch_nbu_exchange_rate(Currency::USD, chrono::NaiveDate::from_ymd(2022, 1, 2), chrono::NaiveDate::from_ymd(2022, 1, 2));
        assert!(result.is_ok());
        let exchange = result.unwrap();
        assert_eq!(exchange.currencies.len(), 1);
        let currency = &exchange.currencies[0];
        assert_eq!(currency.exchangedate, "02.01.2022");
    }

    #[test]
    fn test_fetch_nbu_exchange_rate_one_el() {
        let raw_result = fetch_nbu_exchange_rate(Currency::EUR, chrono::NaiveDate::from_ymd(2022, 1, 2), chrono::NaiveDate::from_ymd(2022, 1, 2));

        let nbu = NationalBank{};
        let trait_result = nbu.convert(Currency::EUR, Currency::UAH, chrono::NaiveDate::from_ymd(2022, 1, 2));

        assert!(raw_result.is_ok());
        assert!(trait_result.is_ok());

        let raw_exchange = raw_result.unwrap();
        let trait_exchange = trait_result.unwrap();

        assert_eq!(raw_exchange.currencies.len(), 1);
        let currency = &raw_exchange.currencies[0];
        assert_eq!(currency.exchangedate, "02.01.2022");

        assert_eq!(trait_exchange.from, Currency::EUR);
        assert_eq!(trait_exchange.to, Currency::UAH);
        assert_eq!(trait_exchange.date, chrono::NaiveDate::from_ymd(2022, 1, 2));

        let rate = Decimal::from_str(currency.rate.as_str()).unwrap();
        assert_eq!(trait_exchange.rate, rate);

        let raw_rate = CurrencyRate {
            from: Currency::EUR,
            to: Currency::UAH,
            rate: Decimal::from_str(currency.rate.as_str()).unwrap(),
            date: chrono::NaiveDate::parse_from_str(currency.exchangedate.as_str(), "%d.%m.%Y").unwrap(),
        };

        assert_eq!(trait_exchange, raw_rate);
    }

    #[test]
    fn test_nbu_multiple_exchange_rates() {
        let nbu = NationalBank{};
        let result = nbu.convert_range(Currency::USD, Currency::UAH, chrono::NaiveDate::from_ymd(2022, 1, 1), chrono::NaiveDate::from_ymd(2022, 1, 2));
        assert!(result.is_ok());
        let exchange = result.unwrap();
        assert_eq!(exchange.len(), 2);
        assert_eq!(exchange[0].date, chrono::NaiveDate::from_ymd(2022, 1, 2));
        assert_eq!(exchange[1].date, chrono::NaiveDate::from_ymd(2022, 1, 1));
    }
}
