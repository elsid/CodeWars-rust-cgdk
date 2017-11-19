#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum FacilityType {
    Unknown = -1,
    ControlCenter = 0,
    VehicleFactory = 1,
}
