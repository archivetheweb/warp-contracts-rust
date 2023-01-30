use atw::{
    action::{ActionResult, ArchivesByURL, ArchivesByURLResult, HandlerResult, ReadResponse},
    state::{ArchivesByURLInfo, State},
};

use super::Actionable;

impl Actionable for ArchivesByURL {
    fn action(self, _caller: String, state: State) -> ActionResult {
        let arcs = state.archives.get(&self.url);

        let arcs = match arcs {
            Some(d) => d
                .iter()
                .rev()
                .take(self.count)
                .map(|x| x.1.to_owned())
                .collect::<Vec<_>>(),
            None => return Err(atw::error::ContractError::ArchiveDoesNotExist),
        };

        Ok(HandlerResult::Read(ReadResponse::ArchivesByURLResult(
            ArchivesByURLResult {
                archives: ArchivesByURLInfo {
                    screenshot_tx: arcs[0].screenshot_tx.clone(),
                    title: arcs[0].title.clone(),
                    url: self.url,
                    last_archived_timestamp: arcs[0].timestamp,
                    archived_info: arcs,
                },
            },
        )))
    }
}
