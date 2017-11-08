use std::collections::HashMap;
use code_wars::model::{
    Facility,
    Player,
    TerrainType,
    WeatherType,
};

pub struct Cache {
    pub facilities: HashMap<i64, Facility>,
    pub players: HashMap<i64, Player>,
    pub terrain_by_cell_x_y: Vec<Vec<TerrainType>>,
    pub weather_by_cell_x_y: Vec<Vec<WeatherType>>,
}
