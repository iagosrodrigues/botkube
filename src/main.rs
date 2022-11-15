mod lib;
use env_logger::Env;
use lib::Botkube;
use log::{error, info};
use std::{io::ErrorKind, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Iniciando botkube...");

    let bot = Botkube::load_config().unwrap_or_else(|error| {
        let error_message = match error.kind() {
            ErrorKind::NotFound => "Arquivo de configuracao nao encontrado...",
            _ => "Erro nao tratado",
        };
        error!("{}", error_message);
        process::exit(1);
    });

    bot.run();

    Ok(())
}
