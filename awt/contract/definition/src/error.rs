use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum ContractError {
    RuntimeError(String),
    UploaderAlreadyAdded,
    UploaderNotRegistered,

    ChooseAnotherID,
    ArchiveDoesNotExist,
    ArchiveRequestNotDone,
    ArchiveRequestDoesNotExist,

    ArchiveAlreadySubmitted,

    OnlyOwnerCanEvolve,
    EvolveNotAllowed,
}
