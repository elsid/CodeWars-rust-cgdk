extern crate argparse;
extern crate byteorder;

mod code_wars;

struct Args {
    host: String,
    port: u16,
    token: String,
}

fn main() {
    use std::io::{stderr, Write};
    use std::process::exit;
    use byteorder::LittleEndian;
    use code_wars::client::run;

    let mut args = Args {
        host: "127.0.0.1".to_string(),
        port: 31001,
        token: "0000000000000000".to_string(),
    };
    parse_args(&mut args);
    match run::<LittleEndian>(&args.host[..], args.port, args.token) {
        Ok(_) => (),
        Err(v) => {
            write!(&mut stderr(), "{:?}\n", v).unwrap();
            exit(-1);
        }
    }
}

fn parse_args(args: &mut Args) {
    use argparse::{ArgumentParser, Store};
    let mut parser = ArgumentParser::new();
    parser.refer(&mut args.host)
        .add_argument("host", Store, "Remote server host");
    parser.refer(&mut args.port)
        .add_argument("port", Store, "Remote server port");
    parser.refer(&mut args.token)
        .add_argument("token", Store, "Authorization token");
    parser.parse_args_or_exit();
}
