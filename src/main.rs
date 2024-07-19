use anyhow::{Context, Result};
use clap::{Arg, Command};
use std::io::{self, ErrorKind};

mod config;
mod error;
mod handlers;

use config::Config;

const DEFAULT_CONFIG: (&str, &str, &str) = (
    "localhost",
    "http://localhost:8899",
    "~/.config/solana/id.json",
);

fn main() -> Result<()> {
    // parse cli
    let matches = Command::new("snipe-cli")
        .version("0.1.0")
        .about("snipe buy/sell raydium engine")
        .arg_required_else_help(true)
        .arg(
            Arg::new("RPC-URL")
                .help("Set RPC url")
                .long("rpc-url")
                .short('r')
                .default_value(DEFAULT_CONFIG.1),
        )
        .arg(
            Arg::new("KEYPAIR")
                .help("Set keypair path")
                .long("keypair")
                .short('k')
                .default_value(DEFAULT_CONFIG.2),
        )
        .subcommand(Command::new("init").about("Initialize snipe engine"))
        .subcommand(Command::new("info").about("Get config info"))
        .subcommand(
            Command::new("keygen")
                .about("Generate new keypair")
                .arg(
                    Arg::new("PATH")
                        .help("Where to store new keypair secret")
                        .long("path")
                        .short('p')
                        .default_value("."),
                )
                .arg(
                    Arg::new("NAME")
                        .help("Filename of new keypair")
                        .long("name")
                        .short('n')
                        .default_value("id.json"),
                ),
        )
        .subcommand(
            Command::new("buy").about("Buy token").arg(
                Arg::new("MINT")
                    .help("mint address of token")
                    .long("mint")
                    .short('m')
                    .index(0)
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("sell").about("Sell token").arg(
                Arg::new("MINT")
                    .help("mint address of token")
                    .long("mint")
                    .short('m')
                    .index(0)
                    .required(true),
            ),
        )
        .get_matches();

    // load config
    // let config = Config::from_args(&matches);
    // get config
    // let config = config::Config::get_config().with_context(|| "failed to get config")?;

    // match subcommand
    if matches.subcommand_matches("init").is_some() {
        // INIT
        handlers::init()?;
    } else if matches.subcommand_matches("info").is_some() {
        // get config
        let config = config::Config::get_config().with_context(|| "failed to get config")?;
        handlers::info(&config)?;
    } else if matches.subcommand_matches("keygen").is_some() {
        // get config
        let config = config::Config::get_config().with_context(|| "failed to get config")?;
        handlers::keygen(&config)?;
    } else if matches.subcommand_matches("buy").is_some() {
        // BUY
    } else if matches.subcommand_matches("sell").is_some() {
        // SELL
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn sanity_check() {
        assert_eq!(true, true);
    }
}
