use serde::Deserialize;


#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexQueryResponse {
    #[serde(rename = "@queryName")]
    query_name: String,
    
    #[serde(rename = "@type")]
    type_str: String,


    #[serde(rename = "FlexStatements")]
    flex_statements: FlexStatements,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexStatements {
    #[serde(rename = "FlexStatement")]
    flex_statement: Vec<FlexStatement>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexStatement {
    #[serde(rename = "@accountId")]
    account_id: String,

    #[serde(rename = "@fromDate")]
    from_date: String,

    #[serde(rename = "@toDate")]
    to_date: String,

    #[serde(rename = "@whenGenerated")]
    when_generated: String,

    #[serde(rename = "Trades")]
    trades: Trades,

    #[serde(rename = "CashTransactions")]
    cash_transactions: CashTransactions,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct Trades {
    #[serde(rename = "Lot")]
    lots: Vec<Lot>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
struct Lot {
    #[serde(rename = "@accountId")]
    account_id: String,

    #[serde(rename = "@currency")]
    currency: String,

    #[serde(rename = "@fxRateToBase")]
    fx_rate_to_base: String,

    #[serde(rename = "@assetCategory")]
    asset_category: String,

    #[serde(rename = "@subCategory")]
    sub_category: String,

    #[serde(rename = "@symbol")]
    symbol: String,

    #[serde(rename = "@description")]
    description: String,

    #[serde(rename = "@securityID")]
    security_id: String,

    #[serde(rename = "@securityIDType")]
    security_id_type: String,

    #[serde(rename = "@listingExchange")]
    listing_exchange: String,

    #[serde(rename = "@reportDate")]
    report_date: String,

    #[serde(rename = "@dateTime")]
    date_time: String,

    #[serde(rename = "@tradeDate")]
    trade_date: String,

    #[serde(rename = "@exchange")]
    exchange: String,

    #[serde(rename = "@quantity")]
    quantity: String,

    #[serde(rename = "@tradePrice")]
    trade_price: String,

    #[serde(rename = "@cost")]
    cost: String,

    #[serde(rename = "@fifoPnlRealized")]
    fifo_pnl_realized: String,

    #[serde(rename = "@buySell")]
    buy_sell: String,

    #[serde(rename = "@transactionID")]
    transaction_id: String,

    #[serde(rename = "@openDateTime")]
    open_date_time: String,

    #[serde(rename = "@levelOfDetail")]
    level_of_detail: String, // wtf is that?
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction")]
    cash_transactions: Vec<CashTransaction>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
struct CashTransaction {
    #[serde(rename = "@accountId")]
    account_id: String,

    #[serde(rename = "@currency")]
    currency: String,

    #[serde(rename = "@assetCategory")]
    asset_category: String,

    #[serde(rename = "@subCategory")]
    sub_category: String,

    #[serde(rename = "@symbol")]
    symbol: String,

    #[serde(rename = "@description")]
    description: String,

    #[serde(rename = "@securityID")]
    security_id: String,

    #[serde(rename = "@securityIDType")]
    security_id_type: String,

    #[serde(rename = "@listingExchange")]
    listing_exchange: String, 

    #[serde(rename = "@multiplier")]
    multiplier: String, // what does it multiplie?

    #[serde(rename = "@dateTime")]
    date_time: String,

    #[serde(rename = "@settleDate")]
    settle_date: String,

    // negative for taxes, positive for dividends
    #[serde(rename = "@amount")]
    amount: String,

    #[serde(rename = "@type")]
    transaction_type: String,

    #[serde(rename = "@transactionID")]
    transaction_id: String,

    #[serde(rename = "@reportDate")]
    report_date: String,

    #[serde(rename = "@levelOfDetail")]
    level_of_detail: String, // wtf is that?
}
