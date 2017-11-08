#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TerrainType {
    Unknown = -1,
    Plain = 0,
    Swamp = 1,
    Forest = 2,
}
