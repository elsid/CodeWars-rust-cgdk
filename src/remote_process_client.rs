use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::io;
use std::net::TcpStream;
use core::fmt::Debug;
use core::hash::Hash;
use core::mem::transmute;
use core::ptr::copy_nonoverlapping;
use model::{
    ActionType,
    Facility,
    FacilityType,
    Game,
    Action,
    Player,
    PlayerContext,
    TerrainType,
    Vehicle,
    VehicleType,
    VehicleUpdate,
    WeatherType,
    World,
};

const PROTOCOL_VERSION: i32 = 3;

pub struct RemoteProcessClient {
    cache: Cache,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

#[derive(Default)]
struct Cache {
    pub facilities: Vec<Facility>,
    pub players: Vec<Player>,
    pub facilities_by_id: HashMap<i64, Facility>,
    pub players_by_id: HashMap<i64, Player>,
    pub terrain_by_cell_x_y: Vec<Vec<Option<TerrainType>>>,
    pub weather_by_cell_x_y: Vec<Vec<Option<WeatherType>>>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Message {
    UnknownMessage,
    GameOver,
    AuthenticationToken(String),
    TeamSize(i32),
    ProtocolVersion(i32),
    GameContext(Game),
    PlayerContext(PlayerContext),
    ActionMessage(Action),
}

macro_rules! read_option_enum_impl {
    ($function:ident, $name:ident, $error_format:tt, $($variant:ident),*) => {
        fn $function(&mut self) -> io::Result<Option<$name>> {
            let value = self.read_i8()?;

            if value < 0 {
                return Ok(None)
            }

            $(if value == $name::$variant as i8 {
                Ok(Some($name::$variant))
            } else)* {
                Err(Error::new(ErrorKind::Other, format!($error_format, value)))
            }
        }
    };
}

impl RemoteProcessClient {
    pub fn connect<'r>(host: &'r str, port: u16) -> io::Result<(Self)> {
        let stream = TcpStream::connect((host, port))?;
        stream.set_nodelay(true)?;
        let result = RemoteProcessClient {
            cache: Cache::default(),
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream),
        };
        Ok(result)
    }

    pub fn write_authentication_token_message(&mut self, token: String) -> io::Result<()> {
        self.write_message(&Message::AuthenticationToken(token.clone()))
    }

    pub fn write_protocol_version_message(&mut self) -> io::Result<()> {
        self.write_message(&Message::ProtocolVersion(PROTOCOL_VERSION))
    }

    pub fn read_team_size_message(&mut self) -> io::Result<i32> {
        match self.read_message()? {
            Message::TeamSize(v) => Ok(v),
            v => Err(Error::new(ErrorKind::Other,
                                format!("Expected Message::TeamSize, but received: {:?}", v))),
        }
    }

    pub fn read_game_message(&mut self) -> io::Result<Game> {
        match self.read_message()? {
            Message::GameContext(v) => Ok(v),
            v => Err(Error::new(ErrorKind::Other,
                                format!("Expected Message::GameContext, but received: {:?}", v))),
        }
    }

    pub fn read_player_context_message(&mut self) -> io::Result<Option<PlayerContext>> {
        match self.read_message()? {
            Message::GameOver => Ok(None),
            Message::PlayerContext(v) => Ok(Some(v)),
            v => return Err(Error::new(ErrorKind::Other,
                                       format!("Expected Message::GameOver, \
                                            Message::PlayerContext or \
                                            Message::PlayerContextWithoutTrees, but \
                                            received: {:?}", v)))
        }
    }

    pub fn write_action_message(&mut self, action: Action) -> io::Result<()> {
        self.write_message(&Message::ActionMessage(action))
    }

    fn read_message(&mut self) -> io::Result<Message> {
        use std::io::{Error, ErrorKind};
        match self.read_i8()? {
            0 => unimplemented!(),
            1 => Ok(Message::GameOver),
            2 => unimplemented!(),
            3 => Ok(Message::TeamSize(self.read_i32()?)),
            4 => unimplemented!(),
            5 => Ok(Message::GameContext(self.read_game()?)),
            6 => Ok(Message::PlayerContext(self.read_player_context()?)),
            7 => unimplemented!(),
            v => Err(Error::new(ErrorKind::Other,
                                format!("RemoteProcessClient::read_message error: invalid message id: {}", v)))
        }
    }

    fn read_game(&mut self) -> io::Result<Game> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other,
                                  "RemoteProcessClient::read_game error: value is false"));
        }

        let result = Game {
            random_seed: self.read_i64()?,
            tick_count: self.read_i32()?,
            world_width: self.read_f64()?,
            world_height: self.read_f64()?,
            fog_of_war_enabled: self.read_bool()?,
            victory_score: self.read_i32()?,
            facility_capture_score: self.read_i32()?,
            vehicle_elimination_score: self.read_i32()?,
            action_detection_interval: self.read_i32()?,
            base_action_count: self.read_i32()?,
            additional_action_count_per_control_center: self.read_i32()?,
            max_unit_group: self.read_i32()?,
            terrain_weather_map_column_count: self.read_i32()?,
            terrain_weather_map_row_count: self.read_i32()?,
            plain_terrain_vision_factor: self.read_f64()?,
            plain_terrain_stealth_factor: self.read_f64()?,
            plain_terrain_speed_factor: self.read_f64()?,
            swamp_terrain_vision_factor: self.read_f64()?,
            swamp_terrain_stealth_factor: self.read_f64()?,
            swamp_terrain_speed_factor: self.read_f64()?,
            forest_terrain_vision_factor: self.read_f64()?,
            forest_terrain_stealth_factor: self.read_f64()?,
            forest_terrain_speed_factor: self.read_f64()?,
            clear_weather_vision_factor: self.read_f64()?,
            clear_weather_stealth_factor: self.read_f64()?,
            clear_weather_speed_factor: self.read_f64()?,
            cloud_weather_vision_factor: self.read_f64()?,
            cloud_weather_stealth_factor: self.read_f64()?,
            cloud_weather_speed_factor: self.read_f64()?,
            rain_weather_vision_factor: self.read_f64()?,
            rain_weather_stealth_factor: self.read_f64()?,
            rain_weather_speed_factor: self.read_f64()?,
            vehicle_radius: self.read_f64()?,
            tank_durability: self.read_i32()?,
            tank_speed: self.read_f64()?,
            tank_vision_range: self.read_f64()?,
            tank_ground_attack_range: self.read_f64()?,
            tank_aerial_attack_range: self.read_f64()?,
            tank_ground_damage: self.read_i32()?,
            tank_aerial_damage: self.read_i32()?,
            tank_ground_defence: self.read_i32()?,
            tank_aerial_defence: self.read_i32()?,
            tank_attack_cooldown_ticks: self.read_i32()?,
            tank_production_cost: self.read_i32()?,
            ifv_durability: self.read_i32()?,
            ifv_speed: self.read_f64()?,
            ifv_vision_range: self.read_f64()?,
            ifv_ground_attack_range: self.read_f64()?,
            ifv_aerial_attack_range: self.read_f64()?,
            ifv_ground_damage: self.read_i32()?,
            ifv_aerial_damage: self.read_i32()?,
            ifv_ground_defence: self.read_i32()?,
            ifv_aerial_defence: self.read_i32()?,
            ifv_attack_cooldown_ticks: self.read_i32()?,
            ifv_production_cost: self.read_i32()?,
            arrv_durability: self.read_i32()?,
            arrv_speed: self.read_f64()?,
            arrv_vision_range: self.read_f64()?,
            arrv_ground_defence: self.read_i32()?,
            arrv_aerial_defence: self.read_i32()?,
            arrv_production_cost: self.read_i32()?,
            arrv_repair_range: self.read_f64()?,
            arrv_repair_speed: self.read_f64()?,
            helicopter_durability: self.read_i32()?,
            helicopter_speed: self.read_f64()?,
            helicopter_vision_range: self.read_f64()?,
            helicopter_ground_attack_range: self.read_f64()?,
            helicopter_aerial_attack_range: self.read_f64()?,
            helicopter_ground_damage: self.read_i32()?,
            helicopter_aerial_damage: self.read_i32()?,
            helicopter_ground_defence: self.read_i32()?,
            helicopter_aerial_defence: self.read_i32()?,
            helicopter_attack_cooldown_ticks: self.read_i32()?,
            helicopter_production_cost: self.read_i32()?,
            fighter_durability: self.read_i32()?,
            fighter_speed: self.read_f64()?,
            fighter_vision_range: self.read_f64()?,
            fighter_ground_attack_range: self.read_f64()?,
            fighter_aerial_attack_range: self.read_f64()?,
            fighter_ground_damage: self.read_i32()?,
            fighter_aerial_damage: self.read_i32()?,
            fighter_ground_defence: self.read_i32()?,
            fighter_aerial_defence: self.read_i32()?,
            fighter_attack_cooldown_ticks: self.read_i32()?,
            fighter_production_cost: self.read_i32()?,
            max_facility_capture_points: self.read_f64()?,
            facility_capture_points_per_vehicle_per_tick: self.read_f64()?,
            facility_width: self.read_f64()?,
            facility_height: self.read_f64()?,
            base_tactical_nuclear_strike_cooldown: self.read_i32()?,
            tactical_nuclear_strike_cooldown_decrease_per_control_center: self.read_i32()?,
            max_tactical_nuclear_strike_damage: self.read_f64()?,
            tactical_nuclear_strike_radius: self.read_f64()?,
            tactical_nuclear_strike_delay: self.read_i32()?,
        };

        Ok(result)
    }

    fn read_player_context(&mut self) -> io::Result<PlayerContext> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_player_context error: value is false"));
        }

        let result = PlayerContext {
            player: self.read_player()?,
            world: self.read_world()?,
        };

        Ok(result)
    }

    fn read_player(&mut self) -> io::Result<Player> {
        use std::io::{Error, ErrorKind};

        match self.read_u8()? {
            0 => return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_player error: value is 0")),
            127 => {
                let id = self.read_i64()?;
                return Ok(self.cache.players_by_id[&id].clone());
            },
            _ => {},
        }

        let result = Player {
            id: self.read_i64()?,
            me: self.read_bool()?,
            strategy_crashed: self.read_bool()?,
            score: self.read_i32()?,
            remaining_action_cooldown_ticks: self.read_i32()?,
            remaining_nuclear_strike_cooldown_ticks: self.read_i32()?,
            next_nuclear_strike_vehicle_id: self.read_i64()?,
            next_nuclear_strike_tick_index: self.read_i32()?,
            next_nuclear_strike_x: self.read_f64()?,
            next_nuclear_strike_y: self.read_f64()?,
        };

        self.cache.players_by_id.insert(result.id, result.clone());

        Ok(result)
    }

    fn read_world(&mut self) -> io::Result<World> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_world error: value is false"));
        }

        let result = World {
            tick_index: self.read_i32()?,
            tick_count: self.read_i32()?,
            width: self.read_f64()?,
            height: self.read_f64()?,
            players: self.read_players()?,
            new_vehicles: self.read_vehicles()?,
            vehicle_updates: self.read_vehicles_update()?,
            terrain_by_cell_x_y: {
                if self.cache.terrain_by_cell_x_y.is_empty() {
                    self.cache.terrain_by_cell_x_y = self.read_terrain_types_2d()?;
                }
                self.cache.terrain_by_cell_x_y.clone()
            },
            weather_by_cell_x_y: {
                if self.cache.weather_by_cell_x_y.is_empty() {
                    self.cache.weather_by_cell_x_y = self.read_weather_types_2d()?;
                }
                self.cache.weather_by_cell_x_y.clone()
            },
            facilities: self.read_facilities()?,
        };

        Ok(result)
    }

    fn read_vehicle(&mut self) -> io::Result<Vehicle> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_vehicle error: value is false"));
        }

        let result = Vehicle {
            id: self.read_i64()?,
            x: self.read_f64()?,
            y: self.read_f64()?,
            radius: self.read_f64()?,
            player_id: self.read_i64()?,
            durability: self.read_i32()?,
            max_durability: self.read_i32()?,
            max_speed: self.read_f64()?,
            vision_range: self.read_f64()?,
            squared_vision_range: self.read_f64()?,
            ground_attack_range: self.read_f64()?,
            squared_ground_attack_range: self.read_f64()?,
            aerial_attack_range: self.read_f64()?,
            squared_aerial_attack_range: self.read_f64()?,
            ground_damage: self.read_i32()?,
            aerial_damage: self.read_i32()?,
            ground_defence: self.read_i32()?,
            aerial_defence: self.read_i32()?,
            attack_cooldown_ticks: self.read_i32()?,
            remaining_attack_cooldown_ticks: self.read_i32()?,
            kind: self.read_vehicle_type()?,
            aerial: self.read_bool()?,
            selected: self.read_bool()?,
            groups: self.read_vec_i32()?,
        };

        Ok(result)
    }

    fn read_vehicle_update(&mut self) -> io::Result<VehicleUpdate> {
        use std::io::{Error, ErrorKind};

        if !self.read_bool()? {
            return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_vehicle_update error: value is false"));
        }

        let result = VehicleUpdate {
            id: self.read_i64()?,
            x: self.read_f64()?,
            y: self.read_f64()?,
            durability: self.read_i32()?,
            remaining_attack_cooldown_ticks: self.read_i32()?,
            selected: self.read_bool()?,
            groups: self.read_vec_i32()?,
        };

        Ok(result)
    }

    fn read_facility(&mut self) -> io::Result<Facility> {
        use std::io::{Error, ErrorKind};

        match self.read_u8()? {
            0 => return Err(Error::new(ErrorKind::Other, "RemoteProcessClient::read_facility error: value is 0")),
            127 => {
                let id = self.read_i64()?;
                return Ok(self.cache.facilities_by_id[&id].clone());
            },
            _ => {},
        }

        let result = Facility {
            id: self.read_i64()?,
            kind: self.read_facility_type()?,
            owner_player_id: self.read_i64()?,
            left: self.read_f64()?,
            top: self.read_f64()?,
            capture_points: self.read_f64()?,
            vehicle_type: self.read_vehicle_type()?,
            production_progress: self.read_i32()?,
        };

        self.cache.facilities_by_id.insert(result.id, result.clone());

        Ok(result)
    }

    read_option_enum_impl!(read_facility_type, FacilityType,
        "RemoteProcessClient::read_facility_type error: invalid FacilityType value: {}",
        ControlCenter, VehicleFactory);

    read_option_enum_impl!(read_vehicle_type, VehicleType,
        "RemoteProcessClient::read_vehicle_type error: invalid VehicleType value: {}",
        Arrv, Fighter, Helicopter, Ifv, Tank);

    read_option_enum_impl!(read_terrain_type, TerrainType,
        "RemoteProcessClient::read_terrain_type error: invalid TerrainType value: {}",
        Plain, Swamp, Forest);

    read_option_enum_impl!(read_weather_type, WeatherType,
        "RemoteProcessClient::read_weather_type error: invalid WeatherType value: {}",
        Clear, Cloud, Rain);

    fn read_players(&mut self) -> io::Result<Vec<Player>> {
        let len = self.read_i32()?;
        if len < 0 {
            Ok(self.cache.players.clone())
        } else {
            let players = self.read_vec_impl(len as usize, |s| s.read_player())?;
            self.cache.players = players.clone();
            Ok(players)
        }
    }

    fn read_vehicles(&mut self) -> io::Result<Vec<Vehicle>> {
        self.read_vec(|s| s.read_vehicle())
    }

    fn read_vehicles_update(&mut self) -> io::Result<Vec<VehicleUpdate>> {
        self.read_vec(|s| s.read_vehicle_update())
    }

    fn read_terrain_types_2d(&mut self) -> io::Result<Vec<Vec<Option<TerrainType>>>> {
        self.read_vec(|s| s.read_vec(|ss| ss.read_terrain_type()))
    }

    fn read_weather_types_2d(&mut self) -> io::Result<Vec<Vec<Option<WeatherType>>>> {
        self.read_vec(|s| s.read_vec(|ss| ss.read_weather_type()))
    }

    fn read_facilities(&mut self) -> io::Result<Vec<Facility>> {
        let len = self.read_i32()?;
        if len < 0 {
            Ok(self.cache.facilities.clone())
        } else {
            let facilities = self.read_vec_impl(len as usize, |s| s.read_facility())?;
            self.cache.facilities = facilities.clone();
            Ok(facilities)
        }
    }

    fn read_vec_i32(&mut self) -> io::Result<Vec<i32>> {
        self.read_vec(|s| s.read_i32())
    }

    #[inline]
    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    fn read_vec<T, F>(&mut self, read: F) -> io::Result<Vec<T>>
        where F: FnMut(&mut Self) -> io::Result<T> {
        use std::io::{Error, ErrorKind};
        let len = self.read_i32()?;
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, format!("RemoteProcessClient::read_vec error: len < 0, where len={}", len)));
        }
        self.read_vec_impl(len as usize, read)
    }

    fn read_vec_impl<T, F>(&mut self, len: usize, mut read: F) -> io::Result<Vec<T>>
        where F: FnMut(&mut Self) -> io::Result<T> {
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(read(self)?);
        }
        Ok(result)
    }

    #[inline]
    fn read_u8(&mut self) -> io::Result<(u8)> {
        self.reader.read_u8()
    }

    #[inline]
    fn read_i8(&mut self) -> io::Result<(i8)> {
        self.reader.read_i8()
    }

    #[inline]
    fn read_i32(&mut self) -> io::Result<(i32)> {
        self.reader.read_i32::<LittleEndian>()
    }

    #[inline]
    fn read_i64(&mut self) -> io::Result<(i64)> {
        self.reader.read_i64::<LittleEndian>()
    }

    #[inline]
    fn read_f64(&mut self) -> io::Result<(f64)> {
        self.reader.read_f64::<LittleEndian>()
    }

    fn write_message(&mut self, value: &Message) -> io::Result<()> {
        self.write_message_id(value.get_id())?;
        self.write_message_content(value)?;
        self.flush()
    }

    fn write_message_id(&mut self, value: i8) -> io::Result<()> {
        self.write_i8(value)
    }

    fn write_message_content(&mut self, value: &Message) -> io::Result<()> {
        match value {
            &Message::UnknownMessage => unimplemented!(),
            &Message::GameOver => unimplemented!(),
            &Message::AuthenticationToken(ref v) => self.write_authentication_token(v),
            &Message::TeamSize(_) => unimplemented!(),
            &Message::ProtocolVersion(v) => self.write_i32(v),
            &Message::GameContext(ref _v) => unimplemented!(),
            &Message::PlayerContext(ref _v) => unimplemented!(),
            &Message::ActionMessage(ref v) => self.write_action(v),
        }
    }

    fn write_authentication_token(&mut self, value: &String) -> io::Result<()> {
        self.write_i32(value.len() as i32)?;

        for b in value.bytes() {
            self.write_u8(b)?;
        }

        Ok(())
    }

    fn write_action(&mut self, value: &Action) -> io::Result<()> {
        self.write_bool(true)?;
        self.write_action_type(value.action)?;
        self.write_i32(value.group)?;
        self.write_f64(value.left)?;
        self.write_f64(value.top)?;
        self.write_f64(value.right)?;
        self.write_f64(value.bottom)?;
        self.write_f64(value.x)?;
        self.write_f64(value.y)?;
        self.write_f64(value.angle)?;
        self.write_f64(value.factor)?;
        self.write_f64(value.max_speed)?;
        self.write_f64(value.max_angular_speed)?;
        self.write_vehicle_type(value.vehicle_type)?;
        self.write_i64(value.facility_id)?;
        self.write_i64(value.vehicle_id)?;
        Ok(())
    }

    #[inline]
    fn write_action_type(&mut self, value: Option<ActionType>) -> io::Result<()> {
        self.write_option_enum(value)
    }

    #[inline]
    fn write_vehicle_type(&mut self, value: Option<VehicleType>) -> io::Result<()> {
        self.write_option_enum(value)
    }

    #[inline]
    fn write_option_enum<T: Into<i8>>(&mut self, value: Option<T>) -> io::Result<()> {
        if let Some(v) = value {
            self.write_i8(v.into())
        } else {
            self.write_i8(-1)
        }
    }

    #[inline]
    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.writer.write_u8(if value { 1 } else { 0 })
    }

    #[inline]
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.writer.write_u8(value)
    }

    #[inline]
    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        self.writer.write_i8(value)
    }

    #[inline]
    fn write_i32(&mut self, value: i32) -> io::Result<()> {
        self.writer.write_i32::<LittleEndian>(value)
    }

    #[inline]
    fn write_i64(&mut self, value: i64) -> io::Result<()> {
        self.writer.write_i64::<LittleEndian>(value)
    }

    #[inline]
    fn write_f64(&mut self, value: f64) -> io::Result<()> {
        self.writer.write_f64::<LittleEndian>(value)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        use std::io::Write;
        self.writer.flush()
    }
}

