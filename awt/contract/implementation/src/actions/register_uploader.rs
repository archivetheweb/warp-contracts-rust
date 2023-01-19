use atw::action::{ActionResult, HandlerResult, RegisterUploader};
use atw::error::ContractError;
use atw::state::{State, Uploader};
use warp_wasm_utils::contract_utils::js_imports::{log, SmartWeave, Transaction};

use super::Actionable;

impl Actionable for RegisterUploader {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        log(("caller ".to_owned() + &SmartWeave::caller()).as_str());
        log(("Transaction owner ".to_owned() + &Transaction::owner()).as_str());

        if state.uploaders.contains_key(&caller) {
            return Err(ContractError::UploaderAlreadyAdded);
        }

        state.uploaders.insert(
            caller.clone(),
            Uploader {
                friendly_name: self.friendly_name,
                upload_count: 0,
            },
        );

        Ok(HandlerResult::Write(state))
    }
}
