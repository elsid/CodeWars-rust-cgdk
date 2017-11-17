extern crate core;

mod model;
mod my_strategy;
mod remote_process_client;
mod strategy;

struct Args {
    host: String,
    port: u16,
    token: String,
}

fn main() {
    use std::io::{stderr, Write};
    use std::process::exit;
    use remote_process_client::{LittleEndian, run};

    let args = parse_args();
    match run::<LittleEndian>(&args.host[..], args.port, args.token) {
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
