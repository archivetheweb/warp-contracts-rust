use atw::action::{ActionResult, HandlerResult, RequestArchiving};
use atw::error::ContractError;
use atw::state::{ArchiveRequest, State};
use warp_wasm_utils::contract_utils::js_imports::{log, SmartWeave, Transaction};

use super::Actionable;

impl Actionable for RequestArchiving {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
        log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());
        log(&("Transaction::id()".to_owned() + &Transaction::id()));

        let tx_id = &Transaction::id();

        if state.archiving_requests.contains_key(tx_id) {
            return Err(ContractError::ChooseAnotherID);
        }

        state.archiving_requests.insert(
            tx_id.clone(),
            ArchiveRequest {
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
