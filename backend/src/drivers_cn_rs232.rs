use std::fs::File;
use i2c_linux::I2c;
use sysfs_gpio::{Direction, Pin};
use strum::IntoEnumIterator;
use sysfs_gpio::Direction::Low;
use common::{
    error::HardwareError,
    definitions::DriverType,
};

use crate::error_handler::{i2c_creation, i2c_read, i2c_set_slave, i2c_write, pin_direction, pin_export, pin_read, pin_write};


const ADDR_X_E: u8 = 0x6C;
const ADDR_Y_E: u8 = 0x61;
const ADDR_Z_E: u8 = 0x64;
const ADDR_T_E: u8 = 0x65;

const ADDR_X_R: u8 = 0x5C;
const ADDR_Y_R: u8 = 0x51;
const ADDR_Z_R: u8 = 0x54;
const ADDR_T_R: u8 = 0x55;

// #[derive(Copy, Clone)]
pub struct DriversCnRs232 {
    i2c_handler: Option<I2c<File>>,
}

impl Default for DriversCnRs232 {
    fn default() -> Self {
        Self {
            i2c_handler: None,
        }
    }
}

/// Drivers permettant la communication I2C au MAX3109 qui sera ensuite convertit en RS232
impl DriversCnRs232 {
    /// Renvoie l'adresse I2C du MAX3109 correspondant au type de drivers choisis
    fn get_i2c_addr_value(i2c_type: DriverType) -> u8 {
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
        };
    }

    /// Renvoie le pin ou l'interruption a lieu en fonction du type de drivers choisis
    fn get_iqr_pin(i2c_type: DriverType) -> Pin {
        match i2c_type {
            DriverType::RX | DriverType::EX => Pin::new(110),
            DriverType::EY | DriverType::RY => Pin::new(111),
            DriverType::EZ | DriverType::RZ => Pin::new(112),
            DriverType::ETHETA | DriverType::RTHETA => Pin::new(113),
            _ => Pin::new(0x00),
        }
    }


    pub fn new() -> Result<Self, HardwareError> {
        let mut driver = Self::default();
        driver.i2c_handler = Some(i2c_creation("/dev/i2c-2".to_string())?);
        let mut pin = Pin::new(110);
        pin_export(pin)?;
        pin_direction(pin, Direction::In)?;

        pin = Pin::new(111);
        pin_export(pin)?;
        pin_direction(pin, Direction::In)?;


        pin = Pin::new(112);
        pin_export(pin)?;
        pin_direction(pin, Direction::In)?;

        pin = Pin::new(113);
        pin_export(pin)?;
        pin_direction(pin, Direction::In)?;
        driver.configuration()?;
        Ok(driver)
    }
    /// Configure l'état de base des MAX3109 afin d'avoir des interruptions lors de la réception de nouvelle donnée.
    /// Retourne une [`HardwareError`] en cas de problème
    fn configuration(&mut self) -> Result<(), HardwareError> {
        let pin = Pin::new(115);
        pin_export(pin)?;
        pin_direction(pin, Low)?;
        let i2c_handler = match self.i2c_handler.as_mut() {
            None => {
                Err(HardwareError::UnknownError("".to_string()))
            }
            Some(a) => {
                Ok(a)
            }
        }?;
        pin_write(pin, 1)?;
        for dt in DriverType::iter() {
            i2c_set_slave(i2c_handler, Self::get_i2c_addr_value(dt) as u16, dt)?;
            match dt {
                DriverType::EX |
                DriverType::EY |
                DriverType::RX |
                DriverType::RY => {
                    i2c_write(i2c_handler, 0x1E, 0x1A, dt)?; // utilise la clock externe
                    i2c_write(i2c_handler, 0x1C, 26u8, dt)?; // utilise la clock externe
                    i2c_write(i2c_handler, 0x1B, 1u8, dt)?; // utilise la clock externe
                    i2c_write(i2c_handler, 0x0A, 0x08, dt)?; // interupt sur remplisage RxFifo
                    i2c_write(i2c_handler, 0x01, 0x40, dt)?;
                    i2c_write(i2c_handler, 0x0B, 0x03, dt)?;
                    i2c_read(i2c_handler, 0x02, dt)?;
                }

                DriverType::EZ |
                DriverType::ETHETA |
                DriverType::RZ |
                DriverType::RTHETA => {}
                _ => {}
            }
        }

        Ok(())
    }


    /// Ecrit sur l'I2C a l'adresse corespodant au driver choisis. Retourne une [`HardwareError`] en cas de problème.
    pub fn write_i2c(&mut self, data: [u8; 9], type_cn: DriverType) -> Result<(), HardwareError> {
        let i2c_addr = Self::get_i2c_addr_value(type_cn);
        let iqr_pin = Self::get_iqr_pin(type_cn);
        let i2c_handler = match self.i2c_handler.as_mut() {
            None => {
                Err(HardwareError::UnknownError("".to_string()))
            }
            Some(a) => {
                Ok(a)
            }
        }?;
        i2c_set_slave(i2c_handler, i2c_addr as u16, type_cn)?;
        i2c_read(i2c_handler, 0x12, type_cn)?;
        i2c_write(i2c_handler, 0x0A, 0xA, type_cn)?;
        i2c_write(i2c_handler, 0x0A, 0x8, type_cn)?;
        i2c_read(i2c_handler, 0x02, type_cn)?;
        for n in data.into_iter() {
            i2c_write(i2c_handler, 0x09, 0x02, type_cn)?; // Desactive le
            // transmetteur
            i2c_write(i2c_handler, 0x00, n, type_cn)?;
            i2c_write(i2c_handler, 0x09, 0x00, type_cn)?; // Reactive le transmetteur

            while pin_read(iqr_pin)? != 0 {}

            i2c_read(i2c_handler, 0x02, type_cn)?;

            let g = i2c_read(i2c_handler, 0x00, type_cn)?;

            if g != n {
                return Err(HardwareError::BadI2cResponse(type_cn, g, n));
            }
        }
        Ok(())
    }

    pub fn read_i2c_origin(&mut self, type_cn: DriverType) -> Result<(), HardwareError> {
        let i2c_addr = Self::get_i2c_addr_value(type_cn);
        let irq_pin = Self::get_iqr_pin(type_cn);
        let i2c_handler = match self.i2c_handler.as_mut() {
            None => {
                Err(HardwareError::UnknownError("".to_string()))
            }
            Some(a) => {
                Ok(a)
            }
        }?;

        i2c_set_slave(i2c_handler, i2c_addr as u16, type_cn)?;
        while i2c_read(i2c_handler, 0x12, type_cn)? == 0 || pin_read(irq_pin)? != 0 {}
        if i2c_read(i2c_handler, 0x00, type_cn)? != 0x00 {
            return Err(HardwareError::UnknownError("Mauvais retour d'origine".to_string()));
        }
        Ok(())
    }
    pub fn x_to_bytes(float: f32) -> [u8; 9] {
        let x = ((-6025.0 * float.abs()) as isize + 8539473) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (x >> 16) as u8;
        bytes[5] = (x >> 8) as u8;
        bytes[6] = (x & 0xff) as u8;
        return bytes;
    }

    pub fn y_to_bytes(float: f32) -> [u8; 9] {
        let y = ((-6025.0 * float) as isize + 2984423) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (y >> 16) as u8;
        bytes[5] = (y >> 8) as u8;
        bytes[6] = (y & 0xff) as u8;
        return bytes;
    }

    pub fn z_to_bytes(float: f32) -> [u8; 9] {
        let z = ((6025.0 * float) as isize + 2048) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (z >> 16) as u8;
        bytes[5] = (z >> 8) as u8;
        bytes[6] = (z & 0xff) as u8;
        return bytes;
    }

    pub fn theta_to_bytes(float: f32) -> [u8; 9] {
        let theta = ((5000.0 * float / 9.0) as isize + 8388608) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (theta >> 16) as u8;
        bytes[5] = (theta >> 8) as u8;
        bytes[6] = (theta & 0xff) as u8;
        return bytes;
    }
}

impl Clone for DriversCnRs232 {
    fn clone(&self) -> Self {
        let mut clone = DriversCnRs232::default();
        clone.i2c_handler = Some(i2c_creation("/dev/i2c-2".to_string()).unwrap());
        return clone;
    }
}
