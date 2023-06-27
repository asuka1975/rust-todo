use actix_web::web;

use crate::handlers::task;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/task")
                            .service(
                                web::resource("")
                                    .route(web::get().to(task::get_tasks))
                                    .route(web::post().to(task::add_task))
                            )
                    )
            )
    );
}