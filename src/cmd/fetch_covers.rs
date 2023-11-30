use crate::{client::Client, Result};

use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct FetchCoversArgs {
    /// The collection of books to retrieve, identified by Policy ID
    #[arg(short, long)]
    collection: String,
    /// The location to output the retrieved images to
    #[arg(short, long)]
    outdir: PathBuf,
    /// The number of books for which to retrieve cover images; default = 10
    #[arg(short, long, default_value = "10")]
    number: u32,
    /// The resolution for images retrieved; default = "high"
    #[arg(short, long, value_enum, default_value = "high")]
    res: Resolution,
    /// The key for querying the blockchain API
    #[arg(from_global)]
    chain_api_key: String,
    /// The key for querying/retrieving IPFS assets
    #[arg(from_global)]
    asset_api_key: Option<String>,
    /// The base url for querying the blockchain API
    #[arg(from_global)]
    chain_base_url: String,
    /// The base url for retrieving image assets
    #[arg(from_global)]
    asset_base_url: String,
}

#[derive(Debug, clap::ValueEnum, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Resolution {
    #[value(alias("hi"))]
    High,
    #[value(alias("lo"))]
    Low,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl FetchCoversArgs {
    pub fn run(self) -> Result {
        let collection_id = self.collection;
        let resolution = self.res;
        let cover_count = self.number;
        let outdir = self.outdir;
        let prefix = format!(
            "{}/{}-{}-",
            outdir.to_string_lossy().trim_end_matches('/'),
            collection_id,
            resolution
        );
        std::fs::create_dir_all(&outdir)?;
        let existing_covers = std::fs::read_dir(outdir)?
            .filter_map(|path| {
                if let Ok(file) = path {
                    file.path()
                        .to_str()
                        .map(|file| file.strip_prefix(&prefix).map(|file| file.to_string()))
                } else {
                    None
                }
            })
            .flatten()
            .map(|file| file.trim_end_matches(".png").to_string())
            .collect::<Vec<_>>();
        let covers_to_download = cover_count as usize - existing_covers.len();
        if covers_to_download == 0 {
            let early_result = serde_json::json!({"status": "success", "files_written": []});
            println!("{early_result}");
            return Ok(());
        }

        let client = Client::new(
            &self.chain_api_key,
            self.asset_api_key.unwrap_or_default(),
            &self.asset_base_url,
            &self.chain_base_url,
        )?;
        if client.is_valid_collection(&collection_id)? {
            let files = client.get_assets_covers(
                &collection_id,
                &existing_covers,
                covers_to_download as u32,
                resolution,
                prefix.clone(),
            )?;
            let result = serde_json::json!({"status": "success", "files_written": files});
            println!("{result}");
        }
        Ok(())
    }
}
