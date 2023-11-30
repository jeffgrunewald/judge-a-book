use crate::{cmd::Resolution, Result};

mod response_types;

use rayon::prelude::*;
use response_types::*;
use std::io::prelude::*;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const BOOKIO_COLLECTION_URL: &str = "https://api.book.io/api/v0/collections";

pub struct Client {
    chain_key: String,
    asset_key: String,
    asset_url: String,
    chain_url: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(
        chain_key: impl AsRef<str>,
        asset_key: impl AsRef<str>,
        asset_url: impl AsRef<str>,
        chain_url: impl AsRef<str>,
    ) -> Result<Self> {
        Ok(Self {
            chain_key: chain_key.as_ref().to_string(),
            asset_key: asset_key.as_ref().to_string(),
            asset_url: asset_url.as_ref().to_string(),
            chain_url: chain_url.as_ref().to_string(),
            client: reqwest::blocking::Client::builder()
                .user_agent(USER_AGENT)
                .build()?,
        })
    }

    pub fn is_valid_collection(&self, id: impl AsRef<str>) -> Result<bool> {
        let response = self.client.get(BOOKIO_COLLECTION_URL).send()?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let collections = response.json::<BookCollectionResp>()?.data;
                Ok(collections
                    .into_iter()
                    .any(|ref collection| collection.collection_id == id.as_ref()))
            }
            error => {
                anyhow::bail!(error.to_string())
            }
        }
    }

    pub fn get_assets_covers(
        &self,
        id: impl AsRef<str>,
        existing_covers: &[String],
        count: u32,
        res: Resolution,
        prefix: String,
    ) -> Result<Vec<String>> {
        let id = id.as_ref().to_string();
        let response = self
            .client
            .get(format!("{}/assets/policy/{}", &self.chain_url, &id))
            .header("project_id", &self.chain_key)
            .send()?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let assets = response
                    .json::<AssetList>()?
                    .0
                    .into_iter()
                    .map(|asset| asset.asset)
                    .collect::<Vec<String>>();
                let downloaded = assets
                    .into_par_iter()
                    .map(|asset| self.get_asset_cid(asset, res))
                    .filter(|maybe_cid| {
                        if let Some(cid) = maybe_cid {
                            !existing_covers.contains(cid)
                        } else {
                            false
                        }
                    })
                    .map(|cid| cid.unwrap())
                    .take_any(count as usize)
                    .map(|cid| self.download_asset_cover(cid, prefix.clone()))
                    .collect::<Result<Vec<_>>>()?;

                Ok(downloaded)
            }
            error => {
                anyhow::bail!(error.to_string())
            }
        }
    }

    fn get_asset_cid(&self, id: impl AsRef<str>, res: Resolution) -> Option<String> {
        if let Ok(response) = self
            .client
            .get(format!("{}/assets/{}", &self.chain_url, id.as_ref()))
            .header("project_id", &self.chain_key)
            .send()
        {
            if response.status() == reqwest::StatusCode::OK {
                response
                    .json::<AssetMetadataResp>()
                    .ok()
                    .map(|asset_metadata| {
                        if res == Resolution::High {
                            extract_hi_res_cid(asset_metadata)
                        } else {
                            extract_lo_res_cid(asset_metadata)
                        }
                    })
                    .unwrap_or_default()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn download_asset_cover(&self, cid: impl AsRef<str>, prefix: String) -> Result<String> {
        let response = self
            .client
            .get(format!("{}/{}", &self.asset_url, cid.as_ref()))
            .header("project_id", &self.asset_key)
            .send()?;
        match response.status() {
            reqwest::StatusCode::OK => {
                let bytes = response.bytes()?.to_vec();
                let file_name = prefix + cid.as_ref() + ".png";
                let mut file = std::fs::File::create(&file_name)?;
                file.write_all(&bytes)?;
                Ok(file_name)
            }
            error => anyhow::bail!(error.to_string()),
        }
    }
}

fn extract_hi_res_cid(asset: AssetMetadataResp) -> Option<String> {
    asset
        .onchain_metadata
        .files
        .into_iter()
        .filter(|file| file.is_high_res_image())
        .map(|file| file.get_cid().ok_or(anyhow::anyhow!("asset not found")))
        .collect::<Result<Vec<_>>>()
        .unwrap_or_default()
        .first()
        .cloned()
}

fn extract_lo_res_cid(asset: AssetMetadataResp) -> Option<String> {
    asset
        .onchain_metadata
        .image
        .strip_prefix("ipfs://")
        .map(|image| image.to_string())
}
