use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum Doors {
    GaucheBas,
    GaucheHaut,
    DroiteBas,
    DroiteHaut,
}

impl Display for Doors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Doors::GaucheBas => "Porte Gauche Bas",
            Doors::GaucheHaut => "Porte Gauche Haut",
            Doors::DroiteBas => "Porte Droite Bas",
            Doors::DroiteHaut => "Porte Droite Haut",
        };
        write!(f, "{}", s)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, strum::EnumIter)]
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
        write!(f, "{}", s)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum Command {
    Go(DriverType, Position, Position),
    Reset(DriverType),
    Zero(DriverType),
    ArrUrg,
    StopArrUrg,
    ArrMom,
    StopArrMom,
    Start,
    Stop,
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

    pub fn set_pos(&mut self,pos: Self){
        *self = pos;
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

    pub fn x_to_bytes(self) -> [u8; 9] {
        let x = ((-6025.0 * self.x().abs()) as isize + 8539473) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (x >> 16) as u8;
        bytes[5] = (x >> 8) as u8;
        bytes[6] = (x & 0xff) as u8;
        return bytes;
    }

    pub fn y_to_bytes(self) -> [u8; 9] {
        let y = ((-6025.0 * self.y()) as isize + 2984423) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (y >> 16) as u8;
        bytes[5] = (y >> 8) as u8;
        bytes[6] = (y & 0xff) as u8;
        return bytes;
    }

    pub fn z_to_bytes(self) -> [u8; 9] {
        let z = ((6025.0 * self.z) as isize + 2048) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (z >> 16) as u8;
        bytes[5] = (z >> 8) as u8;
        bytes[6] = (z & 0xff) as u8;
        return bytes;
    }

    pub fn theta_to_bytes(self) -> [u8; 9] {
        let theta = ((5000.0 * self.theta / 9.0) as isize + 8388608) as usize;
        let mut bytes: [u8; 9] = [0x08, 0x51, 0x00, 0x01, 0x00, 0x00, 0x00, 0x87, 0xff];
        bytes[4] = (theta >> 16) as u8;
        bytes[5] = (theta >> 8) as u8;
        bytes[6] = (theta & 0xff) as u8;
        return bytes;
    }

    pub fn to_bytes(self) -> [[u8; 9]; 4] {
        return [self.x_to_bytes(), self.y_to_bytes(), self.z_to_bytes(), self.theta_to_bytes()];
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",format!("X : {:.2}, Y : {:.2} Z : {:.2}, θ : {:.2}",self.x(),self.y(),self.z(),self.theta()))
    }
}


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct Arm {
    position: Position,
    list_next: VecDeque<Position>,
    is_emitter: bool,

}

impl Default for Arm {
    fn default() -> Self {
        Self {
            position: Position::default(),
            list_next: VecDeque::new(),
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

    pub fn origin_x(&mut self) {
        self.position.set_x(
            if self.is_emitter() { -1417.0 } else { 1417.0 }
        );
    }


    pub fn origin_y(&mut self) {
        self.position.set_y(495.0);
    }


    pub fn origin_z(&mut self) {
        self.position.set_z(0.0);
    }
    pub fn origin_theta(&mut self) {
        self.position.set_theta(0.0);
    }

    pub fn position(&self) -> Position {
        return self.position;
    }
    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn has_next(&self) -> bool {
        !self.list_next.is_empty()
    }

    pub fn del_list(&mut self){
        self.list_next = VecDeque::new();
    }

    pub fn del_in_list(&mut self,index: usize){
        self.list_next.remove(index);
        return;
    }

    pub fn replace_in_list(&mut self,index:usize,pos: Position){
        self.list_next.get_mut(index).unwrap().set_pos(pos);
    }

    pub fn next(&self) -> Option<Position> {
        if self.has_next() {
            Some(self.list_next[0])
        }
        else {
            None
        }
    }

    pub fn list_next(&self) -> VecDeque<Position>{
        return self.list_next.clone();
    }

    pub fn add_next(&mut self, pos: Position) {
        self.list_next.push_back(pos);
    }

    /// Moves the arm from its current position to the next one
    pub fn move_next(&mut self) {
        if self.has_next(){
            let pos = self.list_next.pop_front().unwrap();
            self.set_position(pos);
        }
    }
    pub fn move_next_x(&mut self) {
        if self.has_next() {
            self.position.set_x(self.list_next[0].x());
        }
    }

    pub fn move_next_y(&mut self) {
        if self.has_next() {
            self.position.set_y(self.list_next[0].y());
        }
    }

    pub fn move_next_z(&mut self) {
        if self.has_next() {
            self.position.set_z(self.list_next[0].z());
        }
    }

    pub fn move_next_theta(&mut self) {
        if self.has_next() {
            self.position.set_theta(self.list_next[0].theta());
        }
    }

    pub fn is_emitter(&self) -> bool {
        return self.is_emitter;
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
#[serde(default)]
pub struct Status {
    door_right_open: bool,
    door_left_open: bool,
    bassin_powered: bool,
    bassin_started: bool,
    arr_urg: bool,
    arr_mom: bool,
    movement_ex: bool,
    movement_ey: bool,
    movement_ez: bool,
    movement_et: bool,
    movement_rx: bool,
    movement_ry: bool,
    movement_rz: bool,
    movement_rt: bool,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            door_right_open: true,
            door_left_open: true,
            bassin_powered: false,
            bassin_started: false,
            arr_urg: false,
            arr_mom: false,
            movement_ex: false,
            movement_ey: false,
            movement_ez: false,
            movement_et: false,
            movement_rx: false,
            movement_ry: false,
            movement_rz: false,
            movement_rt: false,
        }
    }
}


impl Status {
    pub fn new(door_right_open: bool, door_left_open: bool, bassin_powered: bool, bassin_started: bool, arr_urg: bool,
               arr_mom: bool, movement_ex: bool, movement_ey: bool, movement_ez: bool, movement_et: bool,
               movement_rx: bool, movement_ry: bool, movement_rz: bool, movement_rt: bool,
    ) -> Self {
        Self {
            door_right_open,
            door_left_open,
            bassin_powered,
            bassin_started,
            arr_urg,
            arr_mom,
            movement_ex,
            movement_ey,
            movement_ez,
            movement_et,
            movement_rx,
            movement_ry,
            movement_rz,
            movement_rt,
        }
    }

    pub fn ne(self, equal: Self) -> bool {
        if self.door_left_open() != equal.door_left_open() {
            return true;
        }
        if self.door_right_open() != equal.door_right_open() {
            return true;
        }

        if self.bassin_powered() != equal.bassin_powered() {
            return true;
        }
        if self.bassin_started() != equal.bassin_started() {
            return true;
        }
        if self.arr_mom() != equal.arr_mom() {
            return true;
        }
        if self.arr_mom() != equal.arr_mom() {
            return true;
        }


        if self.movement_ex() != equal.movement_ex() {
            return true;
        }
        if self.movement_ey() != equal.movement_ey() {
            return true;
        }
        if self.movement_ez() != equal.movement_ez() {
            return true;
        }
        if self.movement_et() != equal.movement_et() {
            return true;
        }


        if self.movement_rx() != equal.movement_rx() {
            return true;
        }
        if self.movement_ry() != equal.movement_ry() {
            return true;
        }
        if self.movement_rz() != equal.movement_rz() {
            return true;
        }
        if self.movement_rt() != equal.movement_rt() {
            return true;
        }

        return false;
    }

    pub fn door_right_open(self) -> bool {
        self.door_right_open
    }
    pub fn door_left_open(self) -> bool {
        self.door_left_open
    }
    pub fn bassin_powered(self) -> bool {
        self.bassin_powered
    }
    pub fn bassin_started(self) -> bool {
        self.bassin_started
    }
    pub fn arr_urg(self) -> bool {
        self.arr_urg
    }
    pub fn arr_mom(self) -> bool {
        self.arr_mom
    }
    pub fn movement_ex(self) -> bool {
        self.movement_ex
    }
    pub fn movement_ey(self) -> bool {
        self.movement_ey
    }
    pub fn movement_ez(self) -> bool {
        self.movement_ez
    }
    pub fn movement_et(self) -> bool {
        self.movement_et
    }
    pub fn movement_rx(self) -> bool {
        self.movement_rx
    }
    pub fn movement_ry(self) -> bool {
        self.movement_ry
    }
    pub fn movement_rz(self) -> bool {
        self.movement_rz
    }
    pub fn movement_rt(self) -> bool {
        self.movement_rt
    }
}