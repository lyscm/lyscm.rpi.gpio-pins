use std::error::Error;
use std::fmt;
use std::process;

use rppal::gpio::Gpio;
use rppal::system::{DeviceInfo, Model};

enum PinType {
    Gpio(u8),
    Ground,
    Power3v3,
    Power5v,
}

impl fmt::Display for PinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PinType::Gpio(pin) => write!(f, "GPIO{}", pin),
            PinType::Ground => write!(f, "{:<5}", "GND"),
            PinType::Power3v3 => write!(f, "{:<5}", "3.3 V"),
            PinType::Power5v => write!(f, "{:<5}", "5 V"),
        }
    }
}

pub struct Status {
    pub gpio: String,
    pub mode: String,
    pub level: String,
    pub pin: String,
}

const HEADER: [PinType; 40] = [
    PinType::Power3v3, // Physical pin 1
    PinType::Power5v,  // Physical pin 2
    PinType::Gpio(2),  // Physical pin 3
    PinType::Power5v,  // Physical pin 4
    PinType::Gpio(3),  // Physical pin 5
    PinType::Ground,   // Physical pin 6
    PinType::Gpio(4),  // Physical pin 7
    PinType::Gpio(14), // Physical pin 8
    PinType::Ground,   // Physical pin 9
    PinType::Gpio(15), // Physical pin 10
    PinType::Gpio(17), // Physical pin 11
    PinType::Gpio(18), // Physical pin 12
    PinType::Gpio(27), // Physical pin 13
    PinType::Ground,   // Physical pin 14
    PinType::Gpio(22), // Physical pin 15
    PinType::Gpio(23), // Physical pin 16
    PinType::Power3v3, // Physical pin 17
    PinType::Gpio(24), // Physical pin 18
    PinType::Gpio(10), // Physical pin 19
    PinType::Ground,   // Physical pin 20
    PinType::Gpio(9),  // Physical pin 21
    PinType::Gpio(25), // Physical pin 22
    PinType::Gpio(11), // Physical pin 23
    PinType::Gpio(8),  // Physical pin 24
    PinType::Ground,   // Physical pin 25
    PinType::Gpio(7),  // Physical pin 26
    PinType::Gpio(0),  // Physical pin 27
    PinType::Gpio(1),  // Physical pin 28
    PinType::Gpio(5),  // Physical pin 29
    PinType::Ground,   // Physical pin 30
    PinType::Gpio(6),  // Physical pin 31
    PinType::Gpio(12), // Physical pin 32
    PinType::Gpio(13), // Physical pin 33
    PinType::Ground,   // Physical pin 34
    PinType::Gpio(19), // Physical pin 35
    PinType::Gpio(16), // Physical pin 36
    PinType::Gpio(26), // Physical pin 37
    PinType::Gpio(20), // Physical pin 38
    PinType::Ground,   // Physical pin 39
    PinType::Gpio(21), // Physical pin 40
];

const MAX_PINS_SHORT: usize = 26;
const MAX_PINS_LONG: usize = 40;

fn format_pin(
    pin: usize,
    gpio: impl fmt::Display,
    mode: impl fmt::Display,
    level: impl fmt::Display,
) -> Status {
    Status {
        pin: format!("{:>2}", pin),
        level: format!("{:>1}", level),
        mode: format!("{:<5}", mode),
        gpio: format!("{:>4}", gpio),
    }
}

fn get_gpio(header: &[PinType]) -> Result<Vec<Status>, Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut stats = vec![];

    for (idx, pin_type) in header.iter().enumerate() {
        match pin_type {
            PinType::Gpio(bcm_gpio) => {
                // Retrieve a Pin without converting it to an InputPin,
                // OutputPin or IoPin, so we can check the pin's mode
                // and level without affecting its state.
                let pin = gpio.get(*bcm_gpio)?;

                stats.push(format_pin(
                    idx + 1,
                    bcm_gpio,
                    format!("{}", pin.mode()).to_uppercase(),
                    pin.read() as u8,
                ));
            }
            _ => stats.push(format_pin(idx + 1, "", pin_type, "")),
        };
    }

    Ok(stats)
}

pub async fn get_gpio_info() -> Result<Vec<Status>, Box<dyn Error>> {
    // Identify the Pi's model, so we can print the appropriate GPIO header.
    match DeviceInfo::new()?.model() {
        Model::RaspberryPiBRev1 => {
            // The GPIO header on the earlier Pi models mostly overlaps with the first 26 pins of
            // the 40-pin header on the newer models. A few pins are switched on the Pi B Rev 1.
            let mut header_rev1 = HEADER;
            header_rev1[2] = PinType::Gpio(0);
            header_rev1[4] = PinType::Gpio(1);
            header_rev1[12] = PinType::Gpio(21);

            get_gpio(&header_rev1[..MAX_PINS_SHORT])
        }
        Model::RaspberryPiA | Model::RaspberryPiBRev2 => get_gpio(&HEADER[..MAX_PINS_SHORT]),
        Model::RaspberryPiAPlus
        | Model::RaspberryPiBPlus
        | Model::RaspberryPi2B
        | Model::RaspberryPi3APlus
        | Model::RaspberryPi3B
        | Model::RaspberryPi3BPlus
        | Model::RaspberryPi4B
        | Model::RaspberryPiZero
        | Model::RaspberryPiZeroW => get_gpio(&HEADER[..MAX_PINS_LONG]),
        model => {
            eprintln!("Error: No GPIO header information available for {}", model);
            process::exit(1);
        }
    }
}

pub async fn get_device_model() -> Result<Model, Box<dyn Error>> {
    Ok(DeviceInfo::new()?.model())
}