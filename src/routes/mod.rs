mod get_profile;
mod get_about;
mod get_personal_information;
mod get_education;
mod get_experience;
mod get_skills;
mod get_projects;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .route("/profile", actix_web::web::get().to(get_profile::get_profile))
            .route("/about", actix_web::web::get().to(get_about::get_about))
            .route("/personal_information", actix_web::web::get().to(get_personal_information::get_personal_information))
            .route("/education", actix_web::web::get().to(get_education::get_education))
            .route("/experience", actix_web::web::get().to(get_experience::get_experience))
            .route("/skills", actix_web::web::get().to(get_skills::get_skills))
            .route("/projects", actix_web::web::get().to(get_projects::get_projects))
    );
}