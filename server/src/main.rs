use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use chart;

async fn stock() -> Result<NamedFile> {
    let symbol = "tsla";
    let points = iex::request_historical_prices(symbol).await;
    chart::create_chart(symbol, points).unwrap();
    println!("Handling stock request");
    Ok(NamedFile::open("target/stock.png")?)
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/stock", web::get().to(stock))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}