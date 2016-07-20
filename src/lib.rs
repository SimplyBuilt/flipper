extern crate hyper;

#[macro_use]
extern crate log;

use hyper::{StatusCode, Decoder, Encoder, Next};
use hyper::header::{ContentLength, Location, Host};
use hyper::net::HttpStream;
use hyper::server::{Handler, Request, Response};

static NOT_FOUND: &'static [u8] = b"404 Not Found";

enum Route {
    NotFound,
    DomainFlip(String)
}

pub struct Flipper {
    route: Route,
}

impl Flipper {
    pub fn new() -> Flipper {
        Flipper {
            route: Route::NotFound,
        }
    }
}

impl Handler<HttpStream> for Flipper {
    fn on_request(&mut self, request: Request<HttpStream>) -> Next {
        debug!("{} {}", request.method(), request.uri());

        match request.headers().get::<Host>() {
            Some(host) => {
                debug!("Found host header: {:?}", host);

                match host.port {
                    None => {
                        let hostdots: Vec<&str> = host.hostname.matches(".").collect();

                        match hostdots.len() {
                            1 => {
                                self.route = Route::DomainFlip(format!("www.{}", host.hostname));
                            }

                            _ => debug!("Subdomain detected; Domain not flip")
                        }
                    }

                    Some(_) => debug!("Port detected; Domain not flipped"),
                }
            }

            None => debug!("Failed to get Host header")
        }

        Next::write()
    }

    fn on_request_readable(&mut self, _transport: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        match self.route {
            Route::DomainFlip(ref host) => {
                debug!("DomainFlip on_response for {}", host);

                response.set_status(StatusCode::PermanentRedirect);
                response.headers_mut().set(Location(host.to_owned()));

                Next::end()
            }

            Route::NotFound => {
                debug!("NotFound on_response");

                response.set_status(StatusCode::NotFound);
                response.headers_mut().set(ContentLength(NOT_FOUND.len() as u64));

                Next::write()
            }
        }
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.route {
            Route::NotFound => {
                transport.write(NOT_FOUND).unwrap();

                Next::end()
            }

            _ => unreachable!()
        }
    }
}
