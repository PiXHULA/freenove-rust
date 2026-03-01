use rppal::gpio::{Gpio, Level};
use std::{thread, time::Duration, time::Instant};

const LED_PIN: u8 = 17; // GPIO17
const BUTTON_PIN: u8 = 18; // GPIO18

const CAPTURE_TIME_MS: u64 = 50; // debounce time

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let gpio = Gpio::new()?;

    // LED as output
    let mut led = gpio.get(LED_PIN)?.into_output();
    let mut led_state = Level::Low;

    // Button as input with pull-up
    let button = gpio.get(BUTTON_PIN)?.into_input_pullup();

    let mut button_state = Level::High;
    let mut last_button_state = Level::High;
    let mut last_change_time = Instant::now();

    loop {
        let reading = button.read();

        if reading != last_button_state {
            last_change_time = Instant::now();
        }

        if last_change_time.elapsed() > Duration::from_millis(CAPTURE_TIME_MS) {
            if reading != button_state {
                button_state = reading;

                if button_state == Level::Low {
                    println!("Button is pressed!");

                    led_state = match led_state {
                        Level::Low => {
                            println!("turn on LED ...");
                            Level::High
                        }
                        Level::High => {
                            println!("turn on LED ...");
                            Level::Low
                        }
                    };
                } else {
                    println!("Button is released");
                }
            }
        }
        led.write(led_state);
        last_button_state = reading;

        thread::sleep(Duration::from_millis(5));
    }
}
