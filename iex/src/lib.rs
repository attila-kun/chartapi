use actix_web::client::Client;
use actix_rt;
use async_std::sync::Mutex;
use dto::{HLOC};
use lazy_static::{lazy_static};
use lru::{LruCache};
use std::{hash::Hash, sync::{Arc}};
use url;

type PriceCache<K> = Arc<Mutex<LruCache<K, Vec<HLOC>>>>;
type HistoricalCache = PriceCache<(String, String)>;
type IntradayCache = PriceCache<(String)>;

static CACHE_SIZE: usize = 300;

lazy_static! {
    static ref IEX_TOKEN: String = std::env::var("IEX_TOKEN").unwrap();
    static ref HISTORICAL_CACHE: HistoricalCache = {
        Arc::new(Mutex::new(LruCache::new(CACHE_SIZE)))
    };
    static ref INTRADAY_CACHE: IntradayCache = {
        Arc::new(Mutex::new(LruCache::new(CACHE_SIZE)))
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

async fn request_prices<K>(
    url: &str,
    cache: &PriceCache<K>,
    key: K
) -> Vec<HLOC> where K : Hash + Eq {

    let mut cache = cache.lock().await;
    let option = cache.get(&key);

    match option {
        Some(val) => val.to_vec(),
        None => {
            let prices = make_request(url).await;
            cache.put(key, prices.to_vec());
            prices
        }
    }
}

pub async fn request_intraday_prices(symbol: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config

    request_prices(
        &format!("https://sandbox.iexapis.com/stable/stock/{}/intraday-prices", symbol),
        &INTRADAY_CACHE,
        symbol.to_string()
    ).await
}

pub async fn request_historical_prices(symbol: &str, interval: &str) -> Vec<HLOC> {
    // TODO: make sandbox part of config

    request_prices(
        &format!("https://sandbox.iexapis.com/stable/stock/{}/chart/{}", symbol, interval),
        &HISTORICAL_CACHE,
        (symbol.to_string(), interval.to_string())
    ).await
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
}