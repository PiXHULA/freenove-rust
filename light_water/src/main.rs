use rppal::gpio::{Gpio, Level, OutputPin};
use std::{thread, time::Duration};

const LED_COUNT: usize = 10;
const PINS: [u8; LED_COUNT] = [17, 18, 27, 22, 23, 24, 25, 2, 3, 8];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let gpio = Gpio::new()?;

    // Store all output pins
    let mut leds: Vec<OutputPin> = Vec::new();
    for &pin in PINS.iter() {
        leds.push(gpio.get(pin)?.into_output_high()); // start OFF
    }

    loop {
        // Left → right
        for led in leds.iter_mut() {
            led.write(Level::Low); // ON
            thread::sleep(Duration::from_millis(100));
            led.write(Level::High); // OFF
        }

        // Right → left
        for led in leds.iter_mut().rev() {
            led.write(Level::Low); // ON
            thread::sleep(Duration::from_millis(100));
            led.write(Level::High); // OFF
        }
    }
}
