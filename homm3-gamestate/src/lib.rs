#![no_std]

use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, msg, prelude::*, MessageId};
use homm3_gamestate_io::*;

#[derive(Debug, Default)]
pub struct Contract {
    state: GameState,
}

static mut CONTRACT: Option<Contract> = None;

fn state_mut() -> &'static mut Contract {
    let state = unsafe { CONTRACT.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    unsafe { CONTRACT = Some(Default::default()) }
}

#[gstd::async_main]
async fn main() {
    let action: Action = msg::load().expect("Error at loading Homm3 Action");
    let contract = state_mut();

    gstd::debug!("Action = {:?}", action);

    match action {
        Action::SaveGameState {
            day,
            current_player,
            player_states,
        } => {
            contract.state = GameState {
                day,
                current_player,
                player_states,
            }
        }
    };
}

#[no_mangle]
extern "C" fn state() {
    let state: <ContractMetadata as Metadata>::State = common_state();

    reply(state).expect("failed to encode or reply from `state()`");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");

    reply(metahash).expect("failed to encode or reply from `metahash()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

fn common_state() -> <ContractMetadata as Metadata>::State {
    state_mut().state.clone()
}
