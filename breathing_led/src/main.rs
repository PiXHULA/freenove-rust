use rppal::gpio::{Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;

const GPIO_PIN: u8 = 18; // wiringPi pin 1
const PWM_RANGE: u32 = 100;
const PWM_PERIOD_US: u64 = 1000; // 1 ms PWM period

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let gpio = Gpio::new()?;
    let mut led: OutputPin = gpio.get(GPIO_PIN)?.into_output();

    loop {
        // Fade in
        for duty in 0..PWM_RANGE {
            soft_pwm(&mut led, duty, PWM_RANGE);
            sleep(Duration::from_millis(20));
        }

        sleep(Duration::from_millis(300));

        // Fade out
        for duty in (1..=PWM_RANGE).rev() {
            soft_pwm(&mut led, duty, PWM_RANGE);
            sleep(Duration::from_millis(20));
        }

        sleep(Duration::from_millis(300));
    }
}

fn soft_pwm(pin: &mut OutputPin, duty: u32, range: u32) {
    let gamma = (duty * duty) / range; // gamma correction
    let on_time = PWM_PERIOD_US * gamma as u64 / range as u64;
    let off_time = PWM_PERIOD_US - on_time;

    if on_time > 0 {
        pin.set_high();
        sleep(Duration::from_micros(on_time));
    }
    if off_time > 0 {
        pin.set_low();
        sleep(Duration::from_micros(off_time));
    }
}
