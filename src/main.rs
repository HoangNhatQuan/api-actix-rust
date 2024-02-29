use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number {
    a: i32,
    b: i32,
}

async fn index(number: web::Json<Number>) -> impl Responder {
    let sum = number.a + number.b;
    HttpResponse::Ok().body(format!("Sum: {}", sum))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/calSum", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
