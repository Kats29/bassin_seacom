use std::cell::RefCell;
use std::sync::Mutex;

use async_recursion::async_recursion;
use futures::executor::block_on;
use sysfs_gpio::{Direction, Pin};

use common::{
    definitions::{
        Arm,
        Command,
        Doors,
        DriverType,
        Status,
    },
    error::HardwareError,
};

use crate::driver_cn_pin::DriverCnPin;
use crate::drivers_cn_rs232::DriversCnRs232;
use crate::error_handler::{handle_pin_direction_error, handle_pin_export_error, handle_pin_read_error, handle_pin_write_error, write_error_log, write_io_log};

pub static ERR_LIST: Mutex<RefCell<Vec<Result<(), HardwareError>>>> = Mutex::new(RefCell::new(vec![]));

pub struct ArmsBackend {
    driver_x_emetteur: DriverCnPin,
    driver_y_emetteur: DriverCnPin,
    driver_z_emetteur: DriverCnPin,
    driver_t_emetteur: DriverCnPin,

    driver_x_recepteur: DriverCnPin,
    driver_y_recepteur: DriverCnPin,
    driver_z_recepteur: DriverCnPin,
    driver_t_recepteur: DriverCnPin,

    driver_rs232: DriversCnRs232,

    pin_ar_mom: Pin,
    pin_on: Pin,
    pin_ordre_ar_urg: Pin,

    pin_info_etat: Pin,
    pin_info_ar_urg: Pin,
    pin_porte_gauche_bas: Pin,
    pin_porte_gauche_haut: Pin,
    pin_porte_droite_bas: Pin,
    pin_porte_droite_haut: Pin,
}

impl Default for ArmsBackend {
    fn default() -> Self {
        Self {
            driver_x_emetteur: DriverCnPin::default(),
            driver_y_emetteur: DriverCnPin::default(),
            driver_z_emetteur: DriverCnPin::default(),
            driver_t_emetteur: DriverCnPin::default(),

            driver_x_recepteur: DriverCnPin::default(),
            driver_y_recepteur: DriverCnPin::default(),
            driver_z_recepteur: DriverCnPin::default(),
            driver_t_recepteur: DriverCnPin::default(),

            driver_rs232: DriversCnRs232::default(),

            pin_ar_mom: Pin::new(0),
            pin_on: Pin::new(0),
            pin_ordre_ar_urg: Pin::new(0),
            pin_info_etat: Pin::new(0),
            pin_info_ar_urg: Pin::new(0),
            pin_porte_gauche_bas: Pin::new(0),
            pin_porte_gauche_haut: Pin::new(0),
            pin_porte_droite_bas: Pin::new(0),
            pin_porte_droite_haut: Pin::new(0),
        }
    }
}

impl ArmsBackend {
    pub fn new() -> Result<Self, HardwareError> {
        let mut arms = Self::default();

        arms.driver_x_emetteur = DriverCnPin::new(DriverType::EX)?;
        arms.driver_y_emetteur = DriverCnPin::new(DriverType::EY)?;
        arms.driver_z_emetteur = DriverCnPin::new(DriverType::EZ)?;
        arms.driver_t_emetteur = DriverCnPin::new(DriverType::ETHETA)?;

        arms.driver_x_recepteur = DriverCnPin::new(DriverType::RX)?;
        arms.driver_y_recepteur = DriverCnPin::new(DriverType::RY)?;
        arms.driver_z_recepteur = DriverCnPin::new(DriverType::RZ)?;
        arms.driver_t_recepteur = DriverCnPin::new(DriverType::RTHETA)?;

        arms.driver_rs232 = DriversCnRs232::new()?;

        arms.global_pin_creation();

        arms.global_pin_export()?;

        arms.global_pin_direction()?;

        match arms.pin_on.set_active_low(true) {
            Ok(_) => {
                write_io_log(format!("GPIO_{} is set active at low", arms.pin_on.get_pin()));
                Ok(())
            }
            Err(_) => {
                write_error_log(format!("Unable to set GPIO_{} active at low", arms.pin_on.get_pin()));
                Err(HardwareError::UnknownError)
            }
        }?;

        Ok(arms)
    }

