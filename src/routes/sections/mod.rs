mod about;
mod education;
mod experience;
mod handler;
mod personal_information;
mod profile;
mod projects;
mod skills;
mod testimonials;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sections")
            .service(about::get_about)
            .service(personal_information::get_personal_information)
            .service(education::get_education)
            .service(experience::get_experience)
            .service(skills::get_skills)
            .service(projects::get_projects)
            .service(testimonials::get_testimonials)
            .service(profile::get_profile),
    );
}
