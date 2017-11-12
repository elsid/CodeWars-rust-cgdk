use std::collections::HashMap;
use std::io;
use byteorder::{ByteOrder, ReadBytesExt};
use code_wars::model::{
    Facility,
    FacilityType,
    Game,
    Player,
    PlayerContext,
    TerrainType,
    Vehicle,
    VehicleType,
    VehicleUpdate,
    WeatherType,
    World,
};
use super::message::Message;
use super::cache::Cache;

pub trait ReadMessage: ReadBytesExt {
    fn read_message<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<Message> {
        use std::io::{Error, ErrorKind};
        match self.read_message_id()? {
            0 => self.read_message_unknown_message(),
            1 => self.read_message_game_over(),
            2 => self.read_message_authentication_token::<B>(),
            3 => self.read_message_team_size::<B>(),
            4 => self.read_message_protocol_version::<B>(),
            5 => self.read_message_game_context::<B>(),
            6 => self.read_message_player_context::<B>(cache),
            7 => self.read_message_moves_message(),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_message error: invalid message id: {}", v)))
        }
    }

    fn read_message_id(&mut self) -> io::Result<i8> {
        self.read_i8()
    }

    fn read_message_unknown_message(&mut self) -> io::Result<Message> {
        unimplemented!()
    }

    fn read_message_game_over(&mut self) -> io::Result<Message> {
        Ok(Message::GameOver)
    }

