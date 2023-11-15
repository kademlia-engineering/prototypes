/*
Last updated: 11-15-2023

Description: This crate defines the web server class using tokio and hyper

Author: James Dean
*/
use hyper::{Method, Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use config::VERSION;

// This method defines the rest api routes exposed by the codebase
// Params: HTTP Request
// Returns: HTTP Response
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(VERSION))),
        (&Method::GET, "/transaction") => Ok(Response::new(Body::from("This is a GET request"))),
        (&Method::POST, "/transaction") => Ok(Response::new(Body::from("This is a POST request"))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// This method initializes the web server
// Params: Port
pub async fn run_server(port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}