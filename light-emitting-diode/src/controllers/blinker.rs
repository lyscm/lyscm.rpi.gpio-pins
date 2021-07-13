use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rust_gpiozero::LED;

#[post("/blink/interval?<pin>&<frequency>&<duration>")]
pub async fn blink_per_interval(pin: u8, frequency: u64, duration: u64) -> String {
    let led = LED::new(pin);

    let now = Instant::now();
    let frequency_duration = Duration::new(1/frequency, 0);
    while duration > now.elapsed().as_secs() {
        led.on();
        sleep(frequency_duration);
        led.off();
        sleep(frequency_duration);
    }

    format!("blinked LED {:?} times per second.", duration/frequency)
}

#[post("/blink/count?<pin>&<amount>")]
pub async fn blink_per_count(pin: u8, amount: u64) -> String {
    blink_per_interval(pin, 1, amount).await
}
