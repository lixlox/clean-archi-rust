use crate::web::routes::WebState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn get_all(container: web::Data<WebState>, _req: HttpRequest) -> HttpResponse {
    match container.query_books.get_all() {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
