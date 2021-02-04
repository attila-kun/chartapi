use actix_web::client::Client;
use actix_rt;
use url;

async fn make_request(url: &str) -> String {
    let mut client = Client::default();

    let response = client.get(url)
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap()
}

async fn request_intraday_prices(symbol: &str, token: &str) -> String {
    // TODO: make sandbox part of config
    let mut url = url::Url::parse(&format!("https://sandbox.iexapis.com/stable/stock/{}/intraday-prices", symbol)).unwrap();
    url.query_pairs_mut().append_pair("token", token);
    make_request(&url.to_string()).await
}

#[actix_rt::test]
pub async fn make_request_test() {
    let IEX_TOKEN = std::env::var("IEX_TOKEN").unwrap();
    let body = request_intraday_prices("tsla", &IEX_TOKEN).await;
    println!("body is: {}", body);
}