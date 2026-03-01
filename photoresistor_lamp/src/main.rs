use rppal::gpio::{Gpio, OutputPin};
use rppal::i2c::I2c;
use std::{thread, time::Duration};

const LED_PIN: u8 = 17; // BCM pin (adjust if needed)

trait AdcDevice {
    fn analog_read(&mut self, channel: u8) -> Result<u8, Box<dyn std::error::Error>>;
}

struct Pcf8591 {
    i2c: I2c,
}

struct Ads7830 {
    i2c: I2c,
}

impl Pcf8591 {
    fn new(address: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;
        Ok(Self { i2c })
    }
}

impl Ads7830 {
    fn new(address: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;
        Ok(Self { i2c })
    }
}

impl AdcDevice for Pcf8591 {
    fn analog_read(&mut self, channel: u8) -> Result<u8, Box<dyn std::error::Error>> {
        let control = 0x40 | (channel & 0x03);
        self.i2c.write(&[control])?;
        let mut buffer = [0u8];
        self.i2c.read(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl AdcDevice for Ads7830 {
    fn analog_read(&mut self, channel: u8) -> Result<u8, Box<dyn std::error::Error>> {
        let command = 0x84 | ((channel & 0x07) << 4);
        self.i2c.write(&[command])?;
        let mut buffer = [0u8];
        self.i2c.read(&mut buffer)?;
        Ok(buffer[0])
    }
}

fn detect_i2c(address: u16) -> bool {
    if let Ok(mut i2c) = I2c::new() {
        if i2c.set_slave_address(address).is_ok() {
            return i2c.write(&[]).is_ok();
        }
    }
    false
}

// Software PWM (common-cathode version: HIGH = ON)
// If your LED is common-anode, invert HIGH/LOW
fn software_pwm(pin: &mut OutputPin, duty: u8) {
    let period = Duration::from_millis(10); // 100 Hz
    let on_time = period * duty as u32 / 100;
    let off_time = period - on_time;

    pin.set_high();
    thread::sleep(on_time);
    pin.set_low();
    thread::sleep(off_time);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting...");

    // Detect ADC
    let mut adc: Box<dyn AdcDevice>;

    if detect_i2c(0x48) {
        println!("Detected PCF8591");
        adc = Box::new(Pcf8591::new(0x48)?);
    } else if detect_i2c(0x4b) {
        println!("Detected ADS7830");
        adc = Box::new(Ads7830::new(0x4b)?);
    } else {
        println!("No correct I2C address found.");
        println!("Run: i2cdetect -y 1");
        return Ok(());
    }

    // Setup LED
    let gpio = Gpio::new()?;
    let mut led = gpio.get(LED_PIN)?.into_output();

    loop {
        let value = adc.analog_read(0)?; // A0
        let duty = ((value as u16 * 100) / 255) as u8;

        let voltage = value as f32 / 255.0 * 3.3;

        println!("ADC value: {} , Voltage: {:.2}V", value, voltage);

        // Run PWM cycle for 100ms
        for _ in 0..10 {
            software_pwm(&mut led, duty);
        }
    }
}