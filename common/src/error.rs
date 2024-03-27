use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::definitions::{DriverType,Doors};


/// Liste des erreurs possibles au niveau HardWare
#[derive(serde::Deserialize, serde::Serialize, Clone,Debug)]
pub enum HardwareError {
    /// Le bassin n'est pas en tension (si le bassin c'est pas allumé, la tension ne peut pas être lu et sera toujours considérer hors tension)
    NotPowered,
    /// Le bassin n'est pas allumé
    NotStarted,
    /// Le bassin est en arrêt momentané
    ArrMom,
    /// Le bassin est en arrêt d'urgence
    ArrUrg,
    /// La porte [`Doors`] est ouverte
    OpenDoor(Doors),
    /// Le mouvement sur [`DriverType`] n'est pas encore fini, donc un nouveau mouvement ne peut pas avoir lieu
    MovmentNotFinished(DriverType),
    /// L'I2C n'a pas pu être créé
    I2cCreation,
    /// L'I2C n'a pas pu être mis à l'adresse correspondant au [`DriverType`]([`u16`])
    I2cSetSlave(u16,DriverType),
    /// L'I2C n'a pas pu lire à l'adresse [`u8`] sur l'I2C correspondant au [`DriverType`]
    I2cRead(DriverType,u8),
    /// L'I2C n'a pas pu écrire [`u8`] a l'adresse [`u8`] sur l'I2C correspondant au [`DriverType`]
    I2cWrite(DriverType,u8,u8),
    BadI2cResponse(DriverType,u8,u8),
    /// Le GPIO_[`u8`] n'a pas pu être exporté
    PinExport(u8),
    /// Le GPIO_[`u8`] n'a pas pu être mis dans la direction voulut
    PinDirection(u8),
    /// Le GPIO_[`u8`] n'a pas pu être écrit
    PinWrite(u8),
    /// Le GPIO_[`u8`] n'a pas pu être lu
    PinRead(u8),
    /// Une autre erreur très peu recurente et fourre tout
    UnknownError(String),
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
            HardwareError::I2cWrite(dt,data,addr) => format!("Écriture sur l'I2C du {} de la donnée {:#04x} a l'adresse {:#04x} raté",dt,data,addr),
            HardwareError::BadI2cResponse(dt,data_in,data_out) => format!("Le {} du bassin a lue la donnée {:#04x} au lieu de {:#04x}",dt,data_in,data_out),
            HardwareError::PinExport(p) => format!("GPIO_{} non exporté",p),
            HardwareError::PinDirection(p) => format!("Direction du GPIO_{}",p),
            HardwareError::PinWrite(p) => format!("Problème d'écriture du GPIO_{}",p),
            HardwareError::PinRead(p) => format!("Problème de lecture du GPIO_{}",p),
            HardwareError::UnknownError(string) => format!("{}", string)
        };
        write!(f,"{}",s)
    }
}

impl Error for HardwareError {}
