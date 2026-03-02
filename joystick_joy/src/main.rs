use rppal::gpio::{Gpio, InputPin};
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

const Z_PIN: u8 = 18; // WiringPi pin 1 (BCM mapping depends on board)

trait AdcDevice {
    fn analog_read(&mut self, channel: u8) -> Result<u8, Box<dyn std::error::Error>>;
}

struct PCF8591 {
    i2c: I2c,
}

struct ADS7830 {
    i2c: I2c,
}

impl PCF8591 {
    fn new(address: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;
        Ok(Self { i2c })
    }
}

impl ADS7830 {
    fn new(address: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;
        Ok(Self { i2c })
    }
}

impl AdcDevice for PCF8591 {
    fn analog_read(&mut self, channel: u8) -> Result<u8, Box<dyn std::error::Error>> {
        let control = 0x40 | (channel & 0x03);
        self.i2c.write(&[control])?;
        let mut buffer = [0u8];
        self.i2c.read(&mut buffer)?;
        Ok(buffer[0])
    }
}

impl AdcDevice for ADS7830 {
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

fn normalize_values(value: u8) -> u8 {
  //top left 0,0 | bottom right 20,20 
    ((value as u16 * 20) / 255) as u8
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Program is starting ...");

    // Detect ADC
    let mut adc: Box<dyn AdcDevice>;

    if detect_i2c(0x48) {
        println!("Detected PCF8591");
        adc = Box::new(PCF8591::new(0x48)?);
    } else if detect_i2c(0x4b) {
        println!("Detected ADS7830");
        adc = Box::new(ADS7830::new(0x4b)?);
    } else {
        println!("No correct I2C address found.");
        println!("Run: i2cdetect -y 1");
        return Ok(());
    }

    let gpio = Gpio::new()?;
    let z_pin: InputPin = gpio.get(Z_PIN)?.into_input_pullup();

    loop {
        let val_z = if z_pin.is_low() { 1 } else { 0 };
        let val_y = adc.analog_read(0)?;
        let val_x = adc.analog_read(1)?;

        println!("val_X: {} ,\tval_Y: {} ,\tval_Z: {}", normalize_values(val_x), normalize_values(val_y), val_z);

        thread::sleep(Duration::from_millis(100));
    }
}