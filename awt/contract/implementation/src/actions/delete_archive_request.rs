use atw::action::{ActionResult, DeleteArchiveRequest, HandlerResult};
use atw::error::ContractError;
use atw::state::State;
use warp_wasm_utils::contract_utils::js_imports::{log, SmartWeave, Transaction};

use super::Actionable;

impl Actionable for DeleteArchiveRequest {
    fn action(self, _caller: String, mut state: State) -> ActionResult {
        log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
        log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());
        log(&("Transaction::id()".to_owned() + &Transaction::id()));

        let archive_request = state.archive_requests.get(&self.archive_id);
        if archive_request.is_none() {
            return Err(ContractError::ArchiveDoesNotExist);
        }

        state.archive_requests.remove(&self.archive_id);

        Ok(HandlerResult::Write(state))
    }
}
