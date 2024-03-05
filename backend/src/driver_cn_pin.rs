use std::{
    thread::sleep,
    time::Duration
};
use sysfs_gpio as gpio;
use gpio::{
    Direction, Pin,
};
use common::{
    definitions::DriverType,
    error::HardwareError,
};

use crate::error_handler::{
    handle_pin_write_error,
    handle_pin_read_error,
    handle_pin_export_error,
    handle_pin_direction_error,
};

pub struct DriverCnPin {
    pin_go: Pin,
    pin_reset: Pin,
    pin_zero: Pin,
    pin_fin_mvt: Pin,
    driver_type: DriverType,
}

impl Default for DriverCnPin {
    fn default() -> Self {
        Self {
            pin_go: Pin::new(0),
            pin_reset: Pin::new(0),
            pin_zero: Pin::new(0),
            pin_fin_mvt: Pin::new(0),
            driver_type: DriverType::EX,
        }
    }
}

impl DriverCnPin {
    pub fn new(driver_type: DriverType) -> Result<Self,HardwareError> {
        let mut driver = Self::default();
        driver.driver_type = driver_type;
        let pin_go: u8;
        let pin_reset: u8;
        let pin_zero: u8;
        let pin_fin_mvt: u8;

        match driver.driver_type {
            DriverType::EX => {
                pin_go = 44;
                pin_reset = 32;
                pin_zero = 73;
                pin_fin_mvt = 65;
            }
            DriverType::EY => {
                pin_go = 45;
                pin_reset = 33;
                pin_zero = 74;
                pin_fin_mvt = 66;
            }
            DriverType::EZ => {
                pin_go = 46;
                pin_reset = 34;
                pin_zero = 75;
                pin_fin_mvt = 67;
            }
            DriverType::ETHETA => {
                pin_go = 47;
                pin_reset = 35;
                pin_zero = 76;
                pin_fin_mvt = 68;
            }
            DriverType::RX => {
                pin_go = 48;
                pin_reset = 36;
                pin_zero = 77;
                pin_fin_mvt = 69;
            }
            DriverType::RY => {
                pin_go = 49;
                pin_reset = 37;
                pin_zero = 78;
                pin_fin_mvt = 70;
            }
            DriverType::RZ => {
                pin_go = 50;
                pin_reset = 38;
                pin_zero = 79;
                pin_fin_mvt = 71;
            }
            DriverType::RTHETA => {
                pin_go = 51;
                pin_reset = 39;
                pin_zero = 80;
                pin_fin_mvt = 72;
            }
            _ => {
                pin_go = 0;
                pin_reset = 0;
                pin_zero = 0;
                pin_fin_mvt = 0;
            }
        }

        driver.pin_go = Pin::new(pin_go as u64);
        driver.pin_reset = Pin::new(pin_reset as u64);
        driver.pin_zero = Pin::new(pin_zero as u64);
        driver.pin_fin_mvt = Pin::new(pin_fin_mvt as u64);

        driver.set_export()?;

        driver.set_direction()?;

        return Ok(driver);
    }

    fn set_direction(&mut self) -> Result<(),HardwareError> {
        handle_pin_direction_error(self.pin_go,Direction::Out)?;
        handle_pin_direction_error(self.pin_reset,Direction::Out)?;
        handle_pin_direction_error(self.pin_zero,Direction::Out)?;
        handle_pin_direction_error(self.pin_fin_mvt,Direction::In)?;

        return Ok(());
    }

    fn set_export(&self) -> Result<(),HardwareError> {
        handle_pin_export_error(self.pin_go)?;
        handle_pin_export_error(self.pin_reset)?;
        handle_pin_export_error(self.pin_zero)?;
        handle_pin_export_error(self.pin_fin_mvt)?;

        return Ok(());
    }


    fn get_driver_type(& self) -> DriverType {
        return self.driver_type;
    }

    pub fn go(&self) -> Result<(),HardwareError> {
        let go = handle_pin_read_error(self.pin_go)?;


        let fin_mvt = handle_pin_read_error(self.pin_fin_mvt)?;

        if go == 1 || fin_mvt == 0 {
            return Err(HardwareError::MovmentNotFinished(self.get_driver_type()));
        }
        handle_pin_write_error(self.pin_go,1)?;
        sleep(Duration::from_millis(10));

        while handle_pin_read_error(self.pin_fin_mvt)? == 1{}

        handle_pin_write_error(self.pin_go,0)?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(),HardwareError>{
        handle_pin_write_error(self.pin_reset,1)?;
        sleep(Duration::from_millis(1));
        handle_pin_write_error(self.pin_reset,0)?;
        Ok(())
    }

    pub fn zero(&self) -> Result<(),HardwareError>{

        handle_pin_write_error(self.pin_zero,1)?;
        sleep(Duration::from_millis(1));
        handle_pin_write_error(self.pin_zero,0)?;
        Ok(())
    }

    pub fn movement_finished(&self) -> Result<(),HardwareError> {
        let go = handle_pin_read_error(self.pin_go)?;

        let fin_mvt = handle_pin_read_error(self.pin_fin_mvt)?;

        if go == 1 || fin_mvt == 0 {
            return Err(HardwareError::MovmentNotFinished(self.get_driver_type()));
        }
        Ok(())
    }

}

impl Clone for DriverCnPin{
    fn clone(&self) -> Self {
        let mut driver = Self::default();
        driver.driver_type = self.driver_type;
        driver.pin_go = self.pin_go.clone();
        driver.pin_zero = self.pin_zero.clone();
        driver.pin_reset = self.pin_reset.clone();
        driver.pin_fin_mvt = self.pin_fin_mvt.clone();
        return driver;
    }
}
