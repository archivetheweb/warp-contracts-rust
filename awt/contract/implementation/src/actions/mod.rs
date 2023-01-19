use async_trait::async_trait;
use warp_pst::{action::ActionResult, state::State};

pub mod delete_archive_request;
pub mod evolve;
pub mod register_uploader;
pub mod request_archiving;
pub mod submit_archive;

pub mod read_archive_requests_by_id;
pub mod read_archive_requests_for;
pub mod read_archives_by_url;
pub mod read_uploaders;

// use warp_wasm_utils::contract_utils::js_imports::log;

pub trait Actionable {
    fn action(self, caller: String, state: State) -> ActionResult;
}

#[async_trait(?Send)]
pub trait AsyncActionable {
    async fn action(self, caller: String, state: State) -> ActionResult;
}
