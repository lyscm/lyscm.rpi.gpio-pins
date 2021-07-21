// src/requests/routes.rs
use actix_web::{get, web, HttpResponse, Responder};

use super::print_status;

#[get("/mgmt/status")]
async fn get_status() -> impl Responder {
    match print_status().await {
        Ok(rsp) => HttpResponse::Ok().json(rsp),
        Err(e) => return HttpResponse::BadRequest().json(e.to_string())
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
}
