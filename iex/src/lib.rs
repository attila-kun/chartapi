use actix_web::client::Client;
use actix_rt;
use lazy_static::{lazy_static};
use url;
use serde::{Deserialize, Serialize};
use serde_json::Result;

lazy_static! {
    static ref IEX_TOKEN: String = std::env::var("IEX_TOKEN").unwrap();
}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct HLOC {
    high: f32,
    low: f32,
    open: f32,
    close: f32
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

async fn request_intraday_prices(symbol: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config
    make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/intraday-prices", symbol)).await
}

async fn request_historical_prices(symbol: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config
    make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/chart/1m", symbol)).await
}

#[actix_rt::test]
pub async fn make_request_test() {
    let body = request_historical_prices("tsla").await;
    println!("body is: {:?}", body);
}