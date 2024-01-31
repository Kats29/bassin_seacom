use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Doors{
    GaucheBas,
    GaucheHaut,
    DroiteBas,
    DroiteHaut
}

impl Display for Doors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Doors::GaucheBas => "Porte Gauche Bas",
            Doors::GaucheHaut => "Porte Gauche Haut",
            Doors::DroiteBas => "Porte Droite Bas",
            Doors::DroiteHaut => "Porte Droite Haut",
        };
        write!(f,"{}",s)
    }
}

#[derive(Debug,Copy, Clone)]
pub enum DriverType {
    EX,
    EY,
    EZ,
    ETHETA,
    RX,
    RY,
    RZ,
    RTHETA,
}
impl Display for DriverType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DriverType::EX => "Driver pour X émetteur",
            DriverType::EY => "Driver pour Y émetteur",
            DriverType::EZ => "Driver pour Z émetteur",
            DriverType::ETHETA => "Driver pour Théta émetteur",
            DriverType::RX => "Driver pour X récepteur",
            DriverType::RY => "Driver pour Y récepteur",
            DriverType::RZ => "Driver pour Z récepteur",
            DriverType::RTHETA => "Driver pour Théta récepteur",
        };
        write!(f,"{}",s)
    }
}

#[derive(Debug)]
pub enum HardwareError {
    MovmentNotFinished(DriverType),
    OpenDoor(Doors),
    I2cCreation,
    I2cSetSlave(u16,DriverType),
    I2cRead(DriverType),
    I2cWrite(DriverType,u8),
    PinExport(u8),
    PinDirection(u8),
    PinWrite(u8),
    PinRead(u8),
    NotPowered,
}

impl Display for HardwareError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HardwareError::MovmentNotFinished(dt) => format!("Mouvement du {} non fini",dt),
            HardwareError::OpenDoor(d) => format!("{} ouverte",d),
            HardwareError::I2cCreation => "Echec de la création de l'I2C".to_string(),
            HardwareError::I2cSetSlave(addr,driver) => format!("Echec du setting de l'adresse I2C a {} pour le {}",addr,driver),
            HardwareError::I2cRead(dt) => format!("Lecture de l'I2C du {} raté",dt),
            HardwareError::I2cWrite(dt,data) => format!("Écriture sur l'I2C du {} de la donnée {:#04x} raté",dt,data),
            HardwareError::PinExport(p) => format!("GPIO_{} non exporté",p),
            HardwareError::PinDirection(p) => format!("Direction du GPIO_{}",p),
            HardwareError::PinWrite(p) => format!("Problème d'écriture du GPIO_{}",p),
            HardwareError::PinRead(p) => format!("Problème de lecture du GPIO_{}",p),
            HardwareError::NotPowered => "Bassin pas allumé".to_string()
        };
        write!(f,"{}",s)
    }
}

impl Error for HardwareError {}