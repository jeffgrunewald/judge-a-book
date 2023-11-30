use judge_a_book::{Cmd, Result};

use clap::Parser;

const ENV_CHAIN_API_KEY: &str = "JUDGE_CHAIN_API_KEY";
const ENV_ASSET_API_KEY: &str = "JUDGE_ASSET_API_KEY";
const ENV_CARDANO_BASE_URL: &str = "JUDGE_CHAIN_URL";
const ENV_IPFS_BASE_URL: &str = "JUDGE_ASSETS_URL";

const CARDANO_BASE_URL_DEFAULT: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
const IPFS_BASE_URL_DEFAULT: &str = "https://ipfs.io/ipfs";

#[derive(Debug, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    name = env!("CARGO_BIN_NAME"),
    version,
    about,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,

    #[arg(
        global = true,
        long,
        env = ENV_CHAIN_API_KEY,
    )]
    chain_api_key: String,
    #[arg(
        global = true,
        long,
        env = ENV_ASSET_API_KEY,
    )]
    asset_api_key: Option<String>,
    #[arg(
        global = true,
        long,
        env = ENV_CARDANO_BASE_URL,
        default_value = CARDANO_BASE_URL_DEFAULT,
    )]
    chain_base_url: String,
    #[arg(
        global = true,
        long,
        env = ENV_IPFS_BASE_URL,
        default_value = IPFS_BASE_URL_DEFAULT,
    )]
    asset_base_url: String,
}

fn main() -> Result {
    let cli = Cli::parse();

    run(cli)
}

fn run(cli: Cli) -> Result {
    let result = match cli.cmd {
        Cmd::FetchCovers(cmd) => cmd.run(),
    };
    match result {
        Ok(()) => Ok(()),
        Err(error) => {
            let err = serde_json::json!({"status": "failure", "error": error.to_string()});
            println!("{err}");
            Ok(())
        }
    }
}
