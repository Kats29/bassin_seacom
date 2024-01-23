use std::{
    thread::sleep,
    time::Duration
};

use sysfs_gpio as gpio;
use gpio::{
    Direction, Pin,
};


pub enum DriverType {
    X,
    Y,
    Z,
    THETA,
}

pub struct DriverCN {
    pin_go: Pin,
    pin_reset: Pin,
    pin_ordre_arr_urg: Pin,
    pin_ar_mom: Pin,
    pin_zero: Pin,
    pin_fin_mvt: Pin,
    pin_info_arr_urg: Pin,
    driver_type: DriverType,
}

impl Default for DriverCN {
    fn default() -> Self {
        Self {
            pin_go: Pin::new(0),
            pin_reset: Pin::new(0),
            pin_ordre_arr_urg: Pin::new(0),
            pin_ar_mom: Pin::new(0),
            pin_zero: Pin::new(0),
            pin_fin_mvt: Pin::new(0),
            pin_info_arr_urg: Pin::new(0),
            driver_type: DriverType::X,
        }
    }
}

impl DriverCN {
    pub fn new(is_emitter: bool, driver_type: DriverType) -> gpio::Result<Self> {
        let mut driver = Self::default();
        driver.driver_type = driver_type;
        let pin_go: u8;
        let pin_reset: u8;
        let pin_ordre_arr_urg: u8;
        let pin_ar_mom: u8;
        let pin_zero: u8;
        let pin_fin_mvt: u8;
        let pin_info_arr_urg: u8;


        if is_emitter {
            match driver.driver_type {
                DriverType::X => {
                    pin_go = 1;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::Y => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::Z => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::THETA => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
            }
        } else {
            match driver.driver_type {
                DriverType::X => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::Y => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::Z => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
                DriverType::THETA => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                }
            }
        }

        driver.pin_go = Pin::new(pin_go as u64);
        driver.pin_reset = Pin::new(pin_reset as u64);
        driver.pin_ordre_arr_urg = Pin::new(pin_ordre_arr_urg as u64);
        driver.pin_ar_mom = Pin::new(pin_ar_mom as u64);
        driver.pin_zero = Pin::new(pin_zero as u64);
        driver.pin_fin_mvt = Pin::new(pin_fin_mvt as u64);
        driver.pin_info_arr_urg = Pin::new(pin_info_arr_urg as u64);

        driver.set_direction()?;

        driver.set_export()?;

        return Ok(driver);
    }

    fn set_direction(&mut self) -> gpio::Result<()> {
        self.pin_go.set_direction(Direction::Out)?;
        self.pin_reset.set_direction(Direction::Out)?;
        self.pin_ordre_arr_urg.set_direction(Direction::Out)?;
        self.pin_ar_mom.set_direction(Direction::Out)?;
        self.pin_zero.set_direction(Direction::Out)?;
        self.pin_fin_mvt.set_direction(Direction::In)?;
        self.pin_info_arr_urg.set_direction(Direction::In)?;

        return Ok(());
    }

    fn set_export(&mut self) -> gpio::Result<()> {
        if !(self.pin_go.is_exported()) {
            self.pin_go.export()?;
        }

        if !(self.pin_reset.is_exported()) {
            self.pin_reset.export()?;
        }

        if !(self.pin_ordre_arr_urg.is_exported()) {
            self.pin_ordre_arr_urg.export()?;
        }

        if !(self.pin_ar_mom.is_exported()) {
            self.pin_ar_mom.export()?;
        }

        if !(self.pin_zero.is_exported()) {
            self.pin_zero.export()?;
        }

        if !(self.pin_fin_mvt.is_exported()) {
            self.pin_fin_mvt.export()?;
        }


        if !(self.pin_info_arr_urg.is_exported()) {
            self.pin_info_arr_urg.export()?;
        }

        return Ok(());
    }

    pub fn go(&mut self) -> gpio::Result<()> {
        let go = self.pin_go.get_value()?;
        let fin_mvt = self.pin_fin_mvt.get_value()?;
        if go == 1 || fin_mvt == 0 {
            return Err(gpio::Error::Unexpected("Mouvement non fini".to_string()));
        }
        self.pin_go.set_value(1)?;
        while self.pin_fin_mvt.get_value().unwrap() == 1 {}
        self.pin_go.set_value(0)?;
        return Ok(());
    }

    pub fn reset(&mut self) -> gpio::Result<()>{
        self.pin_reset.set_value(1)?;
        sleep(Duration::from_millis(300));
        self.pin_reset.set_value(0)?;
        Ok(())
    }

    pub fn zero(&mut self) -> gpio::Result<()>{
        self.pin_zero.set_value(1)?;
        sleep(Duration::from_millis(300));
        self.pin_zero.set_value(0)?;
        Ok(())
    }

    pub fn ar_mom(&mut self) -> gpio::Result<()>{
        self.pin_ar_mom.set_value(1)?;
        Ok(())
    }
}
