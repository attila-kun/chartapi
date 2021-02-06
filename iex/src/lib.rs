use actix_web::client::Client;
use actix_rt;
use dto::{HLOC};
use lazy_static::{lazy_static};
use url;

lazy_static! {
    static ref IEX_TOKEN: String = std::env::var("IEX_TOKEN").unwrap();
}

async fn make_request(url_without_token: &str) -> Vec<HLOC> {

    let mut url = url::Url::parse(url_without_token).unwrap();
    url.query_pairs_mut().append_pair("token", &IEX_TOKEN);

    let mut client = Client::default();
    let response = client.get(url.to_string())
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    let body = String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap();
    serde_json::from_str(&body).unwrap()
}

pub async fn request_intraday_prices(symbol: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config
    make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/intraday-prices", symbol)).await
}

pub async fn request_historical_prices(symbol: &str, interval: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config
    make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/chart/{}", symbol, interval)).await
}

#[actix_rt::test]
async fn test_historical_prices() {
    let body = request_historical_prices("aapl", "1m").await;
    assert!(body.len() > 0);
}

#[actix_rt::test]
async fn test_intraday_prices() {
    let body = request_intraday_prices("aapl").await;
    assert!(body.len() > 0);
    // println!("{:?}", body);
}