impl Into<i8> for ActionType {
    #[inline]
    fn into(self) -> i8 {
        self as i8
    }
}

impl Into<i8> for VehicleType {
    #[inline]
    fn into(self) -> i8 {
        self as i8
    }
}

pub trait Sealed {}

impl Sealed for LittleEndian {}

pub trait ByteOrder: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sealed {
    fn read_u32(buf: &[u8]) -> u32;
    fn read_u64(buf: &[u8]) -> u64;
    fn write_u32(buf: &mut [u8], n: u32);
    fn write_u64(buf: &mut [u8], n: u64);

    #[inline]
    fn read_i32(buf: &[u8]) -> i32 {
        Self::read_u32(buf) as i32
    }

    #[inline]
    fn read_i64(buf: &[u8]) -> i64 {
        Self::read_u64(buf) as i64
    }

    #[inline]
    fn read_f64(buf: &[u8]) -> f64 {
        safe_u64_bits_to_f64(Self::read_u64(buf))
    }

    #[inline]
    fn write_i32(buf: &mut [u8], n: i32) {
        Self::write_u32(buf, n as u32)
    }

    #[inline]
    fn write_i64(buf: &mut [u8], n: i64) {
        Self::write_u64(buf, n as u64)
    }

    #[inline]
    fn write_f64(buf: &mut [u8], n: f64) {
        Self::write_u64(buf, unsafe { transmute(n) })
    }
}

