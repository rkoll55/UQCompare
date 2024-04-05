mod api;
mod model;
mod repository;

use actix_web::{http, middleware::Logger, web::Data, App, HttpServer};
use api::router::routes;
use repository::ddb::DDBRepository;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "error");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let ddb_repo: DDBRepository =
            DDBRepository::init(String::from("uqcompare_courses"), config.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger = Logger::default();

        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(ddb_data.clone())
            .configure(routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
