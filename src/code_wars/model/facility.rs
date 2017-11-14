use super::vehicle::VehicleType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum FacilityType {
    Unknown = -1,
    ControlCenter = 0,
    VehicleFactory = 1,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Facility {
    pub id: i64,
    pub type_: FacilityType,
    pub owner_player_id: i64,
    pub left: f64,
    pub top: f64,
    pub capture_points: f64,
    pub vehicle_type: VehicleType,
    pub production_progress: i32,
}
