//use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// The simple-signal crate is used to handle incoming signals.
use simple_signal::{self, Signal};

use rppal::gpio::Gpio;

#[post("/blink?<pin>&<frequency>")]
pub async fn blink_led(pin: u8, frequency: u64) {
    // Retrieve the GPIO pin and configure it as an output.
    let mut led = Gpio::new().unwrap().get(pin).unwrap().into_output();

    let running = Arc::new(AtomicBool::new(true));

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
    });

    // Blink the LED until running is set to false.
    while running.load(Ordering::SeqCst) {
        led.toggle();
        thread::sleep(Duration::from_millis(1 / frequency * 1000));
    }

    // After we're done blinking, turn the LED off.
    led.set_low();

    // When the pin variable goes out of scope, the GPIO pin mode is automatically reset
    // to its original value, provided reset_on_drop is set to true (default).
}
