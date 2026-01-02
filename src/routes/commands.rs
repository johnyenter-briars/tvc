use actix_web::{HttpResponse, post, web};

use crate::OSD_NAME;

async fn run_cec<F>(f: F) -> HttpResponse
where
    F: FnOnce() -> std::io::Result<()> + Send + 'static,
{
    match web::block(f).await {
        Ok(Ok(())) => HttpResponse::NoContent().finish(),

        Ok(Err(e)) => {
            log::error!("CEC command failed: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }

        Err(e) => {
            log::error!("Actix blocking error: {}", e);
            HttpResponse::InternalServerError().body("internal worker error")
        }
    }
}

async fn run_cec_u8<F>(f: F, param: u8) -> HttpResponse
where
    F: FnOnce(u8) -> std::io::Result<()> + Send + 'static,
{
    match web::block(move || f(param)).await {
        Ok(Ok(())) => HttpResponse::NoContent().finish(),

        Ok(Err(e)) => {
            log::error!("CEC command failed: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }

        Err(e) => {
            log::error!("Actix blocking error: {}", e);
            HttpResponse::InternalServerError().body("internal worker error")
        }
    }
}

async fn run_cec_str<F>(f: F, param: &'static str) -> HttpResponse
where
    F: FnOnce(&str) -> std::io::Result<()> + Send + 'static,
{
    match web::block(move || f(&param)).await {
        Ok(Ok(())) => HttpResponse::NoContent().finish(),

        Ok(Err(e)) => {
            log::error!("CEC command failed: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }

        Err(e) => {
            log::error!("Actix blocking error: {}", e);
            HttpResponse::InternalServerError().body("internal worker error")
        }
    }
}

#[post("/api/register_playback")]
pub async fn register_playback() -> HttpResponse {
    run_cec_str(crate::cec::commands::register_playback, OSD_NAME).await
}

#[post("/api/power_off")]
pub async fn power_off() -> HttpResponse {
    run_cec(crate::cec::commands::power_off).await
}

#[post("/api/power_on")]
pub async fn power_on() -> HttpResponse {
    run_cec(crate::cec::commands::power_on).await
}

#[post("/api/volume_up")]
pub async fn volume_up() -> HttpResponse {
    run_cec(crate::cec::commands::volume_up).await
}

#[post("/api/volume_down")]
pub async fn volume_down() -> HttpResponse {
    run_cec(crate::cec::commands::volume_down).await
}

#[post("/api/toggle_mute")]
pub async fn toggle_mute() -> HttpResponse {
    run_cec(crate::cec::commands::toggle_mute).await
}

#[post("/api/register_playback/{source_physical_address_number}")]
pub async fn active_source(source_physical_address_number: web::Path<String>) -> HttpResponse {
    let p: u8 = source_physical_address_number.parse::<u8>().unwrap();

    run_cec_u8(crate::cec::commands::active_source, p).await
}
