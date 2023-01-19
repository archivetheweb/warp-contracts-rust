use warp_pst::{
    action::{ActionResult, ArchiveRequestByID, HandlerResult, ReadResponse},
    state::State,
};

use super::Actionable;

impl Actionable for ArchiveRequestByID {
    fn action(self, _caller: String, state: State) -> ActionResult {
        Ok(HandlerResult::Read(ReadResponse::ArchiveRequest(
            state.archiving_requests.get(&self.archive_id).cloned(),
        )))
    }
}
