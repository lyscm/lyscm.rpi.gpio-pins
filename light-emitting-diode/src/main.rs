#[macro_use]
extern crate rocket;

use crate::blink::libs::*;
use rocket::Error;

mod blink;

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
        .register(BASE_URL, catchers![resource_not_found])
        .register(BASE_URL, catchers![device_not_found])
        .mount(BASE_URL, routes![blink_led])
        .launch()
        .await
}
