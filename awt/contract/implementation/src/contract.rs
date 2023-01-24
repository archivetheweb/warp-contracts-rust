use async_recursion::async_recursion;

use atw::action::{Action, ActionResult};
use atw::state::State;

use crate::actions::Actionable;
use warp_wasm_utils::contract_utils::js_imports::SmartWeave;

#[async_recursion(?Send)]
pub async fn handle(state: State, action: Action) -> ActionResult {
    // let original_caller = Transaction::owner();
    let effective_caller = SmartWeave::caller();

    match action {
        Action::RegisterUploader(action) => action.action(effective_caller, state),
        Action::DeRegisterUploader(action) => action.action(effective_caller, state),
        Action::SubmitArchive(action) => action.action(effective_caller, state),
        Action::RequestArchiving(action) => action.action(effective_caller, state),
        Action::Evolve(action) => action.action(effective_caller, state),
        Action::DeleteArchiveRequest(action) => action.action(effective_caller, state),
        Action::ArchivesByURL(action) => action.action(effective_caller, state),
        Action::ArchiveRequestsFor(action) => action.action(effective_caller, state),
        Action::ArchiveRequestByID(action) => action.action(effective_caller, state),
    }
}
