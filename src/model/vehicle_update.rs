#[derive(Clone, Debug, PartialEq)]
pub struct VehicleUpdate {
    pub id: i64,
    pub x: f64,
    pub y: f64,
    pub durability: i32,
    pub remaining_attack_cooldown_ticks: i32,
    pub selected: bool,
    pub groups: Vec<i32>,
}
