#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum FacilityType {
    Unknown = -1,
    ControlCenter = 0,
    VehicleFactory = 1,
}

impl Default for FacilityType {
    fn default() -> Self {
        FacilityType::Unknown
    }
}
