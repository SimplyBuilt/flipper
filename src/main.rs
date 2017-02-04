#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate flipper;

use hyper::server::Http;

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
");

fn main() {
    env_logger::init().unwrap();

    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    debug!("Executing with args: {:?}", args);

    if args.flag_version {
        println!("flipper v0.2.0");
        std::process::exit(0);
    }

    let addr = format!("{}:{}", args.flag_bind, args.flag_port).parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Flipper)).unwrap();

    info!("Listening on http://{}", server.local_addr().unwrap());

    server.run().unwrap();
}
