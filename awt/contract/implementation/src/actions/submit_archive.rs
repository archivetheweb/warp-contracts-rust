use std::collections::BTreeMap;

use atw::action::{ActionResult, HandlerResult, SubmitArchive};
use atw::error::ContractError;
use atw::state::{ArchiveSubmission, State};
use warp_wasm_utils::contract_utils::js_imports::{log, SmartWeave, Transaction};

use super::Actionable;

impl Actionable for SubmitArchive {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
        log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());
        log(&("Transaction::id()".to_owned() + &Transaction::id()));

        if !state.uploaders.contains_key(&caller) {
            return Err(ContractError::UploaderNotRegistered);
        }

        let domain = state.archives.get_mut(&self.full_url);
        let submission = ArchiveSubmission {
            full_url: self.full_url.clone(),
            arweave_tx: self.arweave_tx,
            size: self.size,
            uploader_address: self.uploader_address,
            archiving_request_id: self.archiving_request_id,
            timestamp: self.timestamp,
            info: self.info,
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
                state.archives.insert(self.full_url.clone(), h);
            }
        }

        let u = state.uploaders.get_mut(&caller).unwrap();
        u.upload_count = u.upload_count + 1;

        Ok(HandlerResult::Write(state))
    }
}
