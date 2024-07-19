use crate::config::Config;
use crate::error::SnipeError;
use anyhow::Result;
use colored::Colorize;
use solana_sdk::signer::Signer;
use solana_sdk::signer::{keypair::Keypair, EncodableKey};
use std::path::Path;

pub fn info(cfg: &Config) -> Result<()> {
    println!("{}: {}", "Keypair Path".green().bold(), cfg.keypair);
    println!("{}: {}", "RPC URL".green().bold(), cfg.rpc);

    let keypair = match Keypair::read_from_file(Path::new(cfg.keypair.as_str())) {
        Ok(k) => k,
        Err(_) => return Err(SnipeError::FailedToReadKeypair.into()),
    };

    println!("{}: {:?}", "Public Key".green().bold(), keypair.pubkey());
    Ok(())
}
