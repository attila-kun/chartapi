use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use chart;

async fn request_historical_prices(symbol: &str, interval: &str) -> Result<NamedFile> {
    println!("interval: {}", interval);
    let points = iex::request_historical_prices(symbol, interval).await;
    chart::create_chart(symbol, points).unwrap();
    println!("Handling chart request");
    Ok(NamedFile::open("target/stock.png")?)
}

#[get("/chart/{symbol}")]
async fn chart_root(info: web::Path<(String)>) -> Result<NamedFile> {
    request_historical_prices(&info.0, "1m").await
}

#[get("/chart/{symbol}/{interval}")]
async fn chart_interval(info: web::Path<(String, String)>) -> Result<NamedFile> {
    let info = info.into_inner();
    request_historical_prices(&info.0, &info.1).await
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(chart_root)
            .service(chart_interval)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}