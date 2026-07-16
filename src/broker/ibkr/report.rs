use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexQueryResponse {
    #[serde(rename = "@queryName")]
    pub query_name: String,

    #[serde(rename = "@type")]
    pub type_str: String,

    #[serde(rename = "FlexStatements")]
    pub flex_statements: FlexStatements,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexStatements {
    #[serde(rename = "FlexStatement")]
    pub flex_statement: Vec<FlexStatement>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct FlexStatement {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(rename = "@fromDate")]
    pub from_date: String,

    #[serde(rename = "@toDate")]
    pub to_date: String,

    #[serde(rename = "@whenGenerated")]
    pub when_generated: String,

    #[serde(rename = "Trades")]
    pub trades: Trades,

    #[serde(rename = "CashTransactions")]
    pub cash_transactions: CashTransactions,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct Trades {
    #[serde(rename = "Lot")]
    pub lots: Vec<Lot>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct Lot {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(rename = "@currency")]
    pub currency: String,

    #[serde(rename = "@fxRateToBase")]
    pub fx_rate_to_base: String,

    #[serde(rename = "@assetCategory")]
    pub asset_category: String,

    #[serde(rename = "@subCategory")]
    pub sub_category: String,

    #[serde(rename = "@symbol")]
    pub symbol: String,

    #[serde(rename = "@description")]
    pub description: String,

    #[serde(rename = "@securityID")]
    pub security_id: String,

    #[serde(rename = "@securityIDType")]
    pub security_id_type: String,

    #[serde(rename = "@listingExchange")]
    pub listing_exchange: String,

    #[serde(rename = "@reportDate")]
    pub report_date: String,

    #[serde(rename = "@dateTime")]
    pub date_time: String,

    #[serde(rename = "@tradeDate")]
    pub trade_date: String,

    #[serde(rename = "@exchange")]
    pub exchange: String,

    #[serde(rename = "@quantity")]
    pub quantity: String,

    #[serde(rename = "@tradePrice")]
    pub trade_price: String,

    #[serde(rename = "@cost")]
    pub cost: String,

    #[serde(rename = "@fifoPnlRealized")]
    pub fifo_pnl_realized: String,

    #[serde(rename = "@buySell")]
    pub buy_sell: String,

    #[serde(rename = "@transactionID")]
    pub transaction_id: String,

    #[serde(rename = "@openDateTime")]
    pub open_date_time: String,

    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String, // wtf is that?
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction")]
    pub cash_transactions: Vec<CashTransaction>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(default)]
pub struct CashTransaction {
    #[serde(rename = "@accountId")]
    pub account_id: String,

    #[serde(rename = "@currency")]
    pub currency: String,

    #[serde(rename = "@assetCategory")]
    pub asset_category: String,

    #[serde(rename = "@subCategory")]
    pub sub_category: String,

    #[serde(rename = "@symbol")]
    pub symbol: String,

    #[serde(rename = "@description")]
    pub description: String,

    #[serde(rename = "@securityID")]
    pub security_id: String,

    #[serde(rename = "@securityIDType")]
    pub security_id_type: String,

    #[serde(rename = "@listingExchange")]
    pub listing_exchange: String,

    #[serde(rename = "@multiplier")]
    pub multiplier: String, // what does it multiplie?

    #[serde(rename = "@dateTime")]
    pub date_time: String,

    #[serde(rename = "@settleDate")]
    pub settle_date: String,

    // negative for taxes, positive for dividends
    #[serde(rename = "@amount")]
    pub amount: String,

    #[serde(rename = "@type")]
    pub transaction_type: String,

    #[serde(rename = "@transactionID")]
    pub transaction_id: String,

    #[serde(rename = "@reportDate")]
    pub report_date: String,

    #[serde(rename = "@levelOfDetail")]
    pub level_of_detail: String, // wtf is that?
}
