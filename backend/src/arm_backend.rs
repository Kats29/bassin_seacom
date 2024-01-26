use common::definitions::{Arm, Position};
use crate::driver_cn_pin::{DriverCnPin, DriverType};
use  crate::drivers_cn_rs232::DriversCnRs232;

pub struct ArmsBackend{
    bras_emetteur: Arm,
    bras_recepteur: Arm,
    driver_x_emetteur: DriverCnPin,
    driver_y_emetteur: DriverCnPin,
    driver_z_emetteur: DriverCnPin,
    driver_t_emetteur: DriverCnPin,


    driver_x_recepteur: DriverCnPin,
    driver_y_recepteur: DriverCnPin,
    driver_z_recepteur: DriverCnPin,
    driver_t_recepteur: DriverCnPin,

    driver_rs232 : DriversCnRs232,
}

impl Default for ArmsBackend {
    fn default() -> Self {
        Self {
            bras_emetteur: Arm::new(true),
            bras_recepteur: Arm::new(false),

            driver_x_emetteur: DriverCnPin::new(true,DriverType::X).expect("Erreur lors de la création de l'interface pour la CN X émettrice"),
            driver_y_emetteur: DriverCnPin::new(true,DriverType::Y).expect("Erreur lors de la création de l'interface pour la CN Y émettrice"),
            driver_z_emetteur: DriverCnPin::new(true,DriverType::Z).expect("Erreur lors de la création de l'interface pour la CN Z émettrice"),
            driver_t_emetteur: DriverCnPin::new(true,DriverType::THETA).expect("Erreur lors de la création de l'interface pour la CN Théta émettrice"),
            driver_x_recepteur: DriverCnPin::new(false,DriverType::X).expect("Erreur lors de la création de l'interface pour la CN X réceptrice"),
            driver_y_recepteur: DriverCnPin::new(false,DriverType::Y).expect("Erreur lors de la création de l'interface pour la CN Y réceptrice"),
            driver_z_recepteur: DriverCnPin::new(false,DriverType::Z).expect("Erreur lors de la création de l'interface pour la CN Z réceptrice"),
            driver_t_recepteur: DriverCnPin::new(false,DriverType::THETA).expect("Erreur lors de la création de l'interface pour la CN Théta réceptrice"),
        }
    }
}

impl ArmsBackend{
    pub fn new(pos_e: Position, pos_r: Position) -> Self{
        let mut bras = Self::default();
        bras.set_pos_e(pos_e);
        bras.set_pos_r(pos_r);
        return bras;

    }
    fn set_pos_e(&mut self,pos_e: Position){
        self.bras_emetteur.set_position(pos_e);
    }
    fn set_pos_r(&mut self,pos_r: Position){
        self.bras_recepteur.set_position(pos_r);
    }

    pub fn update(&mut self, arm_e: Arm, arm_r:Arm){
        self.set_pos_e(arm_e.position());
        self.set_pos_r(arm_r.position())
    }


    pub fn go(&mut self) -> sysfs_gpio::Result<()>{
        // A changer pour plus de synchro
        self.driver_x_emetteur.go().expect("Le moteur X emetteur est encore actif");
        self.driver_y_emetteur.go().expect("Le moteur Y emetteur est encore actif");
        self.driver_z_emetteur.go().expect("Le moteur Z emetteur est encore actif");
        self.driver_t_emetteur.go().expect("Le moteur Théta emetteur est encore actif");

        self.driver_x_recepteur.go().expect("Le moteur X recepteur est encore actif");
        self.driver_y_recepteur.go().expect("Le moteur Y recepteur est encore actif");
        self.driver_z_recepteur.go().expect("Le moteur Z recepteur est encore actif");
        self.driver_t_recepteur.go().expect("Le moteur Théta recepteur est encore actif");

        Ok(())
    }

    pub fn reset(&mut self) ->  sysfs_gpio::Result<()>{
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

    pub fn zero(&mut self) -> sysfs_gpio::Result<()>{
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
