#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum FacilityType {
    ControlCenter = 0,
    VehicleFactory = 1,
}

#[allow(dead_code)]
impl FacilityType {
    pub const COUNT: usize = 2;
}
