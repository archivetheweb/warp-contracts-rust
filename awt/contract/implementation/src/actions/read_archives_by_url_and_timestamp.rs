use atw::{
    action::{ActionResult, ArchivesByURL, ArchivesByURLResult, HandlerResult, ReadResponse},
    state::{ArchivesByURLInfo, State},
};

use super::Actionable;

impl Actionable for ArchivesByURLAndTimestamp {
    fn action(self, _caller: String, state: State) -> ActionResult {
        let arcs = state.archives.get(&self.url);

        let archive = match arcs {
            Some(d) => d,
            None => return Err(atw::error::ContractError::ArchiveDoesNotExist),
        };

        let archive = match archive.get(&self.timestamp) {
            Some(d) => d,
            None => return Err(atw::error::ContractError::ArchiveAtTimestampDoesNotExist),
        };

        Ok(HandlerResult::Read(
            ReadResponse::ArchivesByURLAndTimestampResult(ArchivesByURLAndTimestampResult {
                archive: d,
            }),
        ))
    }
}
