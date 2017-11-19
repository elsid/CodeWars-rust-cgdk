extern crate core;

mod model;
mod my_strategy;
mod remote_process_client;
mod strategy;

use std::io;
use remote_process_client::RemoteProcessClient;
use strategy::Strategy;

struct Args {
    host: String,
    port: u16,
    token: String,
}

fn main() {
    use std::io::{stderr, Write};
    use std::process::exit;
    use my_strategy::MyStrategy;

    let args = parse_args();

    let client = match RemoteProcessClient::connect(&args.host[..], args.port) {
        Ok(v) => v,
        Err(v) => {
            write!(&mut stderr(), "{:?}\n", v).unwrap();
            exit(-1);
        }
    };

    let mut runner = Runner::new(client, args.token);

    match runner.run::<MyStrategy>() {
        Ok(_) => (),
        Err(v) => {
            write!(&mut stderr(), "{:?}\n", v).unwrap();
            exit(-1);
        }
    }
}

fn parse_args() -> Args {
    if std::env::args().count() == 4 {
        Args {
            host: std::env::args().nth(1).unwrap(),
            port: std::env::args().nth(2).unwrap().parse().expect("Cant't parse port"),
            token: std::env::args().nth(3).unwrap(),
        }
    } else {
        Args {
            host: "127.0.0.1".to_string(),
            port: 31001,
            token: "0000000000000000".to_string(),
        }
    }
}

struct Runner {
    client: RemoteProcessClient,
    token: String,
}

impl Runner {
    pub fn new(client: RemoteProcessClient, token: String,) -> Self {
        Runner { client, token }
    }

    pub fn run<T: Strategy>(&mut self) -> io::Result<()> {
        use model::Action;

        self.client.write_authentication_token_message(self.token.clone())?;
        self.client.write_protocol_version_message()?;
        self.client.read_team_size_message()?;
        let game = self.client.read_game_message()?;
        let mut strategy = T::default();

        while let Some(player_context) = self.client.read_player_context_message()? {
            let mut action = Action::default();
            strategy.act(&player_context.player, &player_context.world, &game, &mut action);
            self.client.write_action_message(action)?;
        }

        Ok(())
    }
}
