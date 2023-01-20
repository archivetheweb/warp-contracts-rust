use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use strum_macros::EnumIter;

#[derive(JsonSchema, Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolve: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_evolve: Option<bool>,
    pub archive_requests: HashMap<String, ArchiveRequest>,
    pub archives: BTreeMap<String, BTreeMap<usize, ArchiveSubmission>>,
    pub uploaders: HashMap<String, Uploader>,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum ContractState {
    State(State),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequest {
    pub id: String,
    pub crawl_options: CrawlOptions,
    pub uploader_address: String, // uploader for this pool
    pub start_timestamp: usize, // start_timestamp of the period where we want to archive the website
    pub end_timestamp: usize,   // end_timestamp
    pub frequency: String,      // frequency of the archiving i.e. here it's once an hour (cron)
    pub requested_by: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrawlOptions {
    pub urls: Vec<String>,
    pub depth: u8,         // depth of the crawl
    pub domain_only: bool, // whether we want a domain only crawl
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveOptions {
    pub depth: u8,         // depth of the crawl
    pub domain_only: bool, // whether we want a domain only crawl
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveSubmission {
    pub full_url: String,   // full url
    pub arweave_tx: String, // tx where the archive is deploy (here can add index tx too)
    pub size: usize,
    pub uploader_address: String,
    pub archive_request_id: String, // index of the archiving request
    pub timestamp: usize,
    pub options: ArchiveOptions,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub friendly_name: String,
    pub upload_count: usize,
}
