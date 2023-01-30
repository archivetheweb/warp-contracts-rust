use std::collections::BTreeMap;

use atw::action::{ActionResult, HandlerResult, SubmitArchive};
use atw::error::ContractError;
use atw::state::{ArchiveSubmission, State};
use url::Url;

use super::Actionable;

impl Actionable for SubmitArchive {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        if !state.uploaders.contains_key(&caller) {
            return Err(ContractError::UploaderNotRegistered);
        }

        match state.archive_requests.get_mut(&self.archive_request_id) {
            Some(req) => req.latest_upload_timestamp = self.timestamp,
            None => return Err(ContractError::ArchiveRequestDoesNotExist),
        };

        let u = match Url::parse(&self.full_url) {
            Ok(u) => u,
            Err(e) => {
                return Err(ContractError::InvalidURL(e.to_string()));
            }
        };

        let domain: String = u.domain().unwrap().into();

        let submission = ArchiveSubmission {
            full_url: self.full_url.clone(),
            arweave_tx: self.arweave_tx,
            size: self.size,
            uploader_address: caller.clone(),
            archive_request_id: self.archive_request_id,
            timestamp: self.timestamp,
            options: self.options,
        };

        match state.archives.get_mut(&domain) {
            Some(d) => {
                if d.get(&self.timestamp).is_some() {
                    return Err(ContractError::ArchiveAlreadySubmitted);
                } else {
                    d.insert(self.timestamp, submission);
                }
            }
            None => {
                let mut h: BTreeMap<i64, ArchiveSubmission> = BTreeMap::new();
                h.insert(self.timestamp, submission);
                state.archives.insert(domain, h);
            }
        }

        let u = state.uploaders.get_mut(&caller).unwrap();
        u.upload_count = u.upload_count + 1;

        Ok(HandlerResult::Write(state))
    }
}
