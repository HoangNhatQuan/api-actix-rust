use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number<T> {
    a: T,
    b: T,
}

async fn index<T: std::ops::Add<Output = T> + Copy + std::fmt::Display>(
    number: web::Json<Number<T>>,
) -> impl Responder {
    let sum = number.a + number.b;
    HttpResponse::Ok().body(format!("Sum: {}", sum))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/calSum", web::post().to(index::<i32>)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
