use warp_pst::{
    action::{ActionResult, Balance, BalanceResult, HandlerResult, ReadResponse, Uploaders},
    error::ContractError,
    state::{State, Uploader},
};
use warp_wasm_utils::contract_utils::js_imports::log;

use super::Actionable;

impl Actionable for Uploaders {
    fn action(self, _caller: String, state: State) -> ActionResult {
        Ok(HandlerResult::Read(ReadResponse::UploadersResult(
            state.uploaders,
        )))
    }
}
