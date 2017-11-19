#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum TerrainType {
    Plain = 0,
    Swamp = 1,
    Forest = 2,
}

#[allow(dead_code)]
impl TerrainType {
    pub const COUNT: usize = 3;
}
