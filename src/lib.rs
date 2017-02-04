extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;

use hyper::StatusCode;
use hyper::header::{ContentLength, ContentType, Location, Host};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::server::{Service, Request, Response};

static NOT_FOUND: &'static [u8] = b"404 Not Found";

pub struct Flipper;

fn flipped_host_response(hostname: &str) -> Response {
    let flipped_host = format!("www.{}", hostname);

    Response::new()
        .with_status(StatusCode::PermanentRedirect)
        .with_header(Location(flipped_host))
}

fn not_found_response() -> Response {
    Response::new()
        .with_status(StatusCode::NotFound)
        .with_header(ContentLength(NOT_FOUND.len() as u64))
        .with_header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])))
        .with_body(NOT_FOUND)
}

impl Service for Flipper {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;

    fn call(&self, request: Request) -> Self::Future {
        debug!("{} {}", request.method(), request.uri());

        ::futures::finished(match request.headers().get::<Host>() {
            Some(host) => {
                match host.port() {
                    None => {
                        let hostname = host.hostname();
                        let hostdots: Vec<&str> = hostname.matches(".").collect();

                        match hostdots.len() {
                            1 => {
                                debug!("Flipping Host {} to www subodomain", host);

                                flipped_host_response(hostname)
                            }

                            _ => {
                                debug!("Subdomain detected; Host name not flipped");

                                not_found_response()

                            }
                        }
                    }

                    Some(_) => {
                        debug!("Port detected; Host name not flipped");

                        not_found_response()
                    }
                }
            }

            None => {
                debug!("Failed to get Host header");

                not_found_response()
            }
        })
    }
}

#[test]
fn test_flipped_host_response(){
    let resp = flipped_host_response("example.com");
    let location = resp.headers().get::<Location>().unwrap();

    assert_eq!(StatusCode::PermanentRedirect, resp.status().clone());
    assert_eq!("www.example.com", location.as_str());
}

#[test]
fn test_not_found_response(){
    let resp = not_found_response();
    let headers = resp.headers();

    let content_length = headers.get::<ContentLength>().unwrap();
    let content_type = headers.get::<ContentType>().unwrap();

    assert_eq!(StatusCode::NotFound, resp.status().clone());
    assert_eq!(13 as u64, content_length.0);
}
