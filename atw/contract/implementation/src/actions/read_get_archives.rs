use atw::{
    action::{ActionResult, Archives, ArchivesResult, HandlerResult, ReadResponse},
    state::{ArchiveInfo, State},
};

use super::Actionable;

impl Actionable for Archives {
    fn action(self, _caller: String, state: State) -> ActionResult {
        let arcs = state.archives;

        let mut v = Vec::with_capacity(arcs.len());
        for (url, arc) in arcs {
            let count = arc.len();
            let latest = arc.into_iter().rev().nth(0).unwrap();

            v.push(ArchiveInfo {
                url: url,
                title: latest.1.title.clone(),
                screenshot_tx: latest.1.screenshot_tx.clone(),
                archived_count: count,
                last_archived_timestamp: latest.0,
            })
        }

        Ok(HandlerResult::Read(ReadResponse::ArchivesResult(
            ArchivesResult { archives: v },
        )))
    }
}
