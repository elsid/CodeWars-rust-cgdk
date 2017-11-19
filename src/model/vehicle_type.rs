#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum VehicleType {
    Arrv = 0,
    Fighter = 1,
    Helicopter = 2,
    Ifv = 3,
    Tank = 4,
}

#[allow(dead_code)]
impl VehicleType {
    pub const COUNT: usize = 5;
}
