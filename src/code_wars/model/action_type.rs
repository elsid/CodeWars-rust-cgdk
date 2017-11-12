#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ActionType {
    Unknown = -1,
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
