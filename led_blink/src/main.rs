use rppal::gpio::{Gpio, OutputPin};
use std::{thread, time::Duration};

const LED1: u8 = 17; // GPIO17
const LED2: u8 = 18; // GPIO18
const LED3: u8 = 27; // GPIO27

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let gpio = Gpio::new()?;

    let mut led1 = Led {
        name: "1",
        pin: gpio.get(17)?.into_output(),
    };
    let mut led2 = Led {
        name: "2",
        pin: gpio.get(18)?.into_output(),
    };
    let mut led3 = Led {
        name: "3",
        pin: gpio.get(27)?.into_output(),
    };

    println!("Using pins {}, {}, {}", LED1, LED2, LED3);

    loop {
        led1.blink();
        led2.blink();
        led3.blink();
    }

    struct Led {
        name: &'static str,
        pin: OutputPin,
    }

    impl Led {
        fn blink(&mut self) {
            self.pin.set_high();
            println!("led [{}] pin [{}] ON", self.name, self.pin.pin());
            thread::sleep(Duration::from_secs(1));

            self.pin.set_low();
            println!("led [{}] pin [{}] OFF", self.name, self.pin.pin());
        }
    }
}
