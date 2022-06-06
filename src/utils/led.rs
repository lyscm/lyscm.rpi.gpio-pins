use std::{error::Error, thread, time::Duration};

use rppal::gpio::{Gpio, Level};

pub async fn blink(pin: u8, duration: u64) -> Result<String, Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let mut led = Gpio::new()?.get(pin)?.into_output();

    for _ in 0..60 {
        led.toggle();
        thread::sleep(Duration::from_millis(duration));
    }

    led.set_low();

    Ok("succesfully switch LED".to_string())
}

pub async fn switch(pin: u8) -> Result<String, Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    let led = Gpio::new()?.get(pin)?;

    Ok(match led.read() {
        Level::Low => {
            led.into_output_high();
            "LED is tured ON!.....".to_string()
        }
        Level::High => {
            led.into_output_low();
            "LED is tured OFF!.....".to_string()
        }
    })
}
