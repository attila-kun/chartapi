use std::sync::{Arc, Mutex};

use actix_web::client::Client;
use actix_rt;
use dto::{HLOC};
use lazy_static::{lazy_static};
use lru::{LruCache};
use url;

type Cache = LruCache<(String, String), Vec<HLOC>>;

lazy_static! {
    static ref IEX_TOKEN: String = std::env::var("IEX_TOKEN").unwrap();
    static ref HISTORICAL_CACHE: Arc<Mutex<Cache>> = {
        Arc::new(Mutex::new(LruCache::new(300)))
    };
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

    let mut cache = HISTORICAL_CACHE.lock().unwrap();
    let key = (symbol.to_string(), interval.to_string());
    let option = cache.get(&key);

    match option {
        Some(val) => val.to_vec(),
        None => {
            let prices = make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/chart/{}", symbol, interval)).await;
            cache.put(key, prices.to_vec());
            prices
        }
    }
}

#[actix_rt::test]
async fn test_historical_prices() {
    let start = std::time::Instant::now();
    let body = request_historical_prices("aapl", "1m").await;
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    assert!(body.len() > 0);
}

#[actix_rt::test]
async fn test_intraday_prices() {
    let body = request_intraday_prices("aapl").await;
    assert!(body.len() > 0);
}