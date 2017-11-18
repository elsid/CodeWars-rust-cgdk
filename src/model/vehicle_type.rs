#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum VehicleType {
    Unknown = -1,
    Arrv = 0,
    Fighter = 1,
    Helicopter = 2,
    Ifv = 3,
    Tank = 4,
}

impl Default for VehicleType {
    fn default() -> Self {
        VehicleType::Unknown
    }
}
