use libbeaglebone as bb;
use bb::{
    gpio::{
        GPIO,
        PinDirection
    },
    prelude::PinState
};
use std::thread;

pub enum DriverType {
    X,
    Y,
    Z,
    THETA,
}

pub struct DriverCN {
    pin_go: GPIO,
    pin_reset: GPIO,
    pin_ordre_arr_urg: GPIO,
    pin_ar_mom: GPIO,
    pin_zero: GPIO,
    pin_fin_mvt: GPIO,
    pin_info_arr_urg: GPIO,
    driver_type: DriverType,
}

impl Default for DriverCN {
    fn default() -> Self {
        Self {
            pin_go: GPIO::new(0),
            pin_reset: GPIO::new(0),
            pin_ordre_arr_urg: GPIO::new(0),
            pin_ar_mom: GPIO::new(0),
            pin_zero: GPIO::new(0),
            pin_fin_mvt: GPIO::new(0),
            pin_info_arr_urg: GPIO::new(0),
            driver_type: DriverType::X,
        }
    }
}

impl DriverCN {
    pub fn new(is_emitter: bool, driver_type: DriverType) -> bb::errors::Result<Self>{
        let mut driver = Self::default();
        driver.driver_type = driver_type;
        let mut pin_go: u8;
        let mut pin_reset: u8;
        let mut pin_ordre_arr_urg: u8;
        let mut pin_ar_mom: u8;
        let mut pin_zero: u8;
        let mut pin_fin_mvt: u8;
        let mut pin_info_arr_urg: u8;



        if is_emitter {
            match driver.driver_type {
                DriverType::X => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
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
                },
                DriverType::THETA => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
            }
        }
        else {
            match driver.driver_type {
                DriverType::X => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
                DriverType::Y => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
                DriverType::Z => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
                DriverType::THETA => {
                    pin_go = 0;
                    pin_reset = 0;
                    pin_ordre_arr_urg = 0;
                    pin_ar_mom = 0;
                    pin_zero = 0;
                    pin_fin_mvt = 0;
                    pin_info_arr_urg = 0;
                },
            }
        }

        driver.pin_go = GPIO::new(pin_go);
        driver.pin_reset = GPIO::new(pin_reset);
        driver.pin_ordre_arr_urg = GPIO::new(pin_ordre_arr_urg);
        driver.pin_ar_mom = GPIO::new(pin_ar_mom);
        driver.pin_zero = GPIO::new(pin_zero);
        driver.pin_fin_mvt = GPIO::new(pin_fin_mvt);
        driver.pin_info_arr_urg = GPIO::new(pin_info_arr_urg);

        driver.set_direction()?;

        driver.set_export()?;

        return Ok(driver);
    }

    fn set_direction(&mut self) -> bb::errors::Result<()>{
        self.pin_go.set_direction(PinDirection::Out)?;
        self.pin_reset.set_direction(PinDirection::Out)?;
        self.pin_ordre_arr_urg.set_direction(PinDirection::Out)?;
        self.pin_ar_mom.set_direction(PinDirection::Out)?;
        self.pin_zero.set_direction(PinDirection::Out)?;
        self.pin_fin_mvt.set_direction(PinDirection::In)?;
        self.pin_info_arr_urg.set_direction(PinDirection::In)?;

        return Ok(());
    }

    fn set_export(&mut self) -> bb::errors::Result<()>{
        self.pin_go.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_reset.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_ordre_arr_urg.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_ar_mom.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_zero.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_fin_mvt.set_export(bb::enums::DeviceState::Exported)?;
        self.pin_info_arr_urg.set_export(bb::enums::DeviceState::Exported)?;

        return Ok(());
    }

    pub fn go(&mut self) -> bb::errors::Result<()>{
        if self.pin_go.read()? == PinState::High
            || self.pin_fin_mvt.read()? == PinState::Low {
                return Err(bb::errors::Error(
                        bb::errors::ErrorKind::Msg(
                            "Mouvement non fini".to_string()),
                        Default::default()
                ));
        }
        self.pin_go.write(PinState::High)?;
        while self.pin_fin_mvt.read()? == PinState::High{}
        self.pin_go.write(PinState::Low)?;
        return Ok(());
    }

}
