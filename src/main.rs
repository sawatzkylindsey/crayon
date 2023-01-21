use axum::{routing::get, Router};
use blarg::{CommandLineParser, Parameter, Scalar, Switch};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const CRAYON: &str = "crayon";

#[tokio::main]
async fn main() {
    let (verbose, port) = parse_arguments();

    println!("Running {CRAYON} at 'localhost:{port}' (verbose={verbose}).");

    run_server(port).await;
}

fn parse_arguments() -> (bool, u16) {
    let mut verbose: bool = false;
    let mut port: u16 = 0;
    let clp = CommandLineParser::new(CRAYON);
    let parser = clp
        .add(Parameter::option(
            Switch::new(&mut verbose, true),
            "verbose",
            Some('v'),
        ))
        .add(Parameter::argument(Scalar::new(&mut port), "port"))
        .build()
        .expect("Invalid argument parser configuration");
    parser.parse();
    (verbose, port)
}

async fn run_server(port: u16) {
    let application = Router::new().route("/", get(|| async { format!("Welcome to {CRAYON}!") }));
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    axum::Server::bind(&address)
        .serve(application.into_make_service())
        .await
        .unwrap();
}
