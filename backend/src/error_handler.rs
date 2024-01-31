use std::fs::{File, OpenOptions};
use sysfs_gpio::{Direction, Pin};
use common::error::{DriverType, HardwareError};
use i2c_linux::I2c;

pub fn handle_pin_read_error(pin : Pin) -> Result<u8,HardwareError>{
    match pin.get_value() {
        Ok(v) => Ok(v),
        Err(_) => Err(HardwareError::PinRead(pin.get_pin() as u8))
    }
}

pub fn handle_pin_write_error(pin : Pin,value : u8) -> Result<(),HardwareError>{
    match pin.set_value(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(HardwareError::PinWrite(pin.get_pin() as u8))
    }
}

pub fn handle_pin_export_error(pin : Pin) -> Result<(),HardwareError>{
    match pin.export() {
        Ok(_) => Ok(()),
        Err(_) => Err(HardwareError::PinExport(pin.get_pin() as u8))
    }
}

pub fn handle_pin_direction_error(pin : Pin,value : Direction) -> Result<(),HardwareError>{
    match pin.set_direction(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(HardwareError::PinDirection(pin.get_pin() as u8))
    }
}

pub fn handle_i2c_creation_error(file_path : String) -> Result<I2c<File>, HardwareError> {
    let i2c_file = match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(a) => Ok(a),
        Err(_) => Err(HardwareError::I2cCreation)
    }?;
    Ok(I2c::new(i2c_file))
}

pub fn handle_i2c_set_slave_error(mut i2c: I2c<File>, i2c_addr : u16,driver: DriverType) -> Result<(),HardwareError>{
    match i2c.smbus_set_slave_address(i2c_addr,false){
        Ok(_) => Ok(()),
        Err(_) => Err(HardwareError::I2cSetSlave(i2c_addr,driver))
    }
}

pub fn handle_i2c_write_error(mut i2c: I2c<File>,data: u8){
}