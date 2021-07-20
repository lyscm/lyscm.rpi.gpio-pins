// src/requests/routes.rs
use crate::libs::print_status;
use actix_web::{get, web, HttpResponse, Responder};


#[get("/status")]
async fn get_status() -> impl Responder {
    match print_status().await {
        Ok(rsp) => HttpResponse::Ok().json(rsp),
        Err(e) => return HttpResponse::BadRequest().json(e.to_string())
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status);
}
