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
    id: i64,
    x: f64,
    y: f64,
    radius: f64,
    player_id: i64,
    durability: i32,
    max_durability: i32,
    max_speed: f64,
    vision_range: f64,
    squared_vision_range: f64,
    ground_attack_range: f64,
    squared_ground_attack_range: f64,
    aerial_attack_range: f64,
    squared_aerial_attack_range: f64,
    ground_damage: i32,
    aerial_damage: i32,
    ground_defence: i32,
    aerial_defence: i32,
    attack_cooldown_ticks: i32,
    remaining_attack_cooldown_ticks: i32,
    type_: VehicleType,
    aerial: bool,
    selected: bool,
    groups: Vec<i32>,
}

impl Vehicle {
    pub fn new() -> Self {
        Vehicle {
            id: 0,
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            player_id: -1,
            durability: -1,
            max_durability: -1,
            max_speed: -1.0,
            vision_range: -1.0,
            squared_vision_range: -1.0,
            ground_attack_range: -1.0,
            squared_ground_attack_range: -1.0,
            aerial_attack_range: -1.0,
            squared_aerial_attack_range: -1.0,
            ground_damage: -1,
            aerial_damage: -1,
            ground_defence: -1,
            aerial_defence: -1,
            attack_cooldown_ticks: -1,
            remaining_attack_cooldown_ticks: -1,
            type_: VehicleType::Unknown,
            aerial: false,
            selected: false,
            groups: vec![],
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&mut self, value: i64) -> &mut Self {
        self.id = value;
        self
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, value: f64) -> &mut Self {
        self.x = value;
        self
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, value: f64) -> &mut Self {
        self.y = value;
        self
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, value: f64) -> &mut Self {
        self.radius = value;
        self
    }

    pub fn player_id(&self) -> i64 {
        self.player_id
    }

    pub fn set_player_id(&mut self, value: i64) -> &mut Self {
        self.player_id = value;
        self
    }

    pub fn durability(&self) -> i32 {
        self.durability
    }

    pub fn set_durability(&mut self, value: i32) -> &mut Self {
        self.durability = value;
        self
    }

    pub fn max_durability(&self) -> i32 {
        self.max_durability
    }

    pub fn set_max_durability(&mut self, value: i32) -> &mut Self {
        self.max_durability = value;
        self
    }

    pub fn max_speed(&self) -> f64 {
        self.max_speed
    }

    pub fn set_max_speed(&mut self, value: f64) -> &mut Self {
        self.max_speed = value;
        self
    }

    pub fn vision_range(&self) -> f64 {
        self.vision_range
    }

    pub fn set_vision_range(&mut self, value: f64) -> &mut Self {
        self.vision_range = value;
        self
    }

    pub fn squared_vision_range(&self) -> f64 {
        self.squared_vision_range
    }

    pub fn set_squared_vision_range(&mut self, value: f64) -> &mut Self {
        self.squared_vision_range = value;
        self
    }

    pub fn ground_attack_range(&self) -> f64 {
        self.ground_attack_range
    }

    pub fn set_ground_attack_range(&mut self, value: f64) -> &mut Self {
        self.ground_attack_range = value;
        self
    }

    pub fn squared_ground_attack_range(&self) -> f64 {
        self.squared_ground_attack_range
    }

    pub fn set_squared_ground_attack_range(&mut self, value: f64) -> &mut Self {
        self.squared_ground_attack_range = value;
        self
    }

    pub fn aerial_attack_range(&self) -> f64 {
        self.aerial_attack_range
    }

    pub fn set_aerial_attack_range(&mut self, value: f64) -> &mut Self {
        self.aerial_attack_range = value;
        self
    }

    pub fn squared_aerial_attack_range(&self) -> f64 {
        self.squared_aerial_attack_range
    }

    pub fn set_squared_aerial_attack_range(&mut self, value: f64) -> &mut Self {
        self.squared_aerial_attack_range = value;
        self
    }

    pub fn ground_damage(&self) -> i32 {
        self.ground_damage
    }

    pub fn set_ground_damage(&mut self, value: i32) -> &mut Self {
        self.ground_damage = value;
        self
    }

    pub fn aerial_damage(&self) -> i32 {
        self.aerial_damage
    }

    pub fn set_aerial_damage(&mut self, value: i32) -> &mut Self {
        self.aerial_damage = value;
        self
    }

    pub fn ground_defence(&self) -> i32 {
        self.ground_defence
    }

    pub fn set_ground_defence(&mut self, value: i32) -> &mut Self {
        self.ground_defence = value;
        self
    }

    pub fn aerial_defence(&self) -> i32 {
        self.aerial_defence
    }

    pub fn set_aerial_defence(&mut self, value: i32) -> &mut Self {
        self.aerial_defence = value;
        self
    }

    pub fn attack_cooldown_ticks(&self) -> i32 {
        self.attack_cooldown_ticks
    }

    pub fn set_attack_cooldown_ticks(&mut self, value: i32) -> &mut Self {
        self.attack_cooldown_ticks = value;
        self
    }

    pub fn remaining_attack_cooldown_ticks(&self) -> i32 {
        self.remaining_attack_cooldown_ticks
    }

    pub fn set_remaining_attack_cooldown_ticks(&mut self, value: i32) -> &mut Self {
        self.remaining_attack_cooldown_ticks = value;
        self
    }

    pub fn type_(&self) -> VehicleType {
        self.type_
    }

    pub fn set_type(&mut self, value: VehicleType) -> &mut Self {
        self.type_ = value;
        self
    }

    pub fn aerial(&self) -> bool {
        self.aerial
    }

    pub fn set_aerial(&mut self, value: bool) -> &mut Self {
        self.aerial = value;
        self
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, value: bool) -> &mut Self {
        self.selected = value;
        self
    }

    pub fn groups(&self) -> &Vec<i32> {
        &self.groups
    }

    pub fn set_groups(&mut self, value: Vec<i32>) -> &mut Self {
        self.groups = value;
        self
    }
}

unit_impl!(Vehicle);
circular_unit_impl!(Vehicle);
