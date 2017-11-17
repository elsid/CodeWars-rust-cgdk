use super::action_type::ActionType;
use super::vehicle_type::VehicleType;

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub action: ActionType,
    pub group: i32,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub factor: f64,
    pub max_speed: f64,
    pub max_angular_speed: f64,
    pub vehicle_type: VehicleType,
    pub facility_id: i64,
    pub vehicle_id: i64,
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
}
