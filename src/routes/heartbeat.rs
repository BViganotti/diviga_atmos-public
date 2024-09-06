use actix_web::HttpResponse;

pub async fn pulse() -> HttpResponse {
    HttpResponse::Ok().finish()
}
