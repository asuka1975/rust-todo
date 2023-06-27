use actix_web::{web, Error, HttpResponse};

pub async fn get_tasks() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_task() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}