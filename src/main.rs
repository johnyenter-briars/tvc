use actix_web::{App, HttpServer, middleware::Logger};
use tvc::{DOMAIN, PORT, routes::commands::{active_source, power_off, power_on, register_playback, toggle_mute, volume_down, volume_up}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://{}:{}", DOMAIN, PORT);

    HttpServer::new(|| {
        App::new()
            .service(register_playback)
            .service(power_off)
            .service(power_on)
            .service(volume_up)
            .service(volume_down)
            .service(toggle_mute)
            .service(active_source)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
    })
    .bind((DOMAIN, PORT))?
    .run()
    .await
}
