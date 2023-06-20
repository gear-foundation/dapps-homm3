#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::ActorId;
use homm3_gamestate_io::*;

#[metawasm]
pub mod metafns {

    pub type State = <ContractMetadata as Metadata>::State;

    pub fn game_state(state: State, saver_id: ActorId) -> gstd::Vec<GameState> {
        state
            .iter()
            .cloned()
            .filter(|state| state.saver_id.eq(&saver_id))
            .collect()
    }
}
