#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ActionType {
    None = 0,
    ClearAndSelect = 1,
    AddToSelection = 2,
    Deselect = 3,
    Assign = 4,
    Dismiss = 5,
    Disband = 6,
    Move = 7,
    Rotate = 8,
    Scale = 9,
    SetupVehicleProduction = 10,
    TacticalNuclearStrike = 11,
}

#[allow(dead_code)]
impl ActionType {
    pub const COUNT: usize = 12;
}
