use actix_web::web;
use super::requests::{ get_course, get_all_courses, create_course };

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/get/{course_code}", web::get().to(get_course))
            .route("/getall", web::get().to(get_all_courses))
            .route("/create", web::post().to(create_course)),
    );
}