    fn read_message_authentication_token<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::AuthenticationToken(self.read_string::<B>()?))
    }

    fn read_message_team_size<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::TeamSize(self.read_i32::<B>()?))
    }

    fn read_message_protocol_version<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::ProtocolVersion(self.read_i32::<B>()?))
    }

    fn read_message_game_context<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::GameContext(self.read_game::<B>()?))
    }

    fn read_message_player_context<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<Message> {
        Ok(Message::PlayerContext(self.read_player_context::<B>(cache)?))
    }

    fn read_message_moves_message(&mut self) -> io::Result<Message> {
        unimplemented!()
    }

    fn read_game<B: ByteOrder>(&mut self) -> io::Result<Game> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_game error: value is false"));
        }

        let result = Game {
            random_seed: self.read_i64::<B>()?,
            tick_count: self.read_i32::<B>()?,
            world_width: self.read_f64::<B>()?,
            world_height: self.read_f64::<B>()?,
            fog_of_war_enabled: self.read_bool()?,
            victory_score: self.read_i32::<B>()?,
            facility_capture_score: self.read_i32::<B>()?,
            vehicle_elimination_score: self.read_i32::<B>()?,
            action_detection_interval: self.read_i32::<B>()?,
            base_action_count: self.read_i32::<B>()?,
            additional_action_count_per_control_center: self.read_i32::<B>()?,
            max_unit_group: self.read_i32::<B>()?,
            terrain_weather_map_column_count: self.read_i32::<B>()?,
            terrain_weather_map_row_count: self.read_i32::<B>()?,
            plain_terrain_vision_factor: self.read_f64::<B>()?,
            plain_terrain_stealth_factor: self.read_f64::<B>()?,
            plain_terrain_speed_factor: self.read_f64::<B>()?,
            swamp_terrain_vision_factor: self.read_f64::<B>()?,
            swamp_terrain_stealth_factor: self.read_f64::<B>()?,
            swamp_terrain_speed_factor: self.read_f64::<B>()?,
            forest_terrain_vision_factor: self.read_f64::<B>()?,
            forest_terrain_stealth_factor: self.read_f64::<B>()?,
            forest_terrain_speed_factor: self.read_f64::<B>()?,
            clear_weather_vision_factor: self.read_f64::<B>()?,
            clear_weather_stealth_factor: self.read_f64::<B>()?,
            clear_weather_speed_factor: self.read_f64::<B>()?,
            cloud_weather_vision_factor: self.read_f64::<B>()?,
            cloud_weather_stealth_factor: self.read_f64::<B>()?,
            cloud_weather_speed_factor: self.read_f64::<B>()?,
            rain_weather_vision_factor: self.read_f64::<B>()?,
            rain_weather_stealth_factor: self.read_f64::<B>()?,
            rain_weather_speed_factor: self.read_f64::<B>()?,
            vehicle_radius: self.read_f64::<B>()?,
            tank_durability: self.read_i32::<B>()?,
            tank_speed: self.read_f64::<B>()?,
            tank_vision_range: self.read_f64::<B>()?,
            tank_ground_attack_range: self.read_f64::<B>()?,
            tank_aerial_attack_range: self.read_f64::<B>()?,
            tank_ground_damage: self.read_i32::<B>()?,
            tank_aerial_damage: self.read_i32::<B>()?,
            tank_ground_defence: self.read_i32::<B>()?,
            tank_aerial_defence: self.read_i32::<B>()?,
            tank_attack_cooldown_ticks: self.read_i32::<B>()?,
            tank_production_cost: self.read_i32::<B>()?,
            ifv_durability: self.read_i32::<B>()?,
            ifv_speed: self.read_f64::<B>()?,
            ifv_vision_range: self.read_f64::<B>()?,
            ifv_ground_attack_range: self.read_f64::<B>()?,
            ifv_aerial_attack_range: self.read_f64::<B>()?,
            ifv_ground_damage: self.read_i32::<B>()?,
            ifv_aerial_damage: self.read_i32::<B>()?,
            ifv_ground_defence: self.read_i32::<B>()?,
            ifv_aerial_defence: self.read_i32::<B>()?,
            ifv_attack_cooldown_ticks: self.read_i32::<B>()?,
            ifv_production_cost: self.read_i32::<B>()?,
            arrv_durability: self.read_i32::<B>()?,
            arrv_speed: self.read_f64::<B>()?,
            arrv_vision_range: self.read_f64::<B>()?,
            arrv_ground_defence: self.read_i32::<B>()?,
            arrv_aerial_defence: self.read_i32::<B>()?,
            arrv_production_cost: self.read_i32::<B>()?,
            arrv_repair_range: self.read_f64::<B>()?,
            arrv_repair_speed: self.read_f64::<B>()?,
            helicopter_durability: self.read_i32::<B>()?,
            helicopter_speed: self.read_f64::<B>()?,
            helicopter_vision_range: self.read_f64::<B>()?,
            helicopter_ground_attack_range: self.read_f64::<B>()?,
            helicopter_aerial_attack_range: self.read_f64::<B>()?,
            helicopter_ground_damage: self.read_i32::<B>()?,
            helicopter_aerial_damage: self.read_i32::<B>()?,
            helicopter_ground_defence: self.read_i32::<B>()?,
            helicopter_aerial_defence: self.read_i32::<B>()?,
            helicopter_attack_cooldown_ticks: self.read_i32::<B>()?,
            helicopter_production_cost: self.read_i32::<B>()?,
            fighter_durability: self.read_i32::<B>()?,
            fighter_speed: self.read_f64::<B>()?,
            fighter_vision_range: self.read_f64::<B>()?,
            fighter_ground_attack_range: self.read_f64::<B>()?,
            fighter_aerial_attack_range: self.read_f64::<B>()?,
            fighter_ground_damage: self.read_i32::<B>()?,
            fighter_aerial_damage: self.read_i32::<B>()?,
            fighter_ground_defence: self.read_i32::<B>()?,
            fighter_aerial_defence: self.read_i32::<B>()?,
            fighter_attack_cooldown_ticks: self.read_i32::<B>()?,
            fighter_production_cost: self.read_i32::<B>()?,
            max_facility_capture_points: self.read_f64::<B>()?,
            facility_capture_points_per_vehicle_per_tick: self.read_f64::<B>()?,
            facility_width: self.read_f64::<B>()?,
            facility_height: self.read_f64::<B>()?,
            base_tactical_nuclear_strike_cooldown: self.read_i32::<B>()?,
            tactical_nuclear_strike_cooldown_decrease_per_control_center: self.read_i32::<B>()?,
            max_tactical_nuclear_strike_damage: self.read_f64::<B>()?,
            tactical_nuclear_strike_radius: self.read_f64::<B>()?,
            tactical_nuclear_strike_delay: self.read_i32::<B>()?,
        };

        Ok(result)
    }

    fn read_player_context<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<PlayerContext> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_player_context error: value is false"));
        }

        let result = PlayerContext {
            player: self.read_player::<B>(&mut cache.players)?,
            world: self.read_world::<B>(cache)?,
        };

        Ok(result)
    }

    fn read_player<B: ByteOrder>(&mut self, cache: &mut HashMap<i64, Player>) -> io::Result<Player> {
        use std::io::{Error, ErrorKind};

        match self.read_u8()? {
            0 => return Err(Error::new(ErrorKind::Other, "ReadMessage::read_player error: value is 0")),
            127 => {
                let id = self.read_i64::<B>()?;
                return Ok(cache[&id].clone());
            },
            _ => {},
        }

        let result = Player {
            id: self.read_i64::<B>()?,
            me: self.read_bool()?,
            strategy_crashed: self.read_bool()?,
            score: self.read_i32::<B>()?,
            remaining_action_cooldown_ticks: self.read_i32::<B>()?,
            remaining_nuclear_strike_cooldown_ticks: self.read_i32::<B>()?,
            next_nuclear_strike_vehicle_id: self.read_i64::<B>()?,
            next_nuclear_strike_tick_index: self.read_i32::<B>()?,
            next_nuclear_strike_x: self.read_f64::<B>()?,
            next_nuclear_strike_y: self.read_f64::<B>()?,
        };

        cache.insert(result.id, result.clone());

        Ok(result)
    }

    fn read_world<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<World> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_world error: value is false"));
        }

        let result = World {
            tick_index: self.read_i32::<B>()?,
            tick_count: self.read_i32::<B>()?,
            width: self.read_f64::<B>()?,
            height: self.read_f64::<B>()?,
            players: self.read_vec_player::<B>(&mut cache.players)?,
            new_vehicles: self.read_vec_vehicle::<B>()?,
            vehicle_updates: self.read_vec_vehicle_update::<B>()?,
            terrain_by_cell_x_y: {
                if cache.terrain_by_cell_x_y.is_empty() {
                    cache.terrain_by_cell_x_y = self.read_vec_vec_terrain_type::<B>()?
                }
                cache.terrain_by_cell_x_y.clone()
            },
            weather_by_cell_x_y: {
                if cache.weather_by_cell_x_y.is_empty() {
                    cache.weather_by_cell_x_y = self.read_vec_vec_weather_type::<B>()?
                }
                cache.weather_by_cell_x_y.clone()
            },
            facilities: self.read_vec_facility::<B>(&mut cache.facilities)?,
        };

        Ok(result)
    }

    fn read_vehicle<B: ByteOrder>(&mut self) -> io::Result<Vehicle> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_vehicle error: value is false"));
        }

        let mut result = Vehicle::new();

        result
            .set_id(self.read_i64::<B>()?)
            .set_x(self.read_f64::<B>()?)
            .set_y(self.read_f64::<B>()?)
            .set_radius(self.read_f64::<B>()?)
            .set_player_id(self.read_i64::<B>()?)
            .set_durability(self.read_i32::<B>()?)
            .set_max_durability(self.read_i32::<B>()?)
            .set_max_speed(self.read_f64::<B>()?)
            .set_vision_range(self.read_f64::<B>()?)
            .set_squared_vision_range(self.read_f64::<B>()?)
            .set_ground_attack_range(self.read_f64::<B>()?)
            .set_squared_ground_attack_range(self.read_f64::<B>()?)
            .set_aerial_attack_range(self.read_f64::<B>()?)
            .set_squared_aerial_attack_range(self.read_f64::<B>()?)
            .set_ground_damage(self.read_i32::<B>()?)
            .set_aerial_damage(self.read_i32::<B>()?)
            .set_ground_defence(self.read_i32::<B>()?)
            .set_aerial_defence(self.read_i32::<B>()?)
            .set_attack_cooldown_ticks(self.read_i32::<B>()?)
            .set_remaining_attack_cooldown_ticks(self.read_i32::<B>()?)
            .set_type(self.read_vehicle_type()?)
            .set_aerial(self.read_bool()?)
            .set_selected(self.read_bool()?)
            .set_groups(self.read_vec_i32::<B>()?);

        Ok(result)
    }

    fn read_vehicle_update<B: ByteOrder>(&mut self) -> io::Result<VehicleUpdate> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_vehicle_update error: value is false"));
        }

        let result = VehicleUpdate {
            id: self.read_i64::<B>()?,
            x: self.read_f64::<B>()?,
            y: self.read_f64::<B>()?,
            durability: self.read_i32::<B>()?,
            remaining_attack_cooldown_ticks: self.read_i32::<B>()?,
            selected: self.read_bool()?,
            groups: self.read_vec_i32::<B>()?,
        };

        Ok(result)
    }

    fn read_terrain_type(&mut self) -> io::Result<TerrainType> {
        use std::io::{Error, ErrorKind};
        match self.read_i8()? {
            -1 => Ok(TerrainType::Unknown),
            0 => Ok(TerrainType::Plain),
            1 => Ok(TerrainType::Swamp),
            2 => Ok(TerrainType::Forest),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_terrain_type error: invalid TerrainType value: {}", v))),
        }
    }

    fn read_weather_type(&mut self) -> io::Result<WeatherType> {
        use std::io::{Error, ErrorKind};
        match self.read_i8()? {
            -1 => Ok(WeatherType::Unknown),
            0 => Ok(WeatherType::Clear),
            1 => Ok(WeatherType::Cloud),
            2 => Ok(WeatherType::Rain),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_weather_type error: invalid WeatherType value: {}", v))),
        }
    }

    fn read_vehicle_type(&mut self) -> io::Result<VehicleType> {
        use std::io::{Error, ErrorKind};
        match self.read_i8()? {
            -1 => Ok(VehicleType::Unknown),
            0 => Ok(VehicleType::None),
            1 => Ok(VehicleType::Arrv),
            2 => Ok(VehicleType::Fighter),
            3 => Ok(VehicleType::Helicopter),
            4 => Ok(VehicleType::Ifv),
            5 => Ok(VehicleType::Tank),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_vehicle_type error: invalid VehicleType value: {}", v))),
        }
    }

    fn read_facility<B: ByteOrder>(&mut self, cache: &mut HashMap<i64, Facility>) -> io::Result<Facility> {
        use std::io::{Error, ErrorKind};

        match self.read_u8()? {
            0 => return Err(Error::new(ErrorKind::Other, "ReadMessage::read_facility error: value is 0")),
            127 => {
                let id = self.read_i64::<B>()?;
                return Ok(cache[&id].clone());
            },
            _ => {},
        }

        let result = Facility {
            id: self.read_i64::<B>()?,
            type_: self.read_facility_type()?,
            owner_player_id: self.read_i64::<B>()?,
            left: self.read_f64::<B>()?,
            top: self.read_f64::<B>()?,
            capture_points: self.read_f64::<B>()?,
            vehicle_type: self.read_vehicle_type()?,
            production_progress: self.read_i32::<B>()?,
        };

        cache.insert(result.id, result.clone());

        Ok(result)
    }

    fn read_facility_type(&mut self) -> io::Result<FacilityType> {
        use std::io::{Error, ErrorKind};
        match self.read_i8()? {
            -1 => Ok(FacilityType::Unknown),
            0 => Ok(FacilityType::ControlCenter),
            1 => Ok(FacilityType::VehicleFactory),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_facility_type error: invalid FacilityType value: {}", v))),
        }
    }

    fn read_string<B: ByteOrder>(&mut self) -> io::Result<String> {
        use std::io::{Error, ErrorKind};
        let buf = self.read_vec_u8::<B>()?;
        match String::from_utf8(buf) {
            Ok(v) => Ok(v),
            Err(v) => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_string error: {:?}", v))),
        }
    }

    fn read_vec_player<B: ByteOrder>(&mut self, cache: &mut HashMap<i64, Player>) -> io::Result<Vec<Player>> {
        let len = self.read_i32::<B>()?;
        if len < 0 {
            Ok(cache.values().cloned().collect())
        } else {
            self.read_vec_impl::<B, _, _>(len as usize, |s| s.read_player::<B>(cache))
        }
    }

    fn read_vec_vehicle<B: ByteOrder>(&mut self) -> io::Result<Vec<Vehicle>> {
        self.read_vec::<B, _, _>(|s| s.read_vehicle::<B>())
    }

    fn read_vec_vehicle_update<B: ByteOrder>(&mut self) -> io::Result<Vec<VehicleUpdate>> {
        self.read_vec::<B, _, _>(|s| s.read_vehicle_update::<B>())
    }

    fn read_vec_vec_terrain_type<B: ByteOrder>(&mut self) -> io::Result<Vec<Vec<TerrainType>>> {
        self.read_vec::<B, _, _>(|s| s.read_vec::<B, _, _>(|ss| ss.read_terrain_type()))
    }

    fn read_vec_vec_weather_type<B: ByteOrder>(&mut self) -> io::Result<Vec<Vec<WeatherType>>> {
        self.read_vec::<B, _, _>(|s| s.read_vec::<B, _, _>(|ss| ss.read_weather_type()))
    }

    fn read_vec_facility<B: ByteOrder>(&mut self, cache: &mut HashMap<i64, Facility>) -> io::Result<Vec<Facility>> {
        let len = self.read_i32::<B>()?;
        if len < 0 {
            Ok(cache.values().cloned().collect())
        } else {
            self.read_vec_impl::<B, _, _>(len as usize, |s| s.read_facility::<B>(cache))
        }
    }

    fn read_vec_i64<B: ByteOrder>(&mut self) -> io::Result<Vec<i64>> {
        self.read_vec::<B, _, _>(|s| s.read_i64::<B>())
    }

    fn read_vec_i32<B: ByteOrder>(&mut self) -> io::Result<Vec<i32>> {
        self.read_vec::<B, _, _>(|s| s.read_i32::<B>())
    }

    fn read_vec_i8<B: ByteOrder>(&mut self) -> io::Result<Vec<i8>> {
        self.read_vec::<B, _, _>(|s| s.read_i8())
    }

    fn read_vec_u8<B: ByteOrder>(&mut self) -> io::Result<Vec<u8>> {
        use std::io::{Error, ErrorKind};
        let len = self.read_i32::<B>()?;
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_vec_u8 error: len < 0, where len={}",  len)));
        }
        let mut result = vec![0; len as usize];
        self.read_exact(&mut result)?;
        Ok(result)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    fn read_vec<B: ByteOrder, T, F>(&mut self, read: F) -> io::Result<Vec<T>>
            where F: FnMut(&mut Self) -> io::Result<T> {
        use std::io::{Error, ErrorKind};
        let len = self.read_i32::<B>()?;
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_vec error: len < 0, where len={}", len)));
        }
        self.read_vec_impl::<B, _, _>(len as usize, read)
    }

    fn read_vec_impl<B: ByteOrder, T, F>(&mut self, len: usize, mut read: F) -> io::Result<Vec<T>>
            where F: FnMut(&mut Self) -> io::Result<T> {
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(read(self)?);
        }
        Ok(result)
    }
}

