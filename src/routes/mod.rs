mod health;
mod sections;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(sections::config)
            .service(health::health),
    );
}
