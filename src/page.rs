use actix_web::{
    http::{StatusCode},
    Responder, Result
};
use actix_files::{NamedFile};

pub async fn index() -> Result<impl Responder> {
    let file = NamedFile::open("./html/index.html")?
        .customize()
        .with_status(StatusCode::OK);
    Ok(file)
}