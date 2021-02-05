use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use chart;

#[get("/chart/{symbol}")]
async fn chart_service(info: web::Path<(String)>) -> Result<NamedFile> {
    let symbol = &info.0;
    let points = iex::request_historical_prices(symbol).await;
    chart::create_chart(symbol, points).unwrap();
    println!("Handling chart request");
    Ok(NamedFile::open("target/stock.png")?)
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(chart_service)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}