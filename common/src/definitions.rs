use std::fmt::{Display, Formatter};

#[derive(serde::Deserialize, serde::Serialize,Copy, Clone,Debug)]
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

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum DriverType {
    EX,
    EY,
    EZ,
    ETHETA,
    RX,
    RY,
    RZ,
    RTHETA,
    R,
    E,
    ALL,
}
impl Display for DriverType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DriverType::EX => "Driver pour le moteur X émetteur",
            DriverType::EY => "Driver pour le moteur Y émetteur",
            DriverType::EZ => "Driver pour le moteur Z émetteur",
            DriverType::ETHETA => "Driver pour le moteur Théta émetteur",
            DriverType::RX => "Driver pour le moteur X récepteur",
            DriverType::RY => "Driver pour le moteur Y récepteur",
            DriverType::RZ => "Driver pour le moteur Z récepteur",
            DriverType::RTHETA => "Driver pour le moteur Théta récepteur",
            DriverType::E => "Drivers pour les moteur émetteur",
            DriverType::R => "Drivers pour les moteur récepteur",
            DriverType::ALL => "Drivers pour tous les moteurs"
        };
        write!(f,"{}",s)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum Command{
    Go(DriverType,Arm,Arm),
    Reset(DriverType),
    Zero(DriverType),
    ArrUrg,
    StopArrUrg,
    ArrMom,
    StopArrMom,
    Start,
    Stop
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
#[serde(default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
    theta: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            theta: 0.0,
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32, theta: f32) -> Self {
        Self { x, y, z, theta }
    }

    pub fn x(self) -> f32 {
        return self.x;
    }
    pub fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    pub fn y(self) -> f32 {
        return self.y;
    }
    pub fn set_y(&mut self, value: f32) {
        self.y = value;
    }

    pub fn z(self) -> f32 {
        return self.z;
    }
    pub fn set_z(&mut self, value: f32) {
        self.z = value;
    }

    pub fn theta(self) -> f32 {
        return self.theta;
    }
    pub fn set_theta(&mut self, value: f32) {
        self.theta = value;
    }

    pub fn x_to_bytes(self) -> [u8;9]{
        let x = ((-6025.0 * self.x().abs()) as isize + 8539473) as usize;
        let mut bytes: [u8;9] = [0x08,0x51,0x00,0x01,0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (x >> 16) as u8;
        bytes[5] = (x >> 8) as u8;
        bytes[6] = (x & 0xff) as u8;
        return bytes;
    }

    pub fn y_to_bytes(self) -> [u8;9]{
        let y = ((-6025.0 * self.y()) as isize + 2984423) as usize;
        let mut bytes: [u8;9] = [0x08,0x51,0x00,0x01,0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (y >> 16) as u8;
        bytes[5] = (y >> 8) as u8;
        bytes[6] = (y & 0xff) as u8;
        return bytes;
    }

    pub fn z_to_bytes(self) -> [u8;9]{
        let z = ((6025.0 * self.z) as isize + 2048) as usize;
        let mut bytes: [u8;9] = [0x08,0x51,0x00,0x01,0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (z >> 16) as u8;
        bytes[5] = (z >> 8) as u8;
        bytes[6] = (z & 0xff) as u8;
        return bytes;
    }

    pub fn theta_to_bytes(self) -> [u8;9]{
        let theta = ((5000.0 * self.theta / 9.0) as isize + 8388608) as usize;
        let mut bytes: [u8;9] = [0x08,0x51,0x00,0x01,0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (theta >> 16) as u8;
        bytes[5] = (theta >> 8) as u8;
        bytes[6] = (theta & 0xff) as u8;
        return bytes;
    }

    pub fn to_bytes(self) -> [[u8; 9]; 4] {
        return [self.x_to_bytes(),self.y_to_bytes(),self.z_to_bytes(),self.theta_to_bytes()];
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
#[serde(default)]
pub struct Arm {
    position: Position,
    next: Position,
    is_emitter: bool,

}

impl Default for Arm {
    fn default() -> Self {
        Self {
            position: Position::default(),
            next: Position::default(),
            is_emitter: true,
        }
    }
}

impl Arm {
    pub fn new(is_emitter: bool) -> Self {
        let mut arm = Self::default();
        arm.is_emitter = is_emitter;
        arm.origin();
        return arm;
    }
    pub fn origin(&mut self) {
        self.set_position(Position::new(
            if self.is_emitter() { -1417.0 } else { 1417.0 },
            495.0,
            0.0,
            0.0,
        ));
    }

    pub fn position(self) -> Position {
        return self.position;
    }
    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn next(self) -> Position {
        return self.next;
    }
    pub fn set_next(&mut self, pos: Position) {
        self.next = pos;
    }

    /// Moves the arm from its current position to the next one
    pub fn move_next(&mut self) {
        self.position = self.next;
    }

    pub fn is_emitter(self) -> bool {
        return self.is_emitter;
    }
}
