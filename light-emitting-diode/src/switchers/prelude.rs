use rust_gpiozero::LED;

#[derive(Debug, PartialEq, FromFormField)]
pub enum Status {
    On,
    Off,
}

#[post("/switch?<pin>&<status>")]
pub async fn switch_status(pin: u8, status: Status) -> String {
    let led = LED::new(pin);

    match status {
        Status::On => led.on(),
        Status::Off => led.off(),
    }

    format!("switched lights {:?}", status)
}