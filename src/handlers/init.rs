use crate::config::Config;
use anyhow::{Context, Result};

pub fn init() -> Result<()> {
    // overwrite or create config
    Config::create_config().with_context(|| "failed to create config")?;
    println!("Successfully created config.");
    Ok(())
}
