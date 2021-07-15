#[macro_use]
extern crate rocket;

use crate::switchers::prelude::*;
use rocket::Error;

mod switchers;

const BASE_URL: &str = "/v1.0/gpio/hat";

#[rocket::main]
async fn main() -> Result<(), Error> {
    rocket::build()
        .mount(BASE_URL, routes![switch_status])
        .launch()
        .await
}