impl<R: ReadBytesExt> ReadMessage for R {}

#[test]
fn test_read_bool() {
    use std::io::Cursor;
    assert_eq!(Cursor::new(vec![0u8]).read_bool().unwrap(), false);
    assert_eq!(Cursor::new(vec![1u8]).read_bool().unwrap(), true);
    assert_eq!(Cursor::new(vec![255u8]).read_bool().unwrap(), true);
}

#[test]
fn test_read_vec_u8() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, 13u8])
        .read_vec_u8::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42u8, 13u8]);
}

#[test]
fn test_read_vec_i8() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, -42i8 as u8])
        .read_vec_i8::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i8, -42i8]);
}

#[test]
fn test_read_vec_i32() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8])
        .read_vec_i32::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i32, 13i32]);
}

#[test]
fn test_read_string() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 'a' as u8, 'b' as u8])
        .read_string::<LittleEndian>()
        .unwrap();
    assert_eq!(result, "ab".to_string());
}

#[test]
fn test_read_facility_type() {
    use std::io::Cursor;
    assert_eq!(Cursor::new(vec![-1i8 as u8]).read_facility_type().unwrap(), FacilityType::Unknown);
    assert_eq!(Cursor::new(vec![0u8]).read_facility_type().unwrap(), FacilityType::ControlCenter);
    assert_eq!(Cursor::new(vec![1u8]).read_facility_type().unwrap(), FacilityType::VehicleFactory);
    assert_eq!(Cursor::new(vec![6u8]).read_facility_type().is_ok(), false);
}

