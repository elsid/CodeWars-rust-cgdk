use super::action_type::ActionType;
use super::vehicle::VehicleType;

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    action: ActionType,
    group: i32,
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
    x: f64,
    y: f64,
    angle: f64,
    factor: f64,
    max_speed: f64,
    max_angular_speed: f64,
    vehicle_type: VehicleType,
    facility_id: i64,
    vehicle_id: i64,
}

impl Move {
    pub fn new() -> Self {
        Move {
            action: ActionType::Unknown,
            group: 0,
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            factor: 0.0,
            max_speed: 0.0,
            max_angular_speed: 0.0,
            vehicle_type: VehicleType::Unknown,
            facility_id: -1,
            vehicle_id: -1,
        }
    }

    pub fn action(&self) -> ActionType {
        self.action
    }

    pub fn set_action(&mut self, value: ActionType) -> &mut Self {
        self.action = value;
        self
    }

    pub fn group(&self) -> i32 {
        self.group
    }

    pub fn set_group(&mut self, value: i32) -> &mut Self {
        self.group = value;
        self
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left = value;
        self
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top = value;
        self
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right = value;
        self
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom = value;
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

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, value: f64) -> &mut Self {
        self.angle = value;
        self
    }

    pub fn factor(&self) -> f64 {
        self.factor
    }

    pub fn set_factor(&mut self, value: f64) -> &mut Self {
        self.factor = value;
        self
    }

    pub fn max_speed(&self) -> f64 {
        self.max_speed
    }

    pub fn set_max_speed(&mut self, value: f64) -> &mut Self {
        self.max_speed = value;
        self
    }

    pub fn max_angular_speed(&self) -> f64 {
        self.max_angular_speed
    }

    pub fn set_max_angular_speed(&mut self, value: f64) -> &mut Self {
        self.max_angular_speed = value;
        self
    }

    pub fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    pub fn set_vehicle_type(&mut self, value: VehicleType) -> &mut Self {
        self.vehicle_type = value;
        self
    }

    pub fn facility_id(&self) -> i64 {
        self.facility_id
    }

    pub fn set_facility_id(&mut self, value: i64) -> &mut Self {
        self.facility_id = value;
        self
    }

    pub fn vehicle_id(&self) -> i64 {
        self.vehicle_id
    }

    pub fn set_vehicle_id(&mut self, value: i64) -> &mut Self {
        self.vehicle_id = value;
        self
    }
}
