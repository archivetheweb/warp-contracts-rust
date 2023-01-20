use atw::{
    action::{
        ActionResult, ArchiveRequestByID, ArchiveRequestByIDResult, HandlerResult, ReadResponse,
    },
    state::State,
};

use super::Actionable;

impl Actionable for ArchiveRequestByID {
    fn action(self, _caller: String, state: State) -> ActionResult {
        Ok(HandlerResult::Read(ReadResponse::ArchiveRequestResult(
            ArchiveRequestByIDResult {
                archives_request: state.archive_requests.get(&self.archive_id).cloned(),
            },
        )))
    }
}
