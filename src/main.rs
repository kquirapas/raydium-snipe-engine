use anyhow::Result;
use clap::{Arg, Command};

mod config;
mod error;

use config::{ClusterConfig, Config};

const DEFAULT_CONFIG: (&str, &str, &str) = (
    "localhost",
    "http://localhost:8899",
    "~/.config/solana/id.json",
);

fn main() -> Result<()> {
    // parse cli
    let matches = Command::new("snipe-cli")
        .version("0.1.0")
        .about("zaiken snipe buy/sell raydium engine")
        .arg_required_else_help(true)
        .arg(
            Arg::new("CLUSTER")
                .help("set cluster")
                .long("cluster")
                .short('c')
                .value_parser(clap::builder::EnumValueParser::<ClusterConfig>::new())
                .default_value(DEFAULT_CONFIG.0),
        )
        .arg(
            Arg::new("RPC-URL")
                .help("set RPC url")
                .long("rpc-url")
                .short('r')
                .default_value(DEFAULT_CONFIG.1),
        )
        .arg(
            Arg::new("KEYPAIR")
                .help("set keypair path")
                .long("keypair")
                .short('k')
                .default_value(DEFAULT_CONFIG.2),
        )
        .subcommand(Command::new("info").about("Get config info"))
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
    let config = Config::from_args(&matches);

    // match subcommand
    if matches.subcommand_matches("info").is_some() {
        // INFO
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
