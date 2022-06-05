use std::{error::Error, thread, time::Duration};

use rppal::gpio::Gpio;

pub async fn toggle(pin: u8, duration: u64) -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut led = Gpio::new()?.get(pin)?.into_output();

    for _ in 0..60 {
        led.toggle();
        thread::sleep(Duration::from_millis(duration));
    }

    led.set_low();

    Ok(())
}

pub async fn switch(pin: u8) -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut led = Gpio::new()?.get(pin)?.into_output();

    led.reset_on_drop();

    if led.is_set_low() {
        println!("LED is tured ON!.....");
        led.toggle();
    } else if led.is_set_high() {
        println!("LED is tured OFF!.....");
        led.toggle();
    }

    Ok(())
}