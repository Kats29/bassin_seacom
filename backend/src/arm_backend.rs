use common::definitions::Arm;
use crate::driver_cn_pin::{DriverCnPin, DriverType};
use crate::drivers_cn_rs232::{DriversCnRs232, I2cAddr};

pub struct ArmsBackend{
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
            driver_x_emetteur: DriverCnPin::new(true,DriverType::X).expect("Erreur lors de la création de l'interface pour la CN X émettrice"),
            driver_y_emetteur: DriverCnPin::new(true,DriverType::Y).expect("Erreur lors de la création de l'interface pour la CN Y émettrice"),
            driver_z_emetteur: DriverCnPin::new(true,DriverType::Z).expect("Erreur lors de la création de l'interface pour la CN Z émettrice"),
            driver_t_emetteur: DriverCnPin::new(true,DriverType::THETA).expect("Erreur lors de la création de l'interface pour la CN Théta émettrice"),

            driver_x_recepteur: DriverCnPin::new(false,DriverType::X).expect("Erreur lors de la création de l'interface pour la CN X réceptrice"),
            driver_y_recepteur: DriverCnPin::new(false,DriverType::Y).expect("Erreur lors de la création de l'interface pour la CN Y réceptrice"),
            driver_z_recepteur: DriverCnPin::new(false,DriverType::Z).expect("Erreur lors de la création de l'interface pour la CN Z réceptrice"),
            driver_t_recepteur: DriverCnPin::new(false,DriverType::THETA).expect("Erreur lors de la création de l'interface pour la CN Théta réceptrice"),

            driver_rs232: DriversCnRs232::new().expect("Erreur lors de la création du Drivers RS232"),

        }
    }
}

impl ArmsBackend{
    pub fn new() -> Self{
        return Self::default();
    }

    pub fn update(&mut self, arm_e: Arm, arm_r:Arm) -> std::io::Result<()>{
        let bytes_positions_e = arm_e.position().to_bytes();
        let bytes_positions_r = arm_r.position().to_bytes();

        self.driver_rs232.write_i2c(&bytes_positions_e[0],I2cAddr::AddrXE)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[1],I2cAddr::AddrYE)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[2],I2cAddr::AddrZE)?;
        self.driver_rs232.write_i2c(&bytes_positions_e[3],I2cAddr::AddrTE)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[0],I2cAddr::AddrXR)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[1],I2cAddr::AddrYR)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[2],I2cAddr::AddrZR)?;
        self.driver_rs232.write_i2c(&bytes_positions_r[3],I2cAddr::AddrTR)?;

        self.go()?;

        Ok(())
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
