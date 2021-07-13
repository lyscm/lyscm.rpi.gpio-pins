use rust_gpiozero::LED;

#[post("/blink/interval?<pin>&<frequency>")]
pub async fn blink_per_interval(pin: u8, frequency: f32) {
    let mut led = LED::new(pin);

    format!("blinking LED {:?} times per second.", frequency);
    led.blink(frequency, frequency);
}

#[post("/blink/count?<pin>&<amount>")]
pub async fn blink_per_count(pin: u8, amount: i32) {
    let mut led = LED::new(pin);

    format!("blinking LED for a total amount of {:?} times.", amount);
    led.set_blink_count(amount);
}