impl Message {
    pub fn get_id(&self) -> i8 {
        match *self {
            Message::UnknownMessage => 0,
            Message::GameOver => 1,
            Message::AuthenticationToken(_) => 2,
            Message::TeamSize(_) => 3,
            Message::ProtocolVersion(_) => 4,
            Message::GameContext(_) => 5,
            Message::PlayerContext(_) => 6,
            Message::ActionMessage(_) => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian default")
    }
}

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        assert!($size == ::core::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data.$which()
    });
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    });
}

impl ByteOrder for LittleEndian {
    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_le)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_le)
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_le);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_le);
    }
}

#[inline]
fn safe_u64_bits_to_f64(u: u64) -> f64 {
    use core::f64::NAN;

    const EXP_MASK: u64 = 0x7FF0000000000000;
    const FRACT_MASK: u64 = 0x000FFFFFFFFFFFFF;

    if u & EXP_MASK == EXP_MASK && u & FRACT_MASK != 0 {
        NAN
    } else {
        unsafe { transmute(u) }
    }
}

pub trait ReadBytesExt: io::Read {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    #[inline]
    fn read_i32<T: ByteOrder>(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(T::read_i32(&buf))
    }

    #[inline]
    fn read_i64<T: ByteOrder>(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(T::read_i64(&buf))
    }

    #[inline]
    fn read_f64<T: ByteOrder>(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(T::read_f64(&buf))
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}

pub trait WriteBytesExt: io::Write {
    #[inline]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n])
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> io::Result<()> {
        self.write_all(&[n as u8])
    }

    #[inline]
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> io::Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> io::Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf)
    }

    #[inline]
    fn write_f64<T: ByteOrder>(&mut self, n: f64) -> io::Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf)
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt for W {}
