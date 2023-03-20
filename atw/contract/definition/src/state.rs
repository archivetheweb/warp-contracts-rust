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
    pub archives: BTreeMap<String, BTreeMap<i64, ArchiveSubmission>>,
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
    pub options: ArchiveRequestOptions,
    pub uploader_address: String,       // uploader for this pool
    pub start_timestamp: i64, // start_timestamp of the period where we want to archive the website
    pub end_timestamp: i64,   // end_timestamp
    pub latest_archived_timestamp: i64, // end_timestamp
    pub frequency: String,    // frequency of the archiving i.e. here it's once an hour (cron)
    pub requested_by: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequestOptions {
    pub urls: Vec<String>, // list of urls to archive
    pub depth: u8,         // depth of the crawl
    // still need this as it's part of state - will need to remove
    pub domain_only: Option<bool>,
    pub crawl_type: Option<CrawlType>, // type of crawl
}

// CrawlType is the type of crawl to perform. Do not change the order of this enum
#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum CrawlType {
    // Crawl only links on the same domain as the starting URL.
    #[default]
    DomainOnly,
    // Crawls all links on the same domain and links on the page (depth 1)
    DomainWithPageLinks,
    // Crawls all links on the same domain and links on the page, going down the tree
    DomainAndLinks,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveOptions {
    pub depth: u8,                     // depth of the crawl
    pub crawl_type: Option<CrawlType>, // type of crawl (option for now)
    // still need this as it's part of state - will need to remove
    pub domain_only: Option<bool>, // whether we want a domain only crawl
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveSubmission {
    pub full_url: String,   // full url
    pub arweave_tx: String, // tx where the archive is deploy (here can add index tx too)
    pub size: usize,
    pub uploader_address: String,
    pub archive_request_id: String, // index of the archiving request
    pub timestamp: i64,
    pub options: ArchiveOptions,
    pub screenshot_tx: String,
    pub title: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURLInfo {
    pub screenshot_tx: String,
    pub title: String,
    pub url: String,
    pub last_archived_timestamp: i64,
    pub archived_info: Vec<ArchiveSubmission>,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInfo {
    pub url: String, // full url
    pub screenshot_tx: String,
    pub title: String,
    pub last_archived_timestamp: i64,
    pub archived_count: usize,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub friendly_name: String,
    pub upload_count: usize,
}
