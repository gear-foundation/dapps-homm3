#![no_std]

use gstd::{errors::Result as GstdResult, msg, prelude::*, MessageId};
use homm3_gamestate_io::*;

#[derive(Debug, Default)]
pub struct Contract {
    states: Vec<GameState>,
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
            saver_id,
            day,
            current_player,
            player_states,
        } => {
            contract.states.push(GameState {
                saver_id,
                day,
                current_player,
                player_states,
            });
        }
    };
}

#[no_mangle]
extern "C" fn state() {
    let state = &state_mut().states;
    reply(state).expect("failed to encode or reply from `state()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

