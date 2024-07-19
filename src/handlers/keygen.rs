use crate::config::Config;
use crate::error::SnipeError;
use anyhow::Result;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::EncodableKey;
use std::path::Path;

pub fn keygen(cfg: &Config) -> Result<()> {
    let keypair_path = Path::new(cfg.keypair.as_str());
    let keypair = Keypair::new();

    if keypair.write_to_file(keypair_path).is_err() {
        return Err(SnipeError::FailedToWriteKeypair.into());
    }

    Ok(())
}
