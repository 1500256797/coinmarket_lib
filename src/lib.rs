pub mod market_pair_response;
pub mod errors;
use market_pair_response::Root;
use reqwest_impersonate as reqwest;
use reqwest_impersonate::impersonate::Impersonate;
use errors::CoinMarketError;
pub async fn fetch_coin_market(coin_name: &str) -> Result<Root, CoinMarketError> {
    let url = format!("https://api.coinmarketcap.com/data-api/v3/cryptocurrency/market-pairs/latest?slug={}&start=1&limit=10&category=spot&centerType=all&direction=desc&spotUntracked=true", coin_name);

    let client_builder = reqwest::Client::builder()
        .impersonate(Impersonate::Chrome123)
        .enable_ech_grease()
        .permute_extensions()
        .cookie_store(true);

    #[cfg(target_os = "windows")]
    let client_builder = client_builder.danger_accept_invalid_certs(true);


    let client = client_builder.build().map_err(CoinMarketError::Reqwest)?;

    let response = client
        .request(reqwest::Method::GET, url)
        .send()
        .await
        .map_err(CoinMarketError::Reqwest)?;

    let body = response.text().await.map_err(CoinMarketError::Reqwest)?;

    let price_data:Root = serde_json::from_str(&body).map_err(CoinMarketError::JsonParse)?;

    Ok(price_data)
}



#[cfg(test)]
mod tests {
    use super::*;

    // https://coinmarketcap.com/currencies/bitcoin/
    #[tokio::test]
    async fn test_fetch_coin_market_btc_name() -> Result<(), Box<dyn std::error::Error>> {
        let coin_name = "bitcoin";
        let result = fetch_coin_market(coin_name).await;
        let price_data = result.unwrap();
        println!("{:#?}", price_data);
        Ok(())
    }

    // https://coinmarketcap.com/currencies/pepe/
    #[tokio::test]
    async fn test_fetch_coin_market_valid_name() -> Result<(), Box<dyn std::error::Error>> {
        let coin_name = "pepe";
        let result = fetch_coin_market(coin_name).await;
        let price_data = result.unwrap();
        println!("{:#?}", price_data);
        Ok(())
    }

    // https://coinmarketcap.com/currencies/toncoin/
    #[tokio::test]
    async fn test_fetch_coin_market_valid_name2() -> Result<(), Box<dyn std::error::Error>> {
        let coin_name = "toncoin";
        let result = fetch_coin_market(coin_name).await;
        let price_data = result.unwrap();
        println!("{:#?}", price_data);
        Ok(())
    }
}
