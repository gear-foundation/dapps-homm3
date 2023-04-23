#![no_std]

use app_io::*;
use gmeta::Metadata;
use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    ActorId, MessageId,
};

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[derive(Debug, Default)]
pub struct Contract {
    saves: Vec<(ActorId, GameState)>,
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

    let event = match action {
        Action::Save(state) => {
            contract.saves.push((msg::source(), state));
            Event::Saved
        }
        Action::Load { hash } => {
            let state = match contract
                .saves
                .iter()
                .find(|(_actor_id, state)| state.tar.hash.eq(&hash))
            {
                Some((_actor_id, state)) => Some(state),
                None => None,
            };
            Event::Loaded(state.cloned())
        }
    };
    gstd::debug!("Event = {:?}, encoded = {:?}", event, event.encode());
    let _msg_id = reply(event).expect("Can't send reply");
}

#[no_mangle]
extern "C" fn state() {
    let state: <ContractMetadata as Metadata>::State = common_state()
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect();

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
    state_mut().saves.clone().into()
}
