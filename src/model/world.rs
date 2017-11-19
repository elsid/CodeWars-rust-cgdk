use super::facility::Facility;
use super::player::Player;
use super::terrain_type::TerrainType;
use super::vehicle::Vehicle;
use super::vehicle_update::VehicleUpdate;
use super::weather_type::WeatherType;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub tick_index: i32,
    pub tick_count: i32,
    pub width: f64,
    pub height: f64,
    pub players: Vec<Player>,
    pub new_vehicles: Vec<Vehicle>,
    pub vehicle_updates: Vec<VehicleUpdate>,
    pub terrain_by_cell_x_y: Vec<Vec<Option<TerrainType>>>,
    pub weather_by_cell_x_y: Vec<Vec<Option<WeatherType>>>,
    pub facilities: Vec<Facility>,
}

#[allow(dead_code)]
impl World {
    pub fn get_my_player(&self) -> Option<&Player> {
        self.players.iter().find(|v| v.me)
    }

    pub fn get_opponent_player(&self) -> Option<&Player> {
        self.players.iter().find(|v| !v.me)
    }
}
