use chart;
use iex;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder, Result};

async fn stock() -> Result<NamedFile> {
    chart::create_chart();
    println!("Handling stock request");
    Ok(NamedFile::open("target/stock.png")?)
}

async fn index() -> Result<NamedFile> {

    let hloc_vec = iex::request_historical_prices("aapl").await;
    println!("{:?}", hloc_vec);
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