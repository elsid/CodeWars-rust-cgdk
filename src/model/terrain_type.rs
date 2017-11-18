#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum TerrainType {
    Unknown = -1,
    Plain = 0,
    Swamp = 1,
    Forest = 2,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType::Unknown
    }
}
