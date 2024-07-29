use thiserror::Error;
use reqwest_impersonate::Error as ReqwestError;
#[derive(Debug, Error)]
pub enum CoinMarketError {
    #[error("Reqwest error: Cant get price data , error detail is {0}")]
    Reqwest(#[from] ReqwestError),
    
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
}