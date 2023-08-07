use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("version", "v0.0.1"))
        .finish()
}
