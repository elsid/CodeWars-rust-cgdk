#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum WeatherType {
    Unknown = -1,
    Clear = 0,
    Cloud = 1,
    Rain = 2,
}
