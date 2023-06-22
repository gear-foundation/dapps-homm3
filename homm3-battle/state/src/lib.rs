#![no_std]

use gmeta::{metawasm, Metadata};
use homm3_battle_io::*;

#[metawasm]
pub mod metafns {

    pub type State = <ContractMetadata as Metadata>::State;

    pub fn game_state(state: State) -> gstd::Vec<RoundInfo> {
        state.rounds.clone()
    }
}
