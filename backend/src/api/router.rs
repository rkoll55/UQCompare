use super::requests::{create_course, get_all_courses, get_course, get_top_courses};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/get/{course_code}", web::get().to(get_course))
            .route("/getall", web::get().to(get_all_courses))
            .route("/create", web::post().to(create_course))
            .route("/top/{num}", web::get().to(get_top_courses)),
    );
}
