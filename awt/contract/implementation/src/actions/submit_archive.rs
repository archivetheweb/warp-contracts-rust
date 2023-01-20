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

        if !state
            .archive_requests
            .contains_key(&self.archive_request_id)
        {
            return Err(ContractError::ArchiveRequestDoesNotExist);
        }

        let u = match Url::parse(&self.full_url) {
            Ok(u) => u,
            Err(e) => {
                return Err(ContractError::InvalidURL(e.to_string()));
            }
        };

        let domain = state.archives.get_mut(&self.full_url);
        let submission = ArchiveSubmission {
            full_url: self.full_url.clone(),
            arweave_tx: self.arweave_tx,
            size: self.size,
            uploader_address: caller.clone(),
            archive_request_id: self.archive_request_id,
            timestamp: self.timestamp,
            options: self.options,
        };
        match domain {
            Some(d) => {
                if d.get(&self.timestamp).is_some() {
                    return Err(ContractError::ArchiveAlreadySubmitted);
                } else {
                    d.insert(self.timestamp, submission);
                }
            }
            None => {
                let mut h: BTreeMap<usize, ArchiveSubmission> = BTreeMap::new();
                h.insert(self.timestamp, submission);
                state.archives.insert(u.domain().unwrap().into(), h);
            }
        }

        let u = state.uploaders.get_mut(&caller).unwrap();
        u.upload_count = u.upload_count + 1;

        Ok(HandlerResult::Write(state))
    }
}
