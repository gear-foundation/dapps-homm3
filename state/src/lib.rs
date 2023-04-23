#![no_std]

use app_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn saved_games(state: State) -> Vec<(ActorId, String)> {
        state
            .iter()
            .map(|(actor_id, state)| (actor_id.clone(), state.tar.filename.clone()))
            .collect()
    }
}
