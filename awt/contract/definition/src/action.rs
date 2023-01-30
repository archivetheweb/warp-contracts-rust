use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::error::ContractError;
use crate::state::{ArchiveOptions, ArchiveRequest, ArchiveSubmission, CrawlOptions, State};

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURL {
    pub url: String,
    pub count: usize,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequestsFor {
    pub address: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUploader {
    pub friendly_name: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeRegisterUploader {}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestArchiving {
    pub crawl_options: CrawlOptions,
    pub uploader_address: String, // uploader for this pool
    pub start_timestamp: i64, // start_timestamp of the period where we want to archive the website
    pub end_timestamp: i64,   // end_timestamp
    pub frequency: String,    // frequency of the archiving i.e. here it's once an hour (cron)
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubmitArchive {
    pub full_url: String,   // full url
    pub arweave_tx: String, // tx where the archive is deploy (here can add index tx too)
    pub size: usize,
    pub archive_request_id: String, // index of the archiving request
    pub timestamp: i64,
    pub options: ArchiveOptions, // frequency of the archiving i.e. here it's once an hour (cron)
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteArchiveRequest {
    pub archive_id: String,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequestByID {
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
    RegisterUploader(RegisterUploader),
    DeRegisterUploader(DeRegisterUploader),
    RequestArchiving(RequestArchiving),
    SubmitArchive(SubmitArchive),
    DeleteArchiveRequest(DeleteArchiveRequest),

    ArchiveRequestByID(ArchiveRequestByID),
    ArchivesByURL(ArchivesByURL),
    ArchiveRequestsFor(ArchiveRequestsFor),
    Evolve(Evolve),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum View {
    ArchivesByURL(ArchivesByURL),
    ArchivesByURLResult(ArchivesByURLResult),

    ArchiveRequestsFor(ArchiveRequestsFor),
    ArchiveRequestsForResult(ArchiveRequestsForResult),

    ArchiveRequestByID(ArchiveRequestByID),
    ArchiveRequestByIDResult(ArchiveRequestByIDResult),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum WriteAction {
    RegisterUploader(RegisterUploader),
    RequestArchiving(RequestArchiving),
    SubmitArchive(SubmitArchive),
    DeleteArchiveRequest(DeleteArchiveRequest),
    DeRegisterUploader(DeRegisterUploader),

    Evolve(Evolve),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURLResult {
    pub archives: Vec<ArchiveSubmission>,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequestsForResult {
    pub archives_requests: Vec<ArchiveRequest>,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveRequestByIDResult {
    pub archives_request: Option<ArchiveRequest>,
}

#[derive(JsonSchema, Clone, PartialEq, Debug, Serialize, Deserialize, Eq, EnumIter)]
#[serde(rename_all = "camelCase", tag = "function")]
pub enum ReadResponse {
    ArchivesResult(ArchivesByURLResult),
    ArchiveRequestsResult(ArchiveRequestsForResult),
    ArchiveRequestResult(ArchiveRequestByIDResult),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HandlerResult {
    Write(State),
    Read(ReadResponse),
}

pub type ActionResult = Result<HandlerResult, ContractError>;
