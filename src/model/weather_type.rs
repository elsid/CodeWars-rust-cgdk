#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum WeatherType {
    Clear = 0,
    Cloud = 1,
    Rain = 2,
}

#[allow(dead_code)]
impl WeatherType {
    pub const COUNT: usize = 3;
}
