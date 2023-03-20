use atw::action::{ActionResult, HandlerResult, UpgradeState};
use atw::state::{CrawlType, State};

use super::Actionable;

impl Actionable for UpgradeState {
    fn action(self, _: String, mut state: State) -> ActionResult {
        // we go through the every archive_submission in the state, and add a new crawl_type field to its options field
        for (_, btree) in state.archives.iter_mut() {
            for (_, archive_submission) in btree {
                if archive_submission.options.domain_only.unwrap() {
                    archive_submission.options.crawl_type = Some(CrawlType::DomainOnly);
                } else {
                    archive_submission.options.crawl_type = Some(CrawlType::DomainAndLinks);
                }
                archive_submission.options.domain_only = None;
            }
        }

        for (_, archive_request) in state.archive_requests.iter_mut() {
            if archive_request.options.domain_only.unwrap() {
                archive_request.options.crawl_type = Some(CrawlType::DomainOnly);
            } else {
                archive_request.options.crawl_type = Some(CrawlType::DomainAndLinks);
            }
            archive_request.options.domain_only = None;
        }

        Ok(HandlerResult::Write(state))
    }
}
