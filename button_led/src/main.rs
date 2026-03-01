use rppal::gpio::{Gpio, Level};
use std::{thread, time::Duration};

const LED_PIN: u8 = 17;     // GPIO17
const BUTTON_PIN: u8 = 18;  // GPIO18

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let gpio = Gpio::new()?;

    // LED as output
    let mut led = gpio.get(LED_PIN)?.into_output();

    // Button as input with pull-up
    let button = gpio
        .get(BUTTON_PIN)?
        .into_input_pullup();

    loop {
        if button.read() == Level::Low {
            // Button pressed
            led.set_high();
            println!("Button is pressed, led turned on >>>");
        } else {
            // Button released
            led.set_low();
            println!("Button is released, led turned off <<<");
        }

        // Small delay to reduce CPU usage & log spam
        thread::sleep(Duration::from_millis(50));
    }
}
