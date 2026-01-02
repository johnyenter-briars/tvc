use actix_web::{App, HttpRequest, HttpServer, middleware::Logger, web};
use tvc::routes::commands::power_off;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(power_off)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