#[test]
fn test_read_player() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let player = Player {
        id: 42,
        me: true,
        strategy_crashed: false,
        score: 13,
        remaining_action_cooldown_ticks: 146,
        remaining_nuclear_strike_cooldown_ticks: 43,
        next_nuclear_strike_vehicle_id: 14,
        next_nuclear_strike_tick_index: 147,
        next_nuclear_strike_x: 1.0,
        next_nuclear_strike_y: 2.0,
    };
    let mut cache = HashMap::new();
    let result = Cursor::new(vec![
        1u8,
        42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        1u8,
        0u8,
        13u8, 0u8, 0u8, 0u8,
        146u8, 0u8, 0u8, 0u8,
        43u8, 0u8, 0u8, 0u8,
        14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        147u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 63u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8,
    ]).read_player::<LittleEndian>(&mut cache).unwrap();
    assert_eq!(result, player);
    assert_eq!(cache[&42i64], result);
}

#[test]
fn test_read_cached_player() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let player = Player {
        id: 42,
        me: true,
        strategy_crashed: false,
        score: 13,
        remaining_action_cooldown_ticks: 146,
        remaining_nuclear_strike_cooldown_ticks: 43,
        next_nuclear_strike_vehicle_id: 14,
        next_nuclear_strike_tick_index: 147,
        next_nuclear_strike_x: 1.0,
        next_nuclear_strike_y: 2.0,
    };
    let mut cache = [(player.id, player.clone())].iter().cloned().collect();
    let result = Cursor::new(vec![
        127u8,
        42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    ]).read_player::<LittleEndian>(&mut cache).unwrap();
    assert_eq!(cache[&42i64], result);
}

