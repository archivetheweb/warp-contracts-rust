use atw::{
    action::{
        ActionResult, ArchiveRequestsFor, ArchiveRequestsForResult, HandlerResult, ReadResponse,
    },
    state::State,
};

use super::Actionable;

impl Actionable for ArchiveRequestsFor {
    fn action(self, _caller: String, state: State) -> ActionResult {
        let arcs = state
            .archive_requests
            .iter()
            .filter_map(|x| {
                if x.1.uploader_address == self.address {
                    return Some(x.1.to_owned());
                }
                None
            })
            .collect();

        Ok(HandlerResult::Read(ReadResponse::ArchiveRequestsResult(
            ArchiveRequestsForResult {
                archives_requests: arcs,
            },
        )))
    }
}
