use actix_web::{
    get, web, middleware,
    http::{StatusCode},
    App, HttpServer, Responder, HttpResponse, Result
};
use actix_files::{Files};

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(Files::new("/static", "./html/static").show_files_listing())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
