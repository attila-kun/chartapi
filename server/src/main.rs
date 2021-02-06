use actix_files::NamedFile;
use actix_web::{App, HttpResponse, HttpServer, Result, get, web};
use chart;
use dto::HLOC;
use serde::{Deserialize};

async fn make_chart(symbol: &str, points: Vec<HLOC>, width: Option<u32>, height: Option<u32>) -> HttpResponse {
    let (width, height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, width),
        (None, Some(height)) => (height, height),
        (None, None) => (450, 450)
    };

    let chart = chart::create_png_chart(symbol, points, width, height).unwrap();
    HttpResponse::Ok()
        .content_type("image/png")
        .body(chart)
}

async fn request_intraday_chart(symbol: &str, width: Option<u32>, height: Option<u32>) -> HttpResponse {
    let points = iex::request_intraday_prices(symbol).await;
    make_chart(symbol, points, width, height).await
}

async fn request_historical_chart(symbol: &str, interval: &str, width: Option<u32>, height: Option<u32>) -> HttpResponse {
    let points = iex::request_historical_prices(symbol, interval).await;
    make_chart(symbol, points, width, height).await
}

#[derive(Deserialize)]
struct Info {
    width: Option<u32>,
    height: Option<u32>
}

#[get("/chart/{symbol}")]
async fn chart_symbol(info: web::Path<(String)>, query_info: web::Query<Info>) -> HttpResponse {
    request_historical_chart(&info.0, "1m", query_info.width, query_info.height).await
}

#[get("/chart/{symbol}/intraday")]
async fn chart_symbol_intraday(info: web::Path<(String)>, query_info: web::Query<Info>) -> HttpResponse {
    request_intraday_chart(&info.0, query_info.width, query_info.height).await
}

#[get("/chart/{symbol}/{interval}")]
async fn chart_symbol_interval(info: web::Path<(String, String)>, query_info: web::Query<Info>) -> HttpResponse {
    let info = info.into_inner();
    request_historical_chart(&info.0, &info.1, query_info.width, query_info.height).await
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(chart_symbol)
            .service(chart_symbol_intraday)
            .service(chart_symbol_interval)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}