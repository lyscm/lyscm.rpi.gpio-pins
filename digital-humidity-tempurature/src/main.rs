#[macro_use]
extern crate rocket;

use crate::switchers::prelude::*;
use crate::status::device_info::*;
use rocket::Error;

mod switchers;
mod status;

const BASE_URL: &str = "/v1.0/gpio/hat";

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        .mount(BASE_URL, routes![switch_status])
        .mount(BASE_URL, routes![get_display])
        .launch()
        .await
}
