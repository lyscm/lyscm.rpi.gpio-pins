use rppal::gpio::{Gpio};
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
#[post("/switch?<pin>")]
pub async fn switch_status(pin: u8) {
    println!(
        "Blinking an LED on a {}.",
        DeviceInfo::new().unwrap().model()
    );

    let mut pin = Gpio::new().unwrap().get(pin).unwrap().into_output();

    if pin.is_set_high() {      
        println!("Turning off LED...");
    } else if pin.is_set_low() {
        println!("Turning on LED...");        
    }

    pin.toggle();
}
