use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub num_market_pairs: i64,
    pub market_pairs: Vec<MarketPair>,
    pub sort: String,
    pub direction: String,
    pub sponsored_exchange: Vec<SponsoredExchange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPair {
    pub rank: i64,
    pub exchange_id: i64,
    pub exchange_name: String,
    pub exchange_slug: String,
    pub exchange_notice: String,
    pub outlier_detected: i64,
    pub price_excluded: i64,
    pub volume_excluded: i64,
    pub market_id: i64,
    pub market_pair: String,
    pub category: String,
    pub market_url: String,
    pub market_score: String,
    pub market_reputation: f64,
    pub base_symbol: String,
    pub base_currency_id: i64,
    pub quote_symbol: String,
    pub quote_currency_id: i64,
    pub price: f64,
    pub volume_usd: f64,
    pub effective_liquidity: f64,
    pub last_updated: String,
    pub quote: f64,
    pub volume_base: f64,
    pub volume_quote: f64,
    pub fee_type: String,
    pub depth_usd_negative_two: f64,
    pub depth_usd_positive_two: f64,
    pub reserves_available: i64,
    pub por_audit_status: i64,
    pub volume_percent: f64,
    pub index_price: i64,
    pub is_verified: i64,
    pub quotes: Vec<Quote>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub center_type: String,
    pub hide_stars_market: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub id: String,
    pub price: f64,
    pub volume24h: f64,
    pub depth_positive_two: f64,
    pub depth_negative_two: f64,
    pub index_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredExchange {
    pub event_id: i64,
    pub custom_name: Option<String>,
    pub custom_logo: Option<String>,
    pub sub_infos: Vec<SubInfo>,
    pub custom_options: String,
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubInfo {
    pub url: String,
    pub show_msg: String,
    pub support_countries: Vec<String>,
    pub exclude_countries: Vec<String>,
    pub custom_options: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub timestamp: String,
    #[serde(rename = "error_code")]
    pub error_code: String,
    #[serde(rename = "error_message")]
    pub error_message: String,
    pub elapsed: String,
    #[serde(rename = "credit_count")]
    pub credit_count: i64,
}
