use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::definitions::{DriverType,Doors};

#[derive(serde::Deserialize, serde::Serialize,Copy, Clone,Debug)]
pub enum HardwareError {
    NotPowered,
    NotStarted,
    ArrMom,
    ArrUrg,
    OpenDoor(Doors),
    MovmentNotFinished(DriverType),
    I2cCreation,
    I2cSetSlave(u16,DriverType),
    I2cRead(DriverType,u8),
    I2cWrite(DriverType,u8),
    BadI2cResponse(DriverType,u8,u8),
    PinExport(u8),
    PinDirection(u8),
    PinWrite(u8),
    PinRead(u8),
}

impl Display for HardwareError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HardwareError::NotPowered => "Bassin pas allumé".to_string(),
            HardwareError::NotStarted => "Bassin pas démarré".to_string(),
            HardwareError::OpenDoor(d) => format!("{} ouverte",d),
            HardwareError::ArrMom => "Les drivers sont en arrêt momentané".to_string(),
            HardwareError::ArrUrg => "Le bassin est en arret d'urgence".to_string(),
            HardwareError::MovmentNotFinished(dt) => format!("Mouvement du {} non fini",dt),
            HardwareError::I2cCreation => "Echec de la création de l'I2C".to_string(),
            HardwareError::I2cSetSlave(addr,driver) => format!("Echec du setting de l'adresse I2C a {} pour le {}",addr,driver),
            HardwareError::I2cRead(dt,register) => format!("Lecture de l'adresse {:#04x} l'I2C du {} a l'adresse ",register,dt),
            HardwareError::I2cWrite(dt,data) => format!("Écriture sur l'I2C du {} de la donnée {:#04x} raté",dt,data),
            HardwareError::BadI2cResponse(dt,data_in,data_out) => format!("Le {} du bassin a lue la donnée {:#04x} au lieu de {:#04x}",dt,data_in,data_out),
            HardwareError::PinExport(p) => format!("GPIO_{} non exporté",p),
            HardwareError::PinDirection(p) => format!("Direction du GPIO_{}",p),
            HardwareError::PinWrite(p) => format!("Problème d'écriture du GPIO_{}",p),
            HardwareError::PinRead(p) => format!("Problème de lecture du GPIO_{}",p),
        };
        write!(f,"{}",s)
    }
}

impl Error for HardwareError {}