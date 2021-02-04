use actix_web::client::Client;
use actix_rt;
use url;

pub async fn make_request(url: &str) -> String {
    let mut client = Client::default();

    // Create request builder and send request

    let response = client.get(url)
       .header("User-Agent", "actix-web/3.0")
       .send()     // <- Send request
       .await;     // <- Wait for response

    String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap()
}

#[actix_rt::test]
pub async fn make_request_test() {
    let IEX_TOKEN = std::env::var("IEX_TOKEN").unwrap();
    let mut url = url::Url::parse("https://sandbox.iexapis.com/stable/stock/twtr/intraday-prices").unwrap();
    url.query_pairs_mut().append_pair("token", &IEX_TOKEN);
    let body = make_request(&url.to_string()).await;
    println!("body is: {}", body);
}