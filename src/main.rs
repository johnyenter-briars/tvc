use actix_web::{App, HttpServer, middleware::Logger};
use tvc::routes::commands::power_off;

const PORT: u16 = 8080;
const DOMAIN: &str = "0.0.0.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://{}:{}", DOMAIN, PORT);

    HttpServer::new(|| {
        App::new()
            .service(power_off)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))

    })
    .bind((DOMAIN, PORT))?
    .run()
    .await
}
