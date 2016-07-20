#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate flipper;

use std::thread;

use hyper::net::{HttpListener};
use hyper::server::{Server};

use flipper::Flipper;

docopt!(Args derive Debug, "
flipper - HTTP domain name flipper, powered by hyper.

Usage:
  esper [--bind=<bind>] [--port=<port>] [--threads=<st>]
  esper (-h | --help)
  esper (-v | --version)

Options:
  -h --help          Show this screen.
  -v --version       Show version.
  -b --bind=<bind>   Bind to specific IP [default: 127.0.0.1]
  -p --port=<port>   Run on a specific port number [default: 3000]
  -t --threads=<st>  Number of server threads [default: 2].
", flag_threads: u8);

fn main() {
    env_logger::init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    debug!("Executing with args: {:?}", args);

    if args.flag_version {
        println!("flipper v0.1.0");
        std::process::exit(0);
    }

    let addr = format!("{}:{}", args.flag_bind, args.flag_port);
    let listener = HttpListener::bind(&addr.parse().unwrap()).unwrap();

    let mut handles = Vec::new();

    for _ in 0..args.flag_threads {
        let listener = listener.try_clone().unwrap();

        handles.push(thread::spawn(move || {
            Server::new(listener).handle(|_| Flipper::new()).unwrap()
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
