use actix_web::{App, HttpServer, middleware::Logger};
use tvc::{DOMAIN, load_config, routes::commands::{active_source, power_off, power_on, register_playback, toggle_mute, volume_down, volume_up}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = load_config()?;
    tvc::cec::commands::set_use_sudo(config.use_sudo);

    log::info!("starting HTTP server at http://{}:{}", DOMAIN, config.port);

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
    .bind((DOMAIN, config.port))?
    .run()
    .await
}
