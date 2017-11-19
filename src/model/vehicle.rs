use super::circular_unit::CircularUnit;
use super::unit::Unit;
use super::vehicle_type::VehicleType;
use super::vehicle_update::VehicleUpdate;

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
    pub kind: Option<VehicleType>,
    pub aerial: bool,
    pub selected: bool,
    pub groups: Vec<i32>,
}

#[allow(dead_code)]
impl Vehicle {
    pub fn with_update(old: &Vehicle, update: &VehicleUpdate) -> Vehicle {
        Vehicle {
            id: old.id,
            x: update.x,
            y: update.y,
            radius: old.radius,
            player_id: old.player_id,
            durability: update.durability,
            max_durability: old.max_durability,
            max_speed: old.max_speed,
            vision_range: old.vision_range,
            squared_vision_range: old.squared_vision_range,
            ground_attack_range: old.ground_attack_range,
            squared_ground_attack_range: old.squared_ground_attack_range,
            aerial_attack_range: old.aerial_attack_range,
            squared_aerial_attack_range: old.squared_aerial_attack_range,
            ground_damage: old.ground_damage,
            aerial_damage: old.aerial_damage,
            ground_defence: old.ground_defence,
            aerial_defence: old.aerial_defence,
            attack_cooldown_ticks: old.attack_cooldown_ticks,
            remaining_attack_cooldown_ticks: update.remaining_attack_cooldown_ticks,
            kind: old.kind,
            aerial: old.aerial,
            selected: old.selected,
            groups: update.groups.clone(),
        }
    }

    pub fn update(&mut self, value: &VehicleUpdate) {
        self.x = value.x;
        self.y = value.y;
        self.durability = value.durability;
        self.remaining_attack_cooldown_ticks = value.remaining_attack_cooldown_ticks;
        self.selected = value.selected;
        self.groups = value.groups.clone();
    }
}

unit_impl!(Vehicle);
circular_unit_impl!(Vehicle);
