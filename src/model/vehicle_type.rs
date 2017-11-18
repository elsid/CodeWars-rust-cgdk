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

impl Default for VehicleType {
    fn default() -> Self {
        VehicleType::Unknown
    }
}
