use async_trait::async_trait;
use atw::{action::ActionResult, state::State};

pub mod delete_archive_request;
pub mod deregister_uploader;
pub mod evolve;
pub mod register_uploader;
pub mod request_archiving;
pub mod submit_archive;

pub mod read_archive_requests_by_id;
pub mod read_archive_requests_for;
pub mod read_archives_by_url;
pub mod read_archives_by_url_and_timestamp;
pub mod read_get_archives;

pub mod upgrade_state;

pub trait Actionable {
    fn action(self, caller: String, state: State) -> ActionResult;
}

#[async_trait(?Send)]
pub trait AsyncActionable {
    async fn action(self, caller: String, state: State) -> ActionResult;
}
