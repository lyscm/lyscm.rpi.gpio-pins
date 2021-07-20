use std::{error::Error, thread, time::Duration};

use rppal::gpio::Gpio;

pub async fn toggle_led(pin: u8, duration: u64) -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut led = Gpio::new()?.get(pin)?.into_output();

    loop {
        led.toggle();
        thread::sleep(Duration::from_millis(duration));
    }
}
