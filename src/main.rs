// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use zero2prod::run;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}", &name)
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run("127.0.0.1")?.await
}
