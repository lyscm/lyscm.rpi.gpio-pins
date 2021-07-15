use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

#[derive(Debug, PartialEq, FromFormField)]
pub enum Status {
    On,
    Off,
}

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
#[post("/switch?<pin>&<status>")]
pub async fn switch_status(pin: u8, status: Status) -> String {
    println!(
        "Blinking an LED on a {}.",
        DeviceInfo::new().unwrap().model()
    );

    let mut led = Gpio::new().unwrap().get(pin).unwrap().into_output();

    match status {
        Status::On => led.set_high(),
        Status::Off => led.set_low(),
    }

    format!("Set lights status to {:?}", status)
}
