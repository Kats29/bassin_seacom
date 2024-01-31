use sysfs_gpio::{Direction, Pin};
use common::definitions::Arm;
use common::error::{HardwareError,DriverType};
use crate::driver_cn_pin::{DriverCnPin};
use crate::drivers_cn_rs232::{DriversCnRs232};


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
    pub fn new() -> Result<Self,HardwareError> {
        let mut arms  = Self::default();

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

        arms.pin_on.set_direction(Direction::Out).expect("Probleme d'export des pins globaux");
        arms.pin_ordre_ar_urg.set_direction(Direction::Out).expect("Probleme d'export des pins globaux");

        arms.pin_info_etat.set_direction(Direction::In).expect("Probleme d'export des pins globaux");
        arms.pin_info_ar_urg.set_direction(Direction::In).expect("Probleme d'export des pins globaux");
        arms.pin_porte_gauche_bas.set_direction(Direction::In).expect("Probleme d'export des pins globaux");
        arms.pin_porte_gauche_haut.set_direction(Direction::In).expect("Probleme d'export des pins globaux");
        arms.pin_porte_droite_bas.set_direction(Direction::In).expect("Probleme d'export des pins globaux");
        arms.pin_porte_droite_haut.set_direction(Direction::In).expect("Probleme d'export des pins globaux");

        //arms.pin_on.set_value(1).expect("Le drivers n'as pas pu être lancé");

        Ok(arms)
    }

    fn global_pin_creation(&mut self){
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
    
    fn global_pin_export(&mut self) -> Result<(),HardwareError>{
        match self.pin_ar_mom.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_ar_mom.get_pin() as u8)),
        }?;
        match self.pin_on.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_on.get_pin() as u8)),
        }?;
        match self.pin_ordre_ar_urg.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_ordre_ar_urg.get_pin() as u8)),
        }?;
        match self.pin_info_etat.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_info_etat.get_pin() as u8)),
        }?;
        match self.pin_info_ar_urg.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_info_ar_urg.get_pin() as u8)),
        }?;

        match self.pin_porte_gauche_bas.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_porte_gauche_bas.get_pin() as u8)),
        }?;
        match self.pin_porte_gauche_haut.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_porte_gauche_haut.get_pin() as u8)),
        }?;
        match self.pin_porte_droite_bas.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_porte_droite_bas.get_pin() as u8)),
        }?;
        match self.pin_porte_droite_haut.export(){
            Ok(_) => Ok(()),
            Err(_) => Err(HardwareError::PinExport(self.pin_porte_droite_haut.get_pin() as u8)),
        }?;
        Ok(())
    }
    
    pub fn update(&self, arm_e: Arm, arm_r: Arm) -> Result<(),HardwareError> {
        let bytes_positions_e = arm_e.position().to_bytes();
        let bytes_positions_r = arm_r.position().to_bytes();

        self.driver_rs232.write_i2c(&bytes_positions_e[0], DriverType::EX)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[1], DriverType::EY)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[2], DriverType::EZ)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[3], DriverType::ETHETA)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[0], DriverType::RX)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[1], DriverType::RY)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[2], DriverType::RZ)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[3], DriverType::RTHETA)?;

        //self.go()?;

        Ok(())
    }

    pub fn go(&self) -> Result<(),HardwareError> {
        // A changer pour plus de synchro
        self.driver_x_emetteur.go()?;
        self.driver_y_emetteur.go()?;
        self.driver_z_emetteur.go()?;
        self.driver_t_emetteur.go()?;

        self.driver_x_recepteur.go()?;
        self.driver_y_recepteur.go()?;
        self.driver_z_recepteur.go()?;
        self.driver_t_recepteur.go()?;

        Ok(())
    }

    pub fn reset(&self) -> Result<(),HardwareError> {
        // a changer comme le go
        self.driver_x_emetteur.reset().expect("Le moteur X emetteur est encore actif");
        self.driver_y_emetteur.reset().expect("Le moteur Y emetteur est encore actif");
        self.driver_z_emetteur.reset().expect("Le moteur Z emetteur est encore actif");
        self.driver_t_emetteur.reset().expect("Le moteur Théta emetteur est encore actif");

        self.driver_x_recepteur.reset().expect("Le moteur X recepteur est encore actif");
        self.driver_y_recepteur.reset().expect("Le moteur Y recepteur est encore actif");
        self.driver_z_recepteur.reset().expect("Le moteur Z recepteur est encore actif");
        self.driver_t_recepteur.reset().expect("Le moteur Théta recepteur est encore actif");
        Ok(())
    }

    pub fn zero(&self) -> Result<(),HardwareError> {
        // a changer comme le go
        self.driver_x_emetteur.zero().expect("Le moteur X emetteur est encore actif");
        self.driver_y_emetteur.zero().expect("Le moteur Y emetteur est encore actif");
        self.driver_z_emetteur.zero().expect("Le moteur Z emetteur est encore actif");
        self.driver_t_emetteur.zero().expect("Le moteur Théta emetteur est encore actif");

        self.driver_x_recepteur.zero().expect("Le moteur X recepteur est encore actif");
        self.driver_y_recepteur.zero().expect("Le moteur Y recepteur est encore actif");
        self.driver_z_recepteur.zero().expect("Le moteur Z recepteur est encore actif");
        self.driver_t_recepteur.zero().expect("Le moteur Théta recepteur est encore actif");
        Ok(())
    }
}
