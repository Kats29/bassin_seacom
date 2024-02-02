use sysfs_gpio::{Direction, Pin};
use common::{
    definitions::{
        Arm,
        Command,
        DriverType,
        Doors,
    },
    error::{
        HardwareError,
    },
};
use crate::driver_cn_pin::DriverCnPin;
use crate::drivers_cn_rs232::DriversCnRs232;
use crate::error_handler::{handle_pin_direction_error, handle_pin_export_error, handle_pin_read_error, handle_pin_write_error};


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


        //arms.pin_on.set_value(1).expect("Le drivers n'as pas pu être lancé");

        Ok(arms)
    }

    fn global_pin_creation(&mut self) {
        self.pin_ar_mom = Pin::new(0);
        self.pin_on = Pin::new(0);
        self.pin_ordre_ar_urg = Pin::new(0);
        self.pin_info_etat = Pin::new(0);
        self.pin_info_ar_urg = Pin::new(0);
        self.pin_porte_gauche_bas = Pin::new(0);
        self.pin_porte_gauche_haut = Pin::new(0);
        self.pin_porte_droite_bas = Pin::new(0);
        self.pin_porte_droite_haut = Pin::new(0);
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

    fn global_pin_direction(&self) -> Result<(),HardwareError>{
        handle_pin_direction_error(self.pin_on,Direction::Out)?;
        handle_pin_direction_error(self.pin_ordre_ar_urg,Direction::Out)?;
        handle_pin_direction_error(self.pin_ar_mom,Direction::Out)?;

        handle_pin_direction_error(self.pin_info_etat,Direction::In)?;
        handle_pin_direction_error(self.pin_info_etat,Direction::In)?;
        handle_pin_direction_error(self.pin_info_ar_urg,Direction::In)?;
        handle_pin_direction_error(self.pin_porte_gauche_bas,Direction::In)?;
        handle_pin_direction_error(self.pin_porte_gauche_haut,Direction::In)?;
        handle_pin_direction_error(self.pin_porte_droite_bas,Direction::In)
    }

    fn check_status(&self) -> Result<(), HardwareError> {
        if handle_pin_read_error(self.pin_ar_mom)? != 0 {
            return Err(HardwareError::ArrMom);
        }

        if handle_pin_read_error(self.pin_info_etat)? != 1 {
            if handle_pin_read_error(self.pin_on)? != 1 {
                return Err(HardwareError::NotStarted);
            }
            return Err(HardwareError::NotPowered);
        }
        if handle_pin_read_error(self.pin_info_ar_urg)? != 0 {
            return Err(HardwareError::ArrUrg);
        }
        if handle_pin_read_error(self.pin_porte_gauche_bas)? != 1 {
            return Err(HardwareError::OpenDoor(Doors::GaucheBas));
        }
        if handle_pin_read_error(self.pin_porte_gauche_haut)? != 1 {
            return Err(HardwareError::OpenDoor(Doors::GaucheHaut));
        }
        if handle_pin_read_error(self.pin_porte_droite_bas)? != 1 {
            return Err(HardwareError::OpenDoor(Doors::DroiteBas));
        }
        if handle_pin_read_error(self.pin_porte_droite_haut)? != 1 {
            return Err(HardwareError::OpenDoor(Doors::DroiteHaut));
        }
        Ok(())
    }

    pub fn update(&self, command: Command) -> Result<(), HardwareError> {
        println!("nouvelle commande");
        //self.check_status()?;
        match command {
            Command::Go(dt, arm_e, arm_r) => {
                println!("Go pour {}",dt);
                self.write_go(dt, arm_e, arm_r)?;
                self.pin_go(dt)
            }
            Command::Reset(dt) => self.reset(dt),
            Command::Zero(dt) => {
                println!("Retour a l'origine");
                self.zero(dt)
            },
            Command::ArrUrg => self.arr_urg(true),
            Command::StopArrUrg => self.arr_urg(false),
            Command::ArrMom => self.arr_mom(true),
            Command::StopArrMom => self.arr_mom(false),
            Command::Start => self.start_bassin(),
            Command::Stop => self.stop_bassin(),
        }
    }

    pub fn write_go(&self, driver_type: DriverType, arm_e: Arm, arm_r: Arm) -> Result<(), HardwareError> {
        match driver_type {
            DriverType::EX => self.driver_rs232.write_i2c(arm_e.position().x_to_bytes(), driver_type),
            DriverType::EY => self.driver_rs232.write_i2c(arm_e.position().y_to_bytes(), driver_type),
            DriverType::EZ => self.driver_rs232.write_i2c(arm_e.position().z_to_bytes(), driver_type),
            DriverType::ETHETA => self.driver_rs232.write_i2c(arm_e.position().theta_to_bytes(), driver_type),
            DriverType::RX => self.driver_rs232.write_i2c(arm_r.position().x_to_bytes(), driver_type),
            DriverType::RY => self.driver_rs232.write_i2c(arm_r.position().y_to_bytes(), driver_type),
            DriverType::RZ => self.driver_rs232.write_i2c(arm_r.position().z_to_bytes(), driver_type),
            DriverType::RTHETA => self.driver_rs232.write_i2c(arm_r.position().theta_to_bytes(), driver_type),
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

    pub fn pin_go(&self, driver_type: DriverType) -> Result<(), HardwareError> {
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
                self.pin_go(DriverType::RX)?;
                self.pin_go(DriverType::RY)?;
                self.pin_go(DriverType::RZ)?;
                self.pin_go(DriverType::RTHETA)
            }
            DriverType::E => {
                self.pin_go(DriverType::EX)?;
                self.pin_go(DriverType::EY)?;
                self.pin_go(DriverType::EZ)?;
                self.pin_go(DriverType::ETHETA)
            }
            DriverType::ALL => {
                self.pin_go(DriverType::E)?;
                self.pin_go(DriverType::R)
            }
        }
    }

    pub fn reset(&self, dt: DriverType) -> Result<(), HardwareError> {
        match dt {
            DriverType::EX => self.driver_x_emetteur.reset(),
            DriverType::EY => self.driver_y_emetteur.reset(),
            DriverType::EZ => self.driver_z_emetteur.reset(),
            DriverType::ETHETA => self.driver_t_emetteur.reset(),
            DriverType::RX => self.driver_x_recepteur.reset(),
            DriverType::RY => self.driver_y_recepteur.reset(),
            DriverType::RZ => self.driver_z_recepteur.reset(),
            DriverType::RTHETA => self.driver_t_recepteur.reset(),
            DriverType::R => {
                self.reset(DriverType::RX)?;
                self.reset(DriverType::RY)?;
                self.reset(DriverType::RZ)?;
                self.reset(DriverType::RTHETA)
            }
            DriverType::E => {
                self.reset(DriverType::EX)?;
                self.reset(DriverType::EY)?;
                self.reset(DriverType::EZ)?;
                self.reset(DriverType::ETHETA)
            }
            DriverType::ALL => {
                self.reset(DriverType::E)?;
                self.reset(DriverType::R)
            }
        }
    }

    pub fn zero(&self,driver_type: DriverType) -> Result<(), HardwareError> {
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

    fn arr_urg(&self, is_hight: bool) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_ordre_ar_urg, if is_hight { 1 } else { 0 })?;
        Ok(())
    }

    fn arr_mom(&self, is_hight: bool) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_ar_mom, if is_hight { 1 } else { 0 })?;
        Ok(())
    }

    fn start_bassin(&self) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_on, 1)
    }

    fn stop_bassin(&self) -> Result<(), HardwareError> {
        handle_pin_write_error(self.pin_on, 0)
    }
}

impl Clone for ArmsBackend{
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