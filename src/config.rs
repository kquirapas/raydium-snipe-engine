use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};
use yaml_rust2::{YamlEmitter, YamlLoader};

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Config {
    pub keypair: String,
    pub rpc: String,
}

impl Config {
    const CONFIG_PATH: &str = "./snipe.yaml";
    const DEFAULT_KEYPAIR_PATH: &str = "./snipe.json";
    const DEFAULT_RPC: &str = "https://api.mainnet-beta.solana.com";

    pub fn get_config() -> Result<Config> {
        let contents = fs::read_to_string(Self::CONFIG_PATH)?;
        let docs = YamlLoader::load_from_str(contents.as_str())?;
        let first_doc = &docs[0];

        Ok(Config {
            keypair: first_doc["keypair"]
                .clone()
                .into_string()
                .expect("invalid keypair"),
            rpc: first_doc["rpc"].clone().into_string().expect("invalid rpc"),
        })
    }

    pub fn create_config() -> Result<()> {
        let input_string = format!(
            "keypair: {}\nrpc: {}",
            Self::DEFAULT_KEYPAIR_PATH,
            Self::DEFAULT_RPC
        );
        let yaml = YamlLoader::load_from_str(&input_string)?;

        let mut output = String::new();
        YamlEmitter::new(&mut output).dump(&yaml[0])?;

        // write to config path
        fs::write(Path::new(Self::CONFIG_PATH), output)?;

        Ok(())
    }
}
