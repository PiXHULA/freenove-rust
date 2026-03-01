use clap::Parser;
use rand::Rng;
use rppal::gpio::{Gpio, OutputPin};
use std::{
    sync::{Arc, atomic::{AtomicU8, Ordering}},
    thread,
    time::Duration,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    red: bool,
    #[arg(long)]
    green: bool,
    #[arg(long)]
    blue: bool,
}

fn software_pwm(pin: &mut OutputPin, duty: u8) {
    let period = Duration::from_millis(10); // 100 Hz
    let on_time = period * duty as u32 / 100;
    let off_time = period - on_time;

    pin.set_low();
    thread::sleep(on_time);
    pin.set_high();
    thread::sleep(off_time);
}

fn pwm_thread(mut pin: rppal::gpio::OutputPin, duty: Arc<AtomicU8>) {
    let period = Duration::from_millis(10);

    loop {
        let d = duty.load(Ordering::Relaxed);
        let on_time = period * d as u32 / 100;
        let off_time = period - on_time;

        pin.set_low(); // ON (common-anode)
        thread::sleep(on_time);
        pin.set_high(); // OFF
        thread::sleep(off_time);
    }
}

fn turn_on_color(color: &str, red: &mut OutputPin, green: &mut OutputPin, blue: &mut OutputPin) {
    red.set_high();
    green.set_high();
    blue.set_high();

    match color {
        "red" => red.set_low(),
        "green" => green.set_low(),
        "blue" => blue.set_low(),
        _ => {
            eprintln!("Use only one of --red, --green, --blue");
            std::process::exit(1);
        }
    }
}

fn gamma_correct(value: u8) -> u8 {
    let gamma = 2.2;
    let normalized = value as f64 / 100.0;
    ((normalized.powf(gamma)) * 100.0) as u8
}

fn fade(current: &mut u8, target: u8) {
    if *current < target {
        *current += 1;
    } else if *current > target {
        *current -= 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let gpio = Gpio::new()?;

    // BCM pins (adjust if needed)
    let red_pin = gpio.get(17)?.into_output();
    let blue_pin = gpio.get(18)?.into_output();
    let green_pin = gpio.get(27)?.into_output();

    let red_duty = Arc::new(AtomicU8::new(0));
    let green_duty = Arc::new(AtomicU8::new(0));
    let blue_duty = Arc::new(AtomicU8::new(0));

    // Spawn PWM threads
    {
        let duty = red_duty.clone();
        thread::spawn(|| pwm_thread(red_pin, duty));
    }
    {
        let duty = green_duty.clone();
        thread::spawn(|| pwm_thread(green_pin, duty));
    }
    {
        let duty = blue_duty.clone();
        thread::spawn(|| pwm_thread(blue_pin, duty));
    }

    let mut random_generator = rand::thread_rng();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    loop {
        let mut target_red = 0;
        let mut target_green = 0;
        let mut target_blue = 0;

        match (args.red, args.green, args.blue) {
            (true, false, false) => target_red = random_generator.gen_range(0..100),
            (false, true, false) => target_green = random_generator.gen_range(0..100),
            (false, false, true) => target_blue = random_generator.gen_range(0..100),
            (false, false, false) => {
                target_red = random_generator.gen_range(0..100);
                target_green = random_generator.gen_range(0..100);
                target_blue = random_generator.gen_range(0..100);
            }
            _ => {
                eprintln!("Use only one of --red, --green, --blue");
                std::process::exit(1);
            }
        }

        // Fade toward target
        for _ in 0..100 {
            fade(&mut red, target_red);
            fade(&mut green, target_green);
            fade(&mut blue, target_blue);

            red_duty.store(gamma_correct(red), Ordering::Relaxed);
            green_duty.store(gamma_correct(green), Ordering::Relaxed);
            blue_duty.store(gamma_correct(blue), Ordering::Relaxed);

            thread::sleep(Duration::from_millis(10));
        }
    }
}
