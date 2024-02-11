mod api;
mod model;
mod repository;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use api::course::{create_course, get_all_courses, get_course, get_top_courses};
use repository::ddb::DDBRepository;

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

        App::new()
            .wrap(logger)
            .wrap(Cors::permissive())
            .app_data(ddb_data)
            .service(get_course)
            .service(get_all_courses)
            .service(create_course)
            .service(get_top_courses)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