    fn global_pin_creation(&mut self) {
        self.pin_ordre_ar_urg = Pin::new(60);
        self.pin_on = Pin::new(61);
        self.pin_ar_mom = Pin::new(62);
        self.pin_info_etat = Pin::new(63);
        self.pin_info_ar_urg = Pin::new(81);
        self.pin_porte_gauche_bas = Pin::new(86);
        self.pin_porte_gauche_haut = Pin::new(87);
        self.pin_porte_droite_bas = Pin::new(88);
        self.pin_porte_droite_haut = Pin::new(89);
    }

    fn global_pin_export(&mut self) -> Result<(), HardwareError> {
        handle_pin_export_error(self.pin_ar_mom)?;
        handle_pin_export_error(self.pin_on)?;
        handle_pin_export_error(self.pin_ordre_ar_urg)?;
        handle_pin_export_error(self.pin_info_etat)?;
        handle_pin_export_error(self.pin_info_ar_urg)?;

        handle_pin_export_error(self.pin_porte_gauche_bas)?;
        handle_pin_export_error(self.pin_porte_gauche_haut)?;
        handle_pin_export_error(self.pin_porte_droite_bas)?;
        handle_pin_export_error(self.pin_porte_droite_haut)
    }

    fn global_pin_direction(&self) -> Result<(), HardwareError> {
        handle_pin_direction_error(self.pin_on, Direction::High)?;
        handle_pin_direction_error(self.pin_ordre_ar_urg, Direction::Low)?;
        handle_pin_direction_error(self.pin_ar_mom, Direction::Low)?;

        handle_pin_direction_error(self.pin_info_etat, Direction::Low)?;
        handle_pin_direction_error(self.pin_info_etat, Direction::Low)?;
        handle_pin_direction_error(self.pin_info_ar_urg, Direction::In)?;
        handle_pin_direction_error(self.pin_porte_gauche_bas, Direction::In)?;
        handle_pin_direction_error(self.pin_porte_gauche_haut, Direction::In)?;
        handle_pin_direction_error(self.pin_porte_droite_bas, Direction::In)
    }

