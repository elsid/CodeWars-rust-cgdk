use std::collections::HashMap;
use std::io;
use core::fmt::Debug;
use core::hash::Hash;
use core::mem::transmute;
use core::ptr::copy_nonoverlapping;
use model::{
    ActionType,
    Facility,
    FacilityType,
    Game,
    Move,
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

pub fn run<'r, B: ByteOrder>(host: &'r str, port: u16, token: String) -> io::Result<()> {
    use std::collections::HashMap;
    use std::io::{BufReader, BufWriter, Error, ErrorKind};
    use std::net::TcpStream;
    use model::Move;
    use my_strategy::MyStrategy;
    use strategy::Strategy;

    let stream = TcpStream::connect((host, port))?;

    stream.set_nodelay(true)?;

    let mut writer = BufWriter::new(stream.try_clone()?);

    writer.write_message::<B>(&Message::AuthenticationToken(token.clone()))?;
    writer.write_message::<B>(&Message::ProtocolVersion(PROTOCOL_VERSION))?;

    let mut cache = Cache {
        facilities: HashMap::new(),
        players: HashMap::new(),
        terrain_by_cell_x_y: vec![],
        weather_by_cell_x_y: vec![],
    };

    let mut reader = BufReader::new(stream);

    let team_size = match reader.read_message::<B>(&mut cache)? {
        Message::TeamSize(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::TeamSize, but received: {:?}", v))),
    };

    if team_size < 0 {
        return Err(Error::new(ErrorKind::Other, format!("Team size < 0: {}", team_size)));
    }

    let game = match reader.read_message::<B>(&mut cache)? {
        Message::GameContext(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::GameContext, but received: {:?}", v))),
    };

    let mut strategy = MyStrategy::default();

    loop {
        let player_context = match reader.read_message::<B>(&mut cache)? {
            Message::GameOver => break,
            Message::PlayerContext(v) => v,
            v => return Err(Error::new(ErrorKind::Other,
                                       format!("Expected Message::GameOver, \
                                                Message::PlayerContext or \
                                                Message::PlayerContextWithoutTrees, but \
                                                received: {:?}", v)))
        };

        let mut move_ = Move::default();
        strategy.move_(&player_context.player, &player_context.world, &game, &mut move_);
        writer.write_message::<B>(&Message::MoveMessage(move_))?;
    }

    Ok(())
}

pub struct Cache {
    pub facilities: HashMap<i64, Facility>,
    pub players: HashMap<i64, Player>,
    pub terrain_by_cell_x_y: Vec<Vec<TerrainType>>,
    pub weather_by_cell_x_y: Vec<Vec<WeatherType>>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Message {
    UnknownMessage,
    GameOver,
    AuthenticationToken(String),
    TeamSize(i32),
    ProtocolVersion(i32),
    GameContext(Game),
    PlayerContext(PlayerContext),
    MoveMessage(Move),
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
            Message::MoveMessage(_) => 7,
        }
    }
}

pub trait ReadMessage: ReadBytesExt {
    fn read_message<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<Message> {
        use std::io::{Error, ErrorKind};
        match self.read_message_id()? {
            0 => unimplemented!(),
            1 => self.read_message_game_over(),
            2 => unimplemented!(),
            3 => self.read_message_team_size::<B>(),
            4 => unimplemented!(),
            5 => self.read_message_game_context::<B>(),
            6 => self.read_message_player_context::<B>(cache),
            7 => unimplemented!(),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_message error: invalid message id: {}", v)))
        }
    }

    fn read_message_id(&mut self) -> io::Result<i8> {
        self.read_i8()
    }

    fn read_message_game_over(&mut self) -> io::Result<Message> {
        Ok(Message::GameOver)
    }

    fn read_message_team_size<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::TeamSize(self.read_i32::<B>()?))
    }

    fn read_message_game_context<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::GameContext(self.read_game::<B>()?))
    }

    fn read_message_player_context<B: ByteOrder>(&mut self, cache: &mut Cache) -> io::Result<Message> {
        Ok(Message::PlayerContext(self.read_player_context::<B>(cache)?))
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

        let result = Vehicle {
            id: self.read_i64::<B>()?,
            x: self.read_f64::<B>()?,
            y: self.read_f64::<B>()?,
            radius: self.read_f64::<B>()?,
            player_id: self.read_i64::<B>()?,
            durability: self.read_i32::<B>()?,
            max_durability: self.read_i32::<B>()?,
            max_speed: self.read_f64::<B>()?,
            vision_range: self.read_f64::<B>()?,
            squared_vision_range: self.read_f64::<B>()?,
            ground_attack_range: self.read_f64::<B>()?,
            squared_ground_attack_range: self.read_f64::<B>()?,
            aerial_attack_range: self.read_f64::<B>()?,
            squared_aerial_attack_range: self.read_f64::<B>()?,
            ground_damage: self.read_i32::<B>()?,
            aerial_damage: self.read_i32::<B>()?,
            ground_defence: self.read_i32::<B>()?,
            aerial_defence: self.read_i32::<B>()?,
            attack_cooldown_ticks: self.read_i32::<B>()?,
            remaining_attack_cooldown_ticks: self.read_i32::<B>()?,
            type_: self.read_vehicle_type()?,
            aerial: self.read_bool()?,
            selected: self.read_bool()?,
            groups: self.read_vec_i32::<B>()?,
        };

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
            0 => Ok(VehicleType::Arrv),
            1 => Ok(VehicleType::Fighter),
            2 => Ok(VehicleType::Helicopter),
            3 => Ok(VehicleType::Ifv),
            4 => Ok(VehicleType::Tank),
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

pub trait WriteMessage: WriteBytesExt {
    fn write_message<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        self.write_message_id(value.get_id())?;
        self.write_message_content::<B>(value)
    }

    fn write_message_id(&mut self, value: i8) -> io::Result<()> {
        self.write_i8(value)
    }

    fn write_message_content<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        match value {
            &Message::UnknownMessage => unimplemented!(),
            &Message::GameOver => unimplemented!(),
            &Message::AuthenticationToken(ref v) => self.write_authentication_token::<B>(v)?,
            &Message::TeamSize(v) => self.write_i32::<B>(v)?,
            &Message::ProtocolVersion(v) => self.write_i32::<B>(v)?,
            &Message::GameContext(ref _v) => unimplemented!(),
            &Message::PlayerContext(ref _v) => unimplemented!(),
            &Message::MoveMessage(ref v) => self.write_move::<B>(v)?,
        }
        self.flush()
    }

    fn write_authentication_token<B: ByteOrder>(&mut self, value: &String) -> io::Result<()> {
        self.write_i32::<B>(value.len() as i32)?;

        for b in value.bytes() {
            self.write_u8(b)?;
        }

        Ok(())
    }

    fn write_move<B: ByteOrder>(&mut self, value: &Move) -> io::Result<()> {
        self.write_bool(true)?;
        self.write_action_type(value.action)?;
        self.write_i32::<B>(value.group)?;
        self.write_f64::<B>(value.left)?;
        self.write_f64::<B>(value.top)?;
        self.write_f64::<B>(value.right)?;
        self.write_f64::<B>(value.bottom)?;
        self.write_f64::<B>(value.x)?;
        self.write_f64::<B>(value.y)?;
        self.write_f64::<B>(value.angle)?;
        self.write_f64::<B>(value.factor)?;
        self.write_f64::<B>(value.max_speed)?;
        self.write_f64::<B>(value.max_angular_speed)?;
        self.write_vehicle_type(value.vehicle_type)?;
        self.write_i64::<B>(value.facility_id)?;
        self.write_i64::<B>(value.vehicle_id)?;
        Ok(())
    }

    fn write_action_type(&mut self, value: ActionType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_vehicle_type(&mut self, value: VehicleType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(if value { 1 } else { 0 })
    }
}

