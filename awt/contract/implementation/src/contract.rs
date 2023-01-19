use async_recursion::async_recursion;

use atw::action::{Action, ActionResult};
use atw::state::State;

use crate::actions::{Actionable, *};
use warp_wasm_utils::contract_utils::js_imports::{log, Block, Contract, SmartWeave, Transaction};

#[async_recursion(?Send)]
pub async fn handle(state: State, action: Action) -> ActionResult {
    let original_caller = Transaction::owner();
    let effective_caller = SmartWeave::caller();

    //Example of accessing functions imported from js:
    // log("log from contract");
    // log(&("Transaction::id()".to_owned() + &Transaction::id()));
    // log(&("Transaction::owner()".to_owned() + &Transaction::owner()));
    // log(&("Transaction::target()".to_owned() + &Transaction::target()));

    // log(&("Block::height()".to_owned() + &Block::height().to_string()));
    // log(&("Block::indep_hash()".to_owned() + &Block::indep_hash()));
    // log(&("Block::timestamp()".to_owned() + &Block::timestamp().to_string()));

    // log(&("Contract::id()".to_owned() + &Contract::id()));
    // log(&("Contract::owner()".to_owned() + &Contract::owner()));

    // log(&("SmartWeave::caller()".to_owned() + &SmartWeave::caller()));

    // for vrf-compatible interactions
    /*log(&("Vrf::value()".to_owned() + &Vrf::value()));
    log(&("Vrf::randomInt()".to_owned() + &Vrf::randomInt(7).to_string()));*/

    match action {
        Action::RegisterUploader(action) => action.action(effective_caller, state),
        Action::SubmitArchive(action) => action.action(effective_caller, state),
        Action::RequestArchiving(action) => action.action(effective_caller, state),
        Action::Evolve(action) => action.action(effective_caller, state),
        // Action::Uploader(action) => action.action(effective_caller, state),
        Action::DeleteArchiveRequest(action) => action.action(effective_caller, state),
        Action::ArchivesByURL(action) => action.action(effective_caller, state),
        Action::ArchiveRequestsFor(action) => action.action(effective_caller, state),
        Action::ArchiveRequestByID(action) => action.action(effective_caller, state),
    }
}
