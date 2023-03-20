use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::error::ContractError;
use crate::state::{
    ArchiveInfo, ArchiveOptions, ArchiveRequest, ArchiveRequestOptions, ArchiveSubmission,
    ArchivesByURLInfo, State,
};

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURL {
    pub url: String,
    pub count: usize,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURLAndTimestamp {
    pub url: String,
    pub timestamp: i64,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Archives {}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeState {}

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
    pub options: ArchiveRequestOptions,
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
    pub screenshot_tx: String,
    pub title: String,
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
    // Write
    RegisterUploader(RegisterUploader),
    DeRegisterUploader(DeRegisterUploader),
    RequestArchiving(RequestArchiving),
    SubmitArchive(SubmitArchive),
    DeleteArchiveRequest(DeleteArchiveRequest),
    Evolve(Evolve),

    // Read
    ArchivesByURL(ArchivesByURL),
    ArchivesByURLAndTimestamp(ArchivesByURLAndTimestamp),
    ArchiveRequestsFor(ArchiveRequestsFor),
    ArchiveRequestByID(ArchiveRequestByID),
    Archives(Archives),
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

    Archives(Archives),
    ArchivesResult(ArchivesResult),

    ArchivesByURLAndTimestamp(ArchivesByURLAndTimestamp),
    ArchivesByURLAndTimestampResult(ArchivesByURLAndTimestampResult),
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
    // UpgradeState(UpgradeState),
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURLResult {
    pub archives: ArchivesByURLInfo,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesByURLAndTimestampResult {
    pub archive: ArchiveSubmission,
}

#[derive(JsonSchema, Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArchivesResult {
    pub archives: Vec<ArchiveInfo>,
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
    ArchivesResult(ArchivesResult),
    ArchivesByURLResult(ArchivesByURLResult),
    ArchivesByURLAndTimestampResult(ArchivesByURLAndTimestampResult),
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
