include!(concat!(env!("OUT_DIR"), "/constants.rs"));
use actix_web::{
    get, http::header, post, web::Json, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Program {
    lang: String,
    program: String,
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    println!("{:?} ba om {}", req.head().peer_addr.unwrap(), req.path());
    HttpResponse::Ok()
        .insert_header(header::ContentType::html())
        .body(html())
}

#[get("/style.css")]
async fn style(req: HttpRequest) -> impl Responder {
    println!("{:?} ba om {}", req.head().peer_addr.unwrap(), req.path());
    HttpResponse::Ok()
        .insert_header(("content-type", "text/css"))
        .body(css())
}

#[get("/main.js")]
async fn script(req: HttpRequest) -> impl Responder {
    println!("{:?} ba om {}", req.head().peer_addr.unwrap(), req.path());
    HttpResponse::Ok()
        .insert_header(("content-type", "application/javascript"))
        .body(js())
}

#[post("/compile")]
async fn compile(json: Json<Program>) -> impl Responder {
    println!("Språk: {}\nKildekode:\n{}", json.lang, json.program);
    "OK"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(style)
            .service(script)
            .service(compile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
