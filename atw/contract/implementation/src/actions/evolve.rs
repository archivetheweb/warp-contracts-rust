use atw::action::{ActionResult, Evolve, HandlerResult};
use atw::error::ContractError;
use atw::state::State;

use warp_wasm_utils::contract_utils::js_imports::Transaction;

use super::Actionable;

impl Actionable for Evolve {
    fn action(self, _caller: String, mut state: State) -> ActionResult {
        match state.can_evolve {
            Some(can_evolve) => {
                if can_evolve && state.owner == Transaction::owner() {
                    state.evolve = Option::from(self.value);
                    Ok(HandlerResult::Write(state))
                } else {
                    Err(ContractError::OnlyOwnerCanEvolve)
                }
            }
            None => Err(ContractError::EvolveNotAllowed),
        }
    }
}
