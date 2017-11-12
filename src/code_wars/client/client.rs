use std::io;
use byteorder::ByteOrder;

const PROTOCOL_VERSION: i32 = 3;

pub fn run<'r, B: ByteOrder>(host: &'r str, port: u16, token: String) -> io::Result<()> {
    use std::collections::HashMap;
    use std::io::{Error, ErrorKind};
    use std::net::TcpStream;
    use code_wars::model::Move;
    use code_wars::MyStrategy;
    use super::cache::Cache;
    use super::message::Message;
    use super::read_message::ReadMessage;
    use super::write_message::WriteMessage;

    let mut stream = TcpStream::connect((host, port))?;

    stream.set_nodelay(true)?;

    stream.write_message::<B>(&Message::AuthenticationToken(token.clone()))?;
    stream.write_message::<B>(&Message::ProtocolVersion(PROTOCOL_VERSION))?;

    let mut cache = Cache {
        facilities: HashMap::new(),
        players: HashMap::new(),
        terrain_by_cell_x_y: vec![],
        weather_by_cell_x_y: vec![],
    };

    let team_size = match stream.read_message::<B>(&mut cache)? {
        Message::TeamSize(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::TeamSize, but received: {:?}", v))),
    };

    if team_size < 0 {
        return Err(Error::new(ErrorKind::Other, format!("Team size < 0: {}", team_size)));
    }

    let game = match stream.read_message::<B>(&mut cache)? {
        Message::GameContext(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::GameContext, but received: {:?}", v))),
    };

    let mut strategy = MyStrategy::new();

    loop {
        let player_context = match stream.read_message::<B>(&mut cache)? {
            Message::GameOver => break,
            Message::PlayerContext(v) => v,
            v => return Err(Error::new(ErrorKind::Other,
                                       format!("Expected Message::GameOver, \
                                                Message::PlayerContext or \
                                                Message::PlayerContextWithoutTrees, but \
                                                received: {:?}", v)))
        };

        let mut move_ = Move::new();
        strategy.move_(&player_context.player, &player_context.world, &game, &mut move_);
        stream.write_message::<B>(&Message::MoveMessage(move_))?;
    }

    Ok(())
}
