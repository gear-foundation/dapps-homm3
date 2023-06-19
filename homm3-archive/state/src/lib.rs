#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::Vec, ActorId};
use homm3_archive_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn saved_games(state: State, saver_id: ActorId) -> Vec<GameArchive> {
        state
            .iter()
            .cloned()
            .filter(|state| state.saver_id.eq(&saver_id))
            .collect()
    }
}
