use std::fs::{File, OpenOptions};
use i2c_linux::I2c;
use sysfs_gpio::{Direction, Pin};

use common::error::{HardwareError,DriverType};
use crate::error_handler::{handle_i2c_creation_error, handle_i2c_set_slave_error, handle_pin_direction_error, handle_pin_export_error};

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

pub struct DriversCnRs232{
    i2c_handler: Option<I2c<File>>,
    pin_iqr: Option<Pin>
}

impl Default for DriversCnRs232{
    fn default() -> Self{
        Self{
            i2c_handler: None,
            pin_iqr: None,
        }
    }
}


impl DriversCnRs232{
    pub fn new() -> Result<Self,HardwareError>{
        let mut driver = Self::default();
        let i2c = handle_i2c_creation_error("/dev/i2c-2".to_string())?;
        let pin = Pin::new(2);
        handle_pin_export_error(pin)?;
        handle_pin_direction_error(pin,Direction::In)?;
        driver.pin_iqr = Some(pin);
        driver.i2c_handler = Some(i2c);
        Ok(driver)
    }

    fn get_i2c_handler(self) -> I2c<File>{
        self.i2c_handler.unwrap()
    }

    pub fn write_i2c(&self, data: &[u8],type_cn : DriverType) -> Result<(),HardwareError>{
        let i2c_addr = get_i2c_addr_value(type_cn);

        println!("set addr slave to :{}",i2c_addr);
        // handle_i2c_set_slave_error(self.get_i2c_handler(), i2c_addr as u16,type_cn)?;
        self.get_i2c_handler();
        println!("command the slave to write to RS232 :{}",i2c_addr);
        for n in data.into_iter(){
            //self.get_i2c_handler().smbus_write_byte(0x00)?;
            println!("write the data {}",n);
            //self.get_i2c_handler().smbus_write_byte(*n)?;

            /*let mut result = true;
            while result {
                self.get_i2c_handler().smbus_write_byte(0x63)?;
                self.get_i2c_handler().smbus_read_byte()?;
                let count = self.get_i2c_handler().smbus_read_byte()?;
                let new = self.get_i2c_handler().smbus_read_byte()?;
                if count == 1 && new&(1)==1 {
                    result = false;
                }
            }*/
        }
        Ok(())
    }
}
