use async_trait::async_trait;
use warp_pst::{action::ActionResult, state::State};

pub mod evolve;
pub mod register_uploader;
pub mod request_archiving;
pub mod submit_archive;
pub mod uploaders;

pub use evolve::*;
pub use register_uploader::*;
pub use request_archiving::*;
pub use submit_archive::*;
pub use uploaders::*;

use warp_wasm_utils::contract_utils::js_imports::log;

pub trait Actionable {
    fn action(self, caller: String, state: State) -> ActionResult;
}

#[async_trait(?Send)]
pub trait AsyncActionable {
    async fn action(self, caller: String, state: State) -> ActionResult;
}
