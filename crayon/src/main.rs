use crate::constant::CRAYON;
use tracing::info;

mod constant;
mod parameter;
mod server;

#[tokio::main]
async fn main() {
    let parameters = parameter::parse();
    tracing_subscriber::fmt::init();
    info!("Running {CRAYON } {parameters:?}.");
    server::run_server(
        parameters.port,
        parameters.resource_root,
        parameters.home_page,
    )
    .await;
}
