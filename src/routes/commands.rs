use actix_web::{HttpResponse, post, web};

use crate::cec;

#[post("/api/power_off")]
pub async fn power_off() -> HttpResponse {
    match web::block(|| cec::commands::power_off()).await {
        Ok(Ok(())) => HttpResponse::NoContent().finish(),

        Ok(Err(e)) => {
            log::error!("CEC power_off failed: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },

        Err(e) => {
            log::error!("Actix blocking error: {}", e);
            HttpResponse::InternalServerError().body("internal worker error")
        }
    }
}