    pub fn check_status(&self) -> Status {
        let tmp = ERR_LIST.lock().unwrap();
        let mut vec_error = tmp.borrow_mut();
        *vec_error = vec![];
        let status = Status::new(
            match handle_pin_read_error(self.pin_porte_droite_bas) {
                Ok(result) => {
                    if result == 0 {
                        vec_error.push(Err(HardwareError::OpenDoor(Doors::DroiteBas)));
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    true
                }
            } || match handle_pin_read_error(self.pin_porte_droite_haut) {
                Ok(result) => {
                    if result == 0 {
                        vec_error.push(Err(HardwareError::OpenDoor(Doors::DroiteHaut)));
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    true
                }
            },

            match handle_pin_read_error(self.pin_porte_gauche_bas) {
                Ok(result) => {
                    if result == 0 {
                        vec_error.push(Err(HardwareError::OpenDoor(Doors::GaucheBas)));
                        true
                    }
                    else{
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    true
                }
            } ||
            match handle_pin_read_error(self.pin_porte_gauche_haut) {
                Ok(result) => {
                    if result == 0 {
                        vec_error.push(Err(HardwareError::OpenDoor(Doors::GaucheBas)));
                        true
                    }
                    else{
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    true
                }
            }
            ,
            match handle_pin_read_error(self.pin_info_etat) {
                Ok(result) => {
                    if result == 0 {
                        vec_error.push(Err(HardwareError::NotStarted));
                        false
                    } else {
                        true
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match handle_pin_read_error(self.pin_on) {
                Ok(result_2) => {
                    if result_2 == 1 {
                        true
                    } else {
                        vec_error.push(Err(HardwareError::NotPowered));
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            }
            ,
            match handle_pin_read_error(self.pin_info_ar_urg) {
                Ok(result) => {
                    if result == 1 {
                        vec_error.push(Err(HardwareError::ArrUrg));
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match handle_pin_read_error(self.pin_ar_mom) {
                Ok(result) => {
                    if result == 1 {
                        vec_error.push(Err(HardwareError::ArrMom));
                        true
                    } else {
                        false
                    }
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },

            match self.driver_x_emetteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },

            match self.driver_y_emetteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match self.driver_z_emetteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },

            match self.driver_t_emetteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },

            match self.driver_x_recepteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match self.driver_y_recepteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match self.driver_z_recepteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
            match self.driver_t_recepteur.movement_finished() {
                Ok(_) => {
                    true
                }
                Err(e) => {
                    vec_error.push(Err(e));
                    false
                }
            },
        );


        return status;
    }

    fn movement_finished(&self) -> Vec<Result<(), HardwareError>> {
        let mut error = vec![];

        match self.driver_x_emetteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_y_emetteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_z_emetteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_t_emetteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_x_recepteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_y_recepteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_z_recepteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        match self.driver_t_recepteur.movement_finished() {
            Ok(_) => {}
            Err(e) => {
                error.push(Err(e));
            }
        }

        error
    }

    pub fn update(&mut self, command: Command) -> Result<(), HardwareError> {
        self.check_status();
        match command {
            Command::Go(dt, arm_e, arm_r) => {
                self.write_go(dt, arm_e, arm_r)?;
                block_on(self.pin_go(dt))
            }
            Command::Reset(dt) => block_on(self.reset(dt)),
            Command::Zero(dt) => {
                self.zero(dt)
            }
            Command::ArrUrg => self.arr_urg(),
            Command::StopArrUrg => self.arr_urg(),
            Command::ArrMom => self.arr_mom(),
            Command::StopArrMom => self.arr_mom(),
            Command::Start => self.start_bassin(),
            Command::Stop => self.stop_bassin(),
        }
    }

    pub fn write_go(&mut self, driver_type: DriverType, arm_e: Arm, arm_r: Arm) -> Result<(), HardwareError> {
        match driver_type {
            DriverType::EX => self.driver_rs232.write_i2c(arm_e.next().x_to_bytes(), driver_type),
            DriverType::EY => self.driver_rs232.write_i2c(arm_e.next().y_to_bytes(), driver_type),
            DriverType::EZ => self.driver_rs232.write_i2c(arm_e.next().z_to_bytes(), driver_type),
            DriverType::ETHETA => self.driver_rs232.write_i2c(arm_e.next().theta_to_bytes(), driver_type),
            DriverType::RX => self.driver_rs232.write_i2c(arm_r.next().x_to_bytes(), driver_type),
            DriverType::RY => self.driver_rs232.write_i2c(arm_r.next().y_to_bytes(), driver_type),
            DriverType::RZ => self.driver_rs232.write_i2c(arm_r.next().z_to_bytes(), driver_type),
            DriverType::RTHETA => self.driver_rs232.write_i2c(arm_r.next().theta_to_bytes(), driver_type),
            DriverType::R => {
                self.write_go(DriverType::RX, Arm::default(), arm_r)?;
                self.write_go(DriverType::RY, Arm::default(), arm_r)?;
                self.write_go(DriverType::RZ, Arm::default(), arm_r)?;
                self.write_go(DriverType::RTHETA, Arm::default(), arm_r)?;
                Ok(())
            }
            DriverType::E => {
                self.write_go(DriverType::EX, arm_e, Arm::default())?;
                self.write_go(DriverType::EY, arm_e, Arm::default())?;
                self.write_go(DriverType::EZ, arm_e, Arm::default())?;
                self.write_go(DriverType::ETHETA, arm_e, Arm::default())?;
                Ok(())
            }
            DriverType::ALL => {
                self.clone().write_go(DriverType::E, arm_e, Arm::default())?;
                self.clone().write_go(DriverType::R, Arm::default(), arm_r)?;
                Ok(())
            }
        }
        /*
        // A changer pour plus de synchro
        self.driver_x_emetteur.go()?;
        self.driver_y_emetteur.go()?;
        self.driver_z_emetteur.go()?;
        self.driver_t_emetteur.go()?;

        self.driver_x_recepteur.go()?;
        self.driver_y_recepteur.go()?;
        self.driver_z_recepteur.go()?;
        self.driver_t_recepteur.go()?;

        Ok(())*/
    }

    #[async_recursion]
    pub async fn pin_go(&self, driver_type: DriverType) -> Result<(), HardwareError> {
        match driver_type {
            DriverType::EX => self.driver_x_emetteur.go(),
            DriverType::EY => self.driver_y_emetteur.go(),
            DriverType::EZ => self.driver_z_emetteur.go(),
            DriverType::ETHETA => self.driver_t_emetteur.go(),
            DriverType::RX => self.driver_x_recepteur.go(),
            DriverType::RY => self.driver_y_recepteur.go(),
            DriverType::RZ => self.driver_z_recepteur.go(),
            DriverType::RTHETA => self.driver_t_recepteur.go(),
            DriverType::R => {
                let x = self.pin_go(DriverType::RX);
                let y = self.pin_go(DriverType::RY);
                let z = self.pin_go(DriverType::RZ);
                let t = self.pin_go(DriverType::RTHETA);
                let a = futures::join!(x,y,z,t);
                let vec = vec![a.0, a.1, a.2, a.3];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }

                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
            DriverType::E => {
                let x = self.pin_go(DriverType::EX);
                let y = self.pin_go(DriverType::EY);
                let z = self.pin_go(DriverType::EZ);
                let t = self.pin_go(DriverType::ETHETA);
                let a = futures::join!(x,y,z,t);
                let vec = vec![a.0, a.1, a.2, a.3];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }

                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
            DriverType::ALL => {
                let r = self.pin_go(DriverType::R);
                let e = self.pin_go(DriverType::E);
                let a = futures::join!(r,e);
                let vec = vec![a.0, a.1];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }
                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
        }
    }

    #[async_recursion]
    pub async fn reset(&self, dt: DriverType) -> Result<(), HardwareError> {
        match dt {
            DriverType::EX => self.driver_x_emetteur.reset(),
            DriverType::EY => self.driver_y_emetteur.reset(),
            DriverType::EZ => self.driver_z_emetteur.reset(),
            DriverType::ETHETA => self.driver_t_emetteur.reset(),
            DriverType::RX => self.driver_x_recepteur.reset(),
            DriverType::RY => self.driver_y_recepteur.reset(),
            DriverType::RZ => self.driver_z_recepteur.reset(),
            DriverType::RTHETA => self.driver_t_recepteur.reset(),
            /*
            DriverType::EY => self.driver_y_emetteur.reset(),
            DriverType::EZ => self.driver_z_emetteur.reset(),
            DriverType::ETHETA => self.driver_t_emetteur.reset(),

            DriverType::RX => self.driver_x_recepteur.reset(),
            DriverType::RY => self.driver_y_recepteur.reset(),
            DriverType::RZ => self.driver_z_recepteur.reset(),
            DriverType::RTHETA => self.driver_t_recepteur.reset(),
            */
            DriverType::R => {
                let x = self.reset(DriverType::RX);
                let y = self.reset(DriverType::RY);
                let z = self.reset(DriverType::RZ);
                let t = self.reset(DriverType::RTHETA);
                let a = futures::join!(x,y,z,t);
                let vec = vec![a.0, a.1, a.2, a.3];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }

                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
            DriverType::E => {
                let x = self.reset(DriverType::EX);
                let y = self.reset(DriverType::EY);
                let z = self.reset(DriverType::EZ);
                let t = self.reset(DriverType::ETHETA);
                let a = futures::join!(x,y,z,t);
                let vec = vec![a.0, a.1, a.2, a.3];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }

                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
            DriverType::ALL => {
                let r = self.reset(DriverType::R);
                let e = self.reset(DriverType::E);
                let a = futures::join!(r,e);
                let vec = vec![a.0, a.1];

                for n in vec {
                    match n {
                        Ok(_) => {}
                        Err(a) => {
                            ERR_LIST.lock().unwrap().borrow_mut().push(Err(a));
                        }
                    }
                }
                if ERR_LIST.lock().unwrap().borrow().is_empty() {
                    return Ok(());
                }
                Err(HardwareError::UnknownError)
            }
        }
    }

    pub fn zero(&self, driver_type: DriverType) -> Result<(), HardwareError> {
        match driver_type {
            DriverType::EX => self.driver_x_emetteur.zero(),
            DriverType::EY => self.driver_y_emetteur.zero(),
            DriverType::EZ => self.driver_z_emetteur.zero(),
            DriverType::ETHETA => self.driver_t_emetteur.zero(),
            DriverType::RX => self.driver_x_recepteur.zero(),
            DriverType::RY => self.driver_y_recepteur.zero(),
            DriverType::RZ => self.driver_z_recepteur.zero(),
            DriverType::RTHETA => self.driver_t_recepteur.zero(),
            DriverType::R => {
                self.zero(DriverType::RX)?;
                self.zero(DriverType::RY)?;
                self.zero(DriverType::RZ)?;
                self.zero(DriverType::RTHETA)
            }
            DriverType::E => {
                self.zero(DriverType::EX)?;
                self.zero(DriverType::EY)?;
                self.zero(DriverType::EZ)?;
                self.zero(DriverType::ETHETA)
            }
            DriverType::ALL => {
                self.zero(DriverType::E)?;
                self.zero(DriverType::R)
            }
        }
    }

    fn arr_urg(&self) -> Result<(), HardwareError> {
        let is_hight = handle_pin_read_error(self.pin_ordre_ar_urg)?;
        handle_pin_write_error(self.pin_ordre_ar_urg, (!is_hight)&1)?;
        Ok(())
    }

    fn arr_mom(&self) -> Result<(), HardwareError> {
        let is_hight = handle_pin_read_error(self.pin_ordre_ar_urg)?;
        handle_pin_write_error(self.pin_ar_mom, (!is_hight)&1)?;
        Ok(())
    }

    fn start_bassin(&self) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_on, 1)
    }

    fn stop_bassin(&self) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_on, 0)
    }
}

impl Clone for ArmsBackend {
    fn clone(&self) -> Self {
        let mut new_arm = ArmsBackend::default();
        new_arm.driver_x_emetteur = self.driver_x_emetteur.clone();
        new_arm.driver_y_emetteur = self.driver_y_emetteur.clone();
        new_arm.driver_z_emetteur = self.driver_z_emetteur.clone();
        new_arm.driver_t_emetteur = self.driver_t_emetteur.clone();

        new_arm.driver_x_recepteur = self.driver_x_recepteur.clone();
        new_arm.driver_y_recepteur = self.driver_y_recepteur.clone();
        new_arm.driver_z_recepteur = self.driver_z_recepteur.clone();
        new_arm.driver_t_recepteur = self.driver_t_emetteur.clone();

        new_arm.pin_on = self.pin_on.clone();
        new_arm.pin_info_etat = self.pin_on.clone();

        new_arm.pin_ordre_ar_urg = self.pin_ordre_ar_urg.clone();

        new_arm.pin_info_etat = self.pin_info_etat.clone();
        new_arm.pin_info_ar_urg = self.pin_info_ar_urg.clone();
        new_arm.pin_porte_gauche_bas = self.pin_porte_gauche_bas.clone();
        new_arm.pin_porte_gauche_haut = self.pin_porte_gauche_haut.clone();
        new_arm.pin_porte_droite_bas = self.pin_porte_droite_bas.clone();
        new_arm.pin_porte_droite_haut = self.pin_porte_droite_haut.clone();

        return new_arm;
    }
}