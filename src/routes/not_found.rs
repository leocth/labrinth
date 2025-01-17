use crate::models::error::ApiError;
use actix_web::{HttpResponse, Responder};

#[allow(clippy::unused_async)]
pub async fn not_found() -> impl Responder {
    let data = ApiError {
        error: "not_found",
        description: "the requested route does not exist",
    };

    HttpResponse::NotFound().json(data)
}
