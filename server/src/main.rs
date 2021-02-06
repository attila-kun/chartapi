use actix_files::NamedFile;
use actix_web::{App, HttpResponse, HttpServer, Result, get, web};
use chart;
use serde::{Deserialize};

async fn request_historical_prices(symbol: &str, interval: &str, width: Option<u32>, height: Option<u32>) -> HttpResponse {

    let (width, height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, width),
        (None, Some(height)) => (height, height),
        (None, None) => (450, 450)
    };

    let points = iex::request_historical_prices(symbol, interval).await;
    let chart = chart::create_png_chart(symbol, points, width, height).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(chart)
}

#[derive(Deserialize)]
struct Info {
    width: Option<u32>,
    height: Option<u32>
}

#[get("/chart/{symbol}")]
async fn chart_root(info: web::Path<(String)>, query_info: web::Query<Info>) -> HttpResponse {
    println!("query string: {} ", info.to_string());
    request_historical_prices(&info.0, "1m", query_info.width, query_info.height).await
}

#[get("/chart/{symbol}/{interval}")]
async fn chart_interval(info: web::Path<(String, String)>, query_info: web::Query<Info>) -> HttpResponse {
    let info = info.into_inner();
    request_historical_prices(&info.0, &info.1, query_info.width, query_info.height).await
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