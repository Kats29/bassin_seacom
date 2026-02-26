//use std::fs::File;
use std::io::Write;

use chrono::Local;
//use i2c_linux::I2c;
use i2cdev::core::I2CDevice; // Needed for smbus methods
use linux_embedded_hal::I2cdev;
use sysfs_gpio::{Direction, Pin};

use common::{definitions::DriverType, error::HardwareError};

use crate::tcp_socket::{STREAM_LOG_ERRORS, STREAM_LOG_IO, STREAM_LOG_TCP};

pub fn pin_read(pin: Pin) -> Result<u8, HardwareError> {
    match pin.get_value() {
        Ok(v) => {
            // write_io_log(format!("Signal {} read from GPIO_{}",v,  pin.get_pin()));
            Ok(v)
        }
        Err(_) => {
            write_error_log(format!(
                "Impossible to read the signal from GPIO_{}",
                pin.get_pin()
            ));
            Err(HardwareError::PinRead(pin.get_pin() as u8))
        }
    }
}

pub fn pin_write(pin: Pin, value: u8) -> Result<(), HardwareError> {
    match pin.set_value(value) {
        Ok(_) => {
            write_io_log(format!("GPIO_{} set to {}", pin.get_pin(), value));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!(
                "Impossible to set the GPIO_{} to {}",
                pin.get_pin(),
                value
            ));
            Err(HardwareError::PinWrite(pin.get_pin() as u8))
        }
    }
}

pub fn pin_export(pin: Pin) -> Result<(), HardwareError> {
    match pin.export() {
        Ok(_) => {
            write_io_log(format!("GPIO_{} exported", pin.get_pin()));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!("Could not export the GPIO_{}", pin.get_pin()));
            Err(HardwareError::PinExport(pin.get_pin() as u8))
        }
    }
}

pub fn pin_direction(pin: Pin, value: Direction) -> Result<(), HardwareError> {
    match pin.set_direction(value) {
        Ok(_) => {
            write_io_log(format!(
                "GPIO_{} direction set to {}",
                pin.get_pin(),
                match value {
                    Direction::In => "In",
                    _ => "Out",
                }
            ));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!(
                "Could not set the GPIO_{} direction to {}",
                pin.get_pin(),
                match value {
                    Direction::In => "In",
                    _ => "Out",
                }
            ));
            Err(HardwareError::PinDirection(pin.get_pin() as u8))
        }
    }
}

pub fn pin_set_active_low(pin: Pin, activ_low: bool) -> Result<(), HardwareError> {
    match pin.set_active_low(activ_low) {
        Ok(_) => {
            write_io_log(format!("GPIO_{} is set active at low", pin.get_pin()));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!(
                "Unable to set GPIO_{} active at low",
                pin.get_pin()
            ));
            Err(HardwareError::UnknownError("".to_string()))
        }
    }
}

pub fn i2c_creation(file_path: String) -> Result<I2cdev, HardwareError> {
    match I2cdev::new(file_path.as_str()) {
        Ok(a) => {
            write_io_log(format!("Creation of the i2c at {}", file_path));
            Ok(a)
        }
        Err(_) => {
            write_error_log(format!("Could not create the i2c at {}", file_path));
            Err(HardwareError::I2cCreation)
        }
    }
}

/// Set the slave address for subsequent I2C operations
pub fn i2c_set_slave(
    __i2c__: &mut I2cdev,
    i2c_addr: u16,
    driver_type: DriverType,
) -> Result<(), HardwareError> {
    match __i2c__.set_slave_address(i2c_addr) {
        Ok(_) => {
            write_io_log(format!(
                "Slave address of the {} i2c set to {:#04x}",
                driver_type, i2c_addr
            ));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!(
                "Could not set the slave address of the {} i2c to {:#04x}",
                driver_type, i2c_addr
            ));
            Err(HardwareError::I2cSetSlave(i2c_addr, driver_type))
        }
    }
}

/// Write a byte to a specific register/command of the I2C device
pub fn i2c_write(
    __i2c__: &mut I2cdev,
    command: u8,
    data: u8,
    driver_type: DriverType,
) -> Result<(), HardwareError> {
    match __i2c__.smbus_write_byte_data(command, data) {
        Ok(_) => {
            write_io_log(format!(
                "Data({:#04x}) write to address({:#04x}) of the {} i2c",
                data, command, driver_type
            ));
            Ok(())
        }
        Err(_) => {
            write_error_log(format!(
                "Could not write data({:#04x}) to the address({:#04x}) of the {} i2c",
                data, command, driver_type
            ));
            Err(HardwareError::I2cWrite(driver_type, data, command))
        }
    }
}

/// Read a byte from a specific register/command of the I2C device
pub fn i2c_read(
    __i2c__: &mut I2cdev,
    command: u8,
    driver_type: DriverType,
) -> Result<u8, HardwareError> {
    match __i2c__.smbus_read_byte_data(command) {
        Ok(data) => {
            write_io_log(format!(
                "Data({:#04x}) read from the address({:#04x}) of the {} i2c",
                data, command, driver_type
            ));
            Ok(data)
        }
        Err(_) => {
            write_error_log(format!(
                "Could not read dat from the address({:#04x}) of the {} i2c",
                command, driver_type
            ));
            Err(HardwareError::I2cRead(driver_type, command))
        }
    }
}

pub fn write_error_log(string: String) {
    write!(
        STREAM_LOG_ERRORS
            .lock()
            .unwrap()
            .borrow_mut()
            .as_mut()
            .unwrap(),
        "{:?} : {}\n",
        Local::now().to_rfc2822(),
        string
    )
    .expect("Write in error.log impossible");
}

pub fn write_io_log(string: String) {
    match write!(
        STREAM_LOG_IO.lock().unwrap().borrow_mut().as_mut().unwrap(),
        "{:?} : {}\n",
        Local::now().to_rfc2822(),
        string
    ) {
        Ok(_) => {}
        Err(_) => {
            write_error_log("Impossible to write in io.log".to_string());
        }
    };
}

pub fn write_tcp_log(string: String) {
    match write!(
        STREAM_LOG_TCP
            .lock()
            .unwrap()
            .borrow_mut()
            .as_mut()
            .unwrap(),
        "{:?} : {}\n",
        Local::now().to_rfc2822(),
        string
    ) {
        Ok(_) => {}
        Err(_) => {
            write_error_log("Impossible to write in tcp.log".to_string());
        }
    };
}
