use super::Actionable;
use atw::action::{ActionResult, HandlerResult, RequestArchiving};
use atw::error::ContractError;
use atw::state::{ArchiveRequest, State};
use url::Url;
use warp_wasm_utils::contract_utils::js_imports::Transaction;

impl Actionable for RequestArchiving {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        let tx_id = &Transaction::id();

        if state.archive_requests.contains_key(tx_id) {
            return Err(ContractError::ChooseAnotherID);
        }

        let urls = self.crawl_options.urls.clone();
        for u in urls {
            match Url::parse(&u) {
                Ok(_) => {}
                Err(e) => return Err(ContractError::InvalidURL(e.to_string())),
            }
        }

        state.archive_requests.insert(
            tx_id.clone(),
            ArchiveRequest {
                id: tx_id.clone(),
                crawl_options: self.crawl_options,
                uploader_address: self.uploader_address,
                start_timestamp: self.start_timestamp,
                end_timestamp: self.end_timestamp,
                frequency: self.frequency,
                requested_by: caller,
            },
        );

        Ok(HandlerResult::Write(state))
    }
}
