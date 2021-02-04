use actix_web::client::Client;
use actix_rt;
use lazy_static::{lazy_static};
use url;

lazy_static! {
    static ref IEX_TOKEN: String = std::env::var("IEX_TOKEN").unwrap();
}

async fn make_request(url_base: &str) -> String {

    let mut url = url::Url::parse(url_base).unwrap();
    url.query_pairs_mut().append_pair("token", &IEX_TOKEN);

    let mut client = Client::default();
    let response = client.get(url.to_string())
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap()
}

async fn request_intraday_prices(symbol: &str) -> String {
    // TODO: make sandbox part of config
    make_request(&format!("https://sandbox.iexapis.com/stable/stock/{}/intraday-prices", symbol)).await
}

#[actix_rt::test]
pub async fn make_request_test() {
    let body = request_intraday_prices("tsla").await;
    println!("body is: {}", body);
}