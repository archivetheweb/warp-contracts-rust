use atw::action::{ActionResult, DeRegisterUploader, HandlerResult};
use atw::error::ContractError;
use atw::state::State;

use super::Actionable;

impl Actionable for DeRegisterUploader {
    fn action(self, caller: String, mut state: State) -> ActionResult {
        if !state.uploaders.contains_key(&caller) {
            return Err(ContractError::UploaderNotRegistered);
        }

        state.uploaders.remove(&caller);

        Ok(HandlerResult::Write(state))
    }
}