impl<W: WriteBytesExt> WriteMessage for W {}

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

#[test]
fn test_read_bool() {
    use std::io::Cursor;
    assert_eq!(Cursor::new(vec![0u8]).read_bool().unwrap(), false);
    assert_eq!(Cursor::new(vec![1u8]).read_bool().unwrap(), true);
    assert_eq!(Cursor::new(vec![255u8]).read_bool().unwrap(), true);
}

#[test]
fn test_read_vec_i8() {
    use std::io::Cursor;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, -42i8 as u8])
        .read_vec_i8::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i8, -42i8]);
}

#[test]
fn test_read_vec_i32() {
    use std::io::Cursor;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8])
        .read_vec_i32::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i32, 13i32]);
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

#[test]
fn test_write_bool_false() {
    let mut buffer = vec![];
    buffer.write_bool(false).unwrap();
    assert_eq!(buffer, vec![0u8]);
}

#[test]
fn test_write_bool_true() {
    let mut buffer = vec![];
    buffer.write_bool(true).unwrap();
    assert_eq!(buffer, vec![1u8]);
}

#[test]
fn test_write_message_authentication_token() {
    let message = Message::AuthenticationToken("foo".to_string());
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        2u8,
        3u8, 0u8, 0u8, 0u8,
        102u8, 111u8, 111u8,
    ]);
}

#[test]
fn test_write_message_protocol_version() {
    let message = Message::ProtocolVersion(42);
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        4u8,
        42u8, 0u8, 0u8, 0u8,
    ]);
}

#[test]
fn test_write_message_move() {
    let move_ = Move {
        action: ActionType::ClearAndSelect,
        group: 1,
        left: 2.0,
        top: 3.0,
        right: 4.0,
        bottom: 5.0,
        x: 6.0,
        y: 7.0,
        angle: 8.0,
        factor: 9.0,
        max_speed: 10.0,
        max_angular_speed: 11.0,
        vehicle_type: VehicleType::Tank,
        facility_id: 12,
        vehicle_id: 13,
    };
    let message = Message::MoveMessage(move_);
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        7u8,
        1u8,
        1u8,
        1u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 24u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 28u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 38u8, 64u8,
        4u8,
        12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    ]);
}
