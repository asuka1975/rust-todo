use actix_web::web;

use crate::task::{get_tasks, add_task};
use crate::page::index;
use actix_files::{Files};

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
    )
    .service(
        Files::new("/static", "./html/static").show_files_listing()
    )
    ;
}