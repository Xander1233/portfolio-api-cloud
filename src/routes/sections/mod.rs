mod get_profile;
mod get_about;
mod get_personal_information;
mod get_education;
mod get_experience;
mod get_skills;
mod get_projects;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/sections")
            .service(get_about::get_about)
            .service(get_profile::get_profile)
            .service(get_personal_information::get_personal_information)
            .service(get_education::get_education)
            .service(get_experience::get_experience)
            .service(get_skills::get_skills)
            // .service(get_projects::get_projects)
    );
}