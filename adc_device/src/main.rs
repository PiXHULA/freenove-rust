use rppal::i2c::I2c;
use std::{thread, time::Duration};

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
        let control_byte = 0x40 | (channel & 0x03);
        self.i2c.write(&[control_byte])?;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    let mut adc: Box<dyn AdcDevice>;

    if detect_i2c(0x48) {
        println!("Detected PCF8591 at 0x48");
        adc = Box::new(Pcf8591::new(0x48)?);
    } else if detect_i2c(0x4b) {
        println!("Detected ADS7830 at 0x4B");
        adc = Box::new(Ads7830::new(0x4b)?);
    } else {
        println!("No correct I2C address found.");
        println!("Please run: i2cdetect -y 1");
        return Ok(());
    }

    loop {
        let adc_value = adc.analog_read(0)?;
        let voltage = adc_value as f32 / 255.0 * 3.3;

        println!("ADC value: {} , Voltage: {:.2}V", adc_value, voltage);

        thread::sleep(Duration::from_millis(100));
    }
}