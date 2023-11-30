use serde::Deserialize;

#[derive(Deserialize)]
pub struct BookCollection {
    pub collection_id: String,
}

#[derive(Deserialize)]
pub struct BookCollectionResp {
    pub data: Vec<BookCollection>,
}

#[derive(Deserialize)]
pub struct Asset {
    pub asset: String,
}

#[derive(Deserialize)]
pub struct AssetList(pub(super) Vec<Asset>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataFile {
    pub name: String,
    pub src: String,
}

impl MetadataFile {
    pub fn is_high_res_image(&self) -> bool {
        &self.name == "High-Res Cover Image"
    }

    pub fn get_cid(&self) -> Option<String> {
        AsRef::<str>::as_ref(&self.src)
            .strip_prefix("ipfs://")
            .map(|cid| cid.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct OnChainMetadata {
    pub files: Vec<MetadataFile>,
    pub image: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetMetadataResp {
    pub onchain_metadata: OnChainMetadata,
}
