use clap::{ArgMatches, ValueEnum};

#[derive(Clone, Default, Eq, PartialEq, ValueEnum)]
pub enum ClusterConfig {
    #[default]
    Localhost,
    Devnet,
    Mainnet,
}

pub struct Config {
    cluster: ClusterConfig,
    keypair: String,
    rpc: String,
}

impl Config {
    pub fn from_args(matches: &ArgMatches) -> Self {
        Self {
            // safe to unwrap due to argument .default()
            cluster: matches
                .get_one::<ClusterConfig>("CLUSTER")
                .unwrap()
                .to_owned(),
            keypair: matches.get_one::<String>("KEYPAIR").unwrap().to_string(),
            rpc: matches.get_one::<String>("RPC-URL").unwrap().to_string(),
        }
    }
}
