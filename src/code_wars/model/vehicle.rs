use super::circular_unit::CircularUnit;
use super::unit::Unit;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum VehicleType {
    Unknown = -1,
    None = 0,
    Arrv = 1,
    Fighter = 2,
    Helicopter = 3,
    Ifv = 4,
    Tank = 5,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vehicle {
    pub id: i64,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub player_id: i64,
    pub durability: i32,
    pub max_durability: i32,
    pub max_speed: f64,
    pub vision_range: f64,
    pub squared_vision_range: f64,
    pub ground_attack_range: f64,
    pub squared_ground_attack_range: f64,
    pub aerial_attack_range: f64,
    pub squared_aerial_attack_range: f64,
    pub ground_damage: i32,
    pub aerial_damage: i32,
    pub ground_defence: i32,
    pub aerial_defence: i32,
    pub attack_cooldown_ticks: i32,
    pub remaining_attack_cooldown_ticks: i32,
    pub type_: VehicleType,
    pub aerial: bool,
    pub selected: bool,
    pub groups: Vec<i32>,
}

unit_impl!(Vehicle);
circular_unit_impl!(Vehicle);
