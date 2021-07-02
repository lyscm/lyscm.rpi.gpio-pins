use rust_gpiozero::LED;

#[derive(Debug, PartialEq, FromFormField)]
pub enum Status {
    On,
    Off,
}

#[post("/led?<pin>&<status>")]
pub async fn set_status(pin: u8, status: Status) -> String {
    let led = LED::new(pin);

    match status {
        Status::On => led.on(),
        Status::Off => led.off(),
    }

    format!("turned lights {:?}", status)
}

#[catch(500)]
pub fn device_not_found() -> &'static str {
    "Raspberry Pi device cannot be found!"
}