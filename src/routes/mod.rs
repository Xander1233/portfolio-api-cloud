mod sections;
mod health;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(sections::config)
            .service(health::health)
    );
}