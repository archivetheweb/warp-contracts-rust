use atw::{
    action::{ActionResult, HandlerResult, ReadResponse, UploaderResult, Uploaders},
    state::State,
};

use super::Actionable;

impl Actionable for Uploaders {
    fn action(self, _caller: String, state: State) -> ActionResult {
        Ok(HandlerResult::Read(ReadResponse::UploadersResult(
            UploaderResult {
                uploaders: state.uploaders,
            },
        )))
    }
}
