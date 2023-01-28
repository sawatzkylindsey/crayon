use crate::constant::CRAYON;

mod constant;
mod parameter;
mod server;

#[tokio::main]
async fn main() {
    let parameters = parameter::parse();
    println!("Running {CRAYON} {parameters:?}.");
    server::run_server(parameters.port, parameters.resource_root).await;
}
