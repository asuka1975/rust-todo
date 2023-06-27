use actix_web::web;

use crate::task::{get_tasks, add_task};
use actix_web::{
    http::{StatusCode},
    Responder, Result
};
use actix_files::{NamedFile};

async fn index() -> Result<impl Responder> {
    let file = NamedFile::open("./html/index.html")?
        .customize()
        .with_status(StatusCode::OK);
    Ok(file)
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/task")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_tasks))
                                    .route(web::post().to(add_task))
                            )
                    )
            )
    ).service(
        web::scope("")
            .service(
                web::resource("")
                    .route(web::get().to(index))
            )
    );
}