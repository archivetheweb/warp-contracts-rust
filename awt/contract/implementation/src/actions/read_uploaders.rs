use warp_pst::{
    action::{ActionResult, HandlerResult, ReadResponse, Uploaders},
    state::State,
};

use super::Actionable;

impl Actionable for Uploaders {
    fn action(self, _caller: String, state: State) -> ActionResult {
        Ok(HandlerResult::Read(ReadResponse::UploadersResult(
            state.uploaders,
        )))
    }
}
