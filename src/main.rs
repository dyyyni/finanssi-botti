mod bot;
mod open_banking;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    bot::run_bot().await;
}