#[test]
fn test_read_message_game_over() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let mut cache = Cache {
        facilities: HashMap::new(),
        players: HashMap::new(),
        terrain_by_cell_x_y: vec![],
        weather_by_cell_x_y: vec![],
    };
    assert_eq!(
        Cursor::new(vec![1u8]).read_message::<LittleEndian>(&mut cache).unwrap(),
        Message::GameOver
    );
}

#[test]
fn test_read_message_team_size() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let mut cache = Cache {
        facilities: HashMap::new(),
        players: HashMap::new(),
        terrain_by_cell_x_y: vec![],
        weather_by_cell_x_y: vec![],
    };
    assert_eq!(
        Cursor::new(vec![3u8, 42u8, 0u8, 0u8, 0u8]).read_message::<LittleEndian>(&mut cache).unwrap(),
        Message::TeamSize(42)
    );
}

#[test]
fn test_read_vec_player() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let player = Player {
        id: 42,
        me: true,
        strategy_crashed: false,
        score: 13,
        remaining_action_cooldown_ticks: 146,
        remaining_nuclear_strike_cooldown_ticks: 43,
        next_nuclear_strike_vehicle_id: 14,
        next_nuclear_strike_tick_index: 147,
        next_nuclear_strike_x: 1.0,
        next_nuclear_strike_y: 2.0,
    };
    let mut cache = [(player.id, player.clone())].iter().cloned().collect();
    let result = Cursor::new(vec![
        1u8, 0u8, 0u8, 0u8,
        127u8,
        42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    ]).read_vec_player::<LittleEndian>(&mut cache).unwrap();
    assert_eq!(vec![cache[&42i64].clone()], result);
}

#[test]
fn test_read_cached_vec_player() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let player = Player {
        id: 42,
        me: true,
        strategy_crashed: false,
        score: 13,
        remaining_action_cooldown_ticks: 146,
        remaining_nuclear_strike_cooldown_ticks: 43,
        next_nuclear_strike_vehicle_id: 14,
        next_nuclear_strike_tick_index: 147,
        next_nuclear_strike_x: 1.0,
        next_nuclear_strike_y: 2.0,
    };
    let mut cache = [(player.id, player.clone())].iter().cloned().collect();
    let result = Cursor::new(vec![
        255u8, 255u8, 255u8, 255u8,
    ]).read_vec_player::<LittleEndian>(&mut cache).unwrap();
    assert_eq!(vec![cache[&42i64].clone()], result);
}
