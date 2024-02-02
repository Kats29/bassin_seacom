use std::fs::File;
use i2c_linux::I2c;
use sysfs_gpio::{Direction, Pin};

use common::{
    error::HardwareError,
    definitions::DriverType,
};
use crate::error_handler::{handle_i2c_creation_error, handle_i2c_read_error, handle_i2c_set_slave_error, handle_i2c_write_error, handle_pin_direction_error, handle_pin_export_error, handle_pin_read_error};

pub fn get_i2c_addr_value(i2c_type: DriverType) -> u8 {
    return match i2c_type {
        DriverType::EX => ADDR_X_E,
        DriverType::EY => ADDR_Y_E,
        DriverType::EZ => ADDR_Z_E,
        DriverType::ETHETA => ADDR_T_E,
        DriverType::RX => ADDR_X_R,
        DriverType::RY => ADDR_Y_R,
        DriverType::RZ => ADDR_Z_R,
        DriverType::RTHETA => ADDR_T_R,
        _ => 0x00,
    }
}

pub fn get_iqr_pin(i2c_type: DriverType) -> Pin {
    match i2c_type {
        DriverType::RX | DriverType::EX => Pin::new(0x1),
        DriverType::EY | DriverType::RY => Pin::new(0x1),
        DriverType::EZ | DriverType::RZ => Pin::new(0x1),
        DriverType::ETHETA | DriverType::RTHETA => Pin::new(0x1),
        _ => Pin::new(0x00),
    }
}

const ADDR_X_E: u8 = 0x6C;
const ADDR_Y_E: u8= 0x61;
const ADDR_Z_E: u8 = 0x64;
const ADDR_T_E: u8 = 0x65;

const ADDR_X_R: u8 = 0x5C;
const ADDR_Y_R: u8= 0x51;
const ADDR_Z_R: u8 = 0x54;
const ADDR_T_R: u8 = 0x55;

// #[derive(Copy, Clone)]
pub struct DriversCnRs232{
    i2c_handler: Option<I2c<File>>,
}

impl Default for DriversCnRs232{
    fn default() -> Self{
        Self{
            i2c_handler: None,
        }
    }
}


impl DriversCnRs232{
    pub fn new() -> Result<Self,HardwareError>{
        let mut driver = Self::default();
        
        let mut pin = Pin::new(2);
        handle_pin_export_error(pin)?;
        handle_pin_direction_error(pin,Direction::In)?;

        pin = Pin::new(2);
        handle_pin_export_error(pin)?;
        handle_pin_direction_error(pin,Direction::In)?;

        pin = Pin::new(2);
        handle_pin_export_error(pin)?;
        handle_pin_direction_error(pin,Direction::In)?;


        pin = Pin::new(2);
        handle_pin_export_error(pin)?;
        handle_pin_direction_error(pin,Direction::In)?;

        driver.i2c_handler = None;
        Ok(driver)
    }

    pub fn write_i2c(self, data: [u8; 9], type_cn : DriverType) -> Result<(),HardwareError>{
        let i2c_addr = get_i2c_addr_value(type_cn);
        let iqr_pin = get_iqr_pin(type_cn);
        println!("set addr slave to :{}",i2c_addr);
        handle_i2c_set_slave_error(self.clone().i2c_handler.unwrap(), i2c_addr as u16,type_cn)?;
        println!("command the slave to write to RS232 :{}",i2c_addr);
        for n in data.into_iter(){
            handle_i2c_write_error(self.clone().i2c_handler.unwrap(),0x00,n,type_cn)?;
            println!("write the data {}",n);
            println!("Wait for the interrupt (mean got a response)");

            while handle_pin_read_error(iqr_pin)? == 1 {}

            println!("Read the interupt register to clear it");
            handle_i2c_read_error(self.clone().i2c_handler.unwrap(), 0x02, type_cn)?;

            println!("Read the return value");
            let g = handle_i2c_read_error(self.clone().i2c_handler.unwrap(), 0x00, type_cn)?;

            println!("Compare it the value writed");
            if g!=n {
                return Err(HardwareError::BadI2cResponse(type_cn,g,n));
            }
        }
        Ok(())
    }
}

impl Clone for DriversCnRs232 {
     fn clone(&self) -> Self {
         let mut clone = DriversCnRs232::default();
         clone.i2c_handler = Some(handle_i2c_creation_error("/dev/i2c".to_string()).unwrap());
         return clone;
     }
}