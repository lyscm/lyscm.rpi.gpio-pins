#[macro_use]
extern crate rocket;


use crate::{controllers::switcher::*};
use crate::{controllers::blinker::*};
use rocket::Error;

mod controllers;

const BASE_URL: &str = "/v1.0/gpio/led";

#[catch(500)]
fn device_not_found() -> &'static str {
    "Raspberry Pi device cannot be found!"
}

#[catch(404)]
fn resource_not_found() -> &'static str {
    "Endpoint not found! Please check for the correct URL and the respective parameters."
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        // Faults
        .register(BASE_URL, catchers![resource_not_found])
        .register(BASE_URL, catchers![device_not_found])
        // Switch routes
        .mount(BASE_URL, routes![switch_status])
        // Blinker routes
        .mount(BASE_URL, routes![blink_per_interval])
        .mount(BASE_URL, routes![blink_per_count])
        .launch()
        .await
}