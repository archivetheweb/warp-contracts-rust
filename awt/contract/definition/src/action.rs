use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::error::ContractError;
use crate::state::{ArchiveSubmission, CrawlOptions, State, Uploader};

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub target: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Uploaders {}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURL {
    pub url: String,
    pub count: usize,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUploader {
    pub friendly_name: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestArchiving {
    pub crawl_options: CrawlOptions,
    pub uploader_address: String, // uploader for this pool
    pub start_timestamp: usize, // start_timestamp of the period where we want to archive the website
    pub end_timestamp: usize,   // end_timestamp
    pub frequency: String,      // frequency of the archiving i.e. here it's once an hour (cron)
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubmitArchive {
    pub full_url: String,   // full url
    pub arweave_tx: String, // tx where the archive is deploy (here can add index tx too)
    pub size: usize,
    pub uploader_address: String,
    pub archiving_request_id: String, // index of the archiving request
    pub timestamp: usize,
    pub info: CrawlOptions, // frequency of the archiving i.e. here it's once an hour (cron)
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteArchiveRequest {
    pub archive_id: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Evolve {
    pub value: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum Action {
    // Balance(Balance),
    RegisterUploader(RegisterUploader),
    RequestArchiving(RequestArchiving),
    SubmitArchive(SubmitArchive),
    DeleteArchiveRequest(DeleteArchiveRequest),
    ArchivesByURL(ArchivesByURL),
    // ArchiveRequestByID(),
    // ArchiveRequestsFor(),
    Uploader(Uploaders),

    Evolve(Evolve),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum View {
    Balance(Balance),
    BalanceResult(BalanceResult),
    // ArchiveRequestsFor(),
    // Uploader(),
    Uploader(Uploaders),
    ArchivesByURL(ArchivesByURL),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum WriteAction {
    RegisterUploader(RegisterUploader),
    RequestArchiving(RequestArchiving),
    SubmitArchive(SubmitArchive),
    DeleteArchiveRequest(DeleteArchiveRequest),

    Evolve(Evolve),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResult {
    pub balance: u64,
    pub ticker: String,
    pub target: String,
}

#[derive(JsonSchema, Clone, PartialEq, Debug, Serialize, Deserialize, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum ReadResponse {
    BalanceResult(BalanceResult),
    UploadersResult(HashMap<String, Uploader>),
    Archives(Vec<ArchiveSubmission>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HandlerResult {
    Write(State),
    Read(ReadResponse),
}

pub type ActionResult = Result<HandlerResult, ContractError>;
