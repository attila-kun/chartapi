use actix_web::client::Client;
use actix_rt;

pub async fn make_request() -> String {
    let mut client = Client::default();

    // Create request builder and send request
    let response = client.get("http://www.rust-lang.org")
       .header("User-Agent", "actix-web/3.0")
       .send()     // <- Send request
       .await;     // <- Wait for response

    String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap()
}

#[actix_rt::test]
pub async fn make_request_test() {
    let body = make_request().await;
    println!("{}", body);
}