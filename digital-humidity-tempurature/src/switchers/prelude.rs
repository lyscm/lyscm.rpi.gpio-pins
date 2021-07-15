use std::thread;
use std::time::Duration;

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

    let mut pin = Gpio::new().unwrap().get(pin).unwrap().into_output();
    
    
    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(1000));
    }
}
