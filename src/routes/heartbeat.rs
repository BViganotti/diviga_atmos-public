use actix_web::{get, HttpResponse};

#[get("/pulse")]
pub async fn pulse() -> HttpResponse {
    HttpResponse::Ok().finish()
}
