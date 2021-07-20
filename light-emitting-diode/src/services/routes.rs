// src/requests/routes.rs
use crate::{services::model::{CommandTypes, Request}};
use crate::libs::toggle_led;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/transit")]
async fn set_transit(req: web::Json<Request>) -> impl Responder {
    let pin = req.pin;
    let duration = req.duration;
    match req.command_type {
        CommandTypes::Switch => HttpResponse::MethodNotAllowed().json("unknown method"),
        CommandTypes::Blink => match toggle_led(pin, duration).await {
            Ok(rsp) => HttpResponse::Ok().json(rsp),
            Err(e) => return HttpResponse::BadRequest().json(e.to_string())
        },
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_transit);
}
