use std::fs::{File, OpenOptions};
use sysfs_gpio::Pin;


pub enum I2cAddr {
    AddrXE,
    AddrYE,
    AddrZE,
    AddrTE,
    AddrXR,
    AddrYR,
    AddrZR,
    AddrTR,
}

const ADDR_X_E: u8 = 0x1;
const ADDR_Y_E: u8= 0x1;
const ADDR_Z_E: u8 = 0x1;
const ADDR_T_E: u8 = 0x1;

const ADDR_X_R: u8 = 0x1;
const ADDR_Y_R: u8= 0x1;
const ADDR_Z_R: u8 = 0x1;
const ADDR_T_R: u8 = 0x1;


pub struct DriversCnRs232{
    i2c_handler: Option<File>
}

impl Default for DriversCnRs232{
    fn default() -> Self{
        Self{
            i2c_handler: None
        }
    }
}

impl DriversCnRs232{
    pub fn new() -> Self{
        let mut driver = Self::default();
        let mut i2c = OpenOptions::new().read(true).write(true).open("/dev/i2c-1").expect("TODO: panic message open");

        driver.i2c_handler = Some(i2c);
        return driver;
    }

    pub fn write_i2c(self, data: *[u8],addr_cn : I2cAddr){
        let mut i2c_addr;
        match addr_cn {
            I2cAddr::AddrXE => {
                i2c_addr = ADDR_X_E + 1;
            }
            I2cAddr::AddrYE => {
                i2c_addr = ADDR_Y_E + 1;
            }

            I2cAddr::AddrZE => {
                i2c_addr = ADDR_Z_E + 1;
            }

            I2cAddr::AddrTE => {
                i2c_addr = ADDR_T_E + 1;
            }
            I2cAddr::AddrXR => {
                i2c_addr = ADDR_X_R + 1;
            }
            I2cAddr::AddrYR => {
                i2c_addr = ADDR_Y_R + 1;
            }

            I2cAddr::AddrZR => {
                i2c_addr = ADDR_Z_R + 1;
            }

            I2cAddr::AddrTR => {
                i2c_addr = ADDR_T_R + 1;
            }
        }
        write!(self.i2c_handler.expect("J'arrive pas a récup le filehandler"),
               "{}", (i2c_addr+1) as char
        ).expect("J'arrive pas a écrire dans l'i2c");
        for n in data{
            write!(self.i2c_handler.expect("J'arrive pas a récup le filehandler"),
                   "{}", (n) as char
            ).expect("J'arrive pas a écrire dans l'i2c");
        }
    }
}