use atw::{
    action::{ActionResult, ArchivesByURL, ArchivesByURLResult, HandlerResult, ReadResponse},
    state::State,
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
                .collect(),
            None => vec![],
        };

        Ok(HandlerResult::Read(ReadResponse::ArchivesResult(
            ArchivesByURLResult { archives: arcs },
        )))
    }
}
