use actix_web::{
    get, web, middleware,
    http::{StatusCode},
    App, HttpServer, Responder, HttpResponse, Result, Either
};
use actix_files::{Files, NamedFile};

use rust_todo::router;

#[get("/")]
async fn index() -> Result<impl Responder> {
    let file = NamedFile::open("./html/index.html")?
        .customize()
        .with_status(StatusCode::OK);
    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(router::router)
            .wrap(middleware::Compress::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
