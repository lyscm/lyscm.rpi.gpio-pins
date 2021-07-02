#[macro_use]
extern crate rocket;

use crate::libs::*;
use rocket::Error;
mod libs;

const BASE_URL: &str = "/v1.0/gpio";

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        .register(BASE_URL, catchers![device_not_found])
        .mount(BASE_URL, routes![set_status])
        .launch()
        .await
}
