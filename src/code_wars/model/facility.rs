use super::facility_type::FacilityType;
use super::vehicle_type::VehicleType;

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
