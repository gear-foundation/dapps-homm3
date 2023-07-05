#![no_std]

use gstd::{errors::Result as GstdResult, msg, prelude::*, MessageId};
use homm3_archive_io::*;

#[derive(Debug, Default)]
pub struct Contract {
    saves: Vec<GameArchive>,
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
        Action::SaveArchive(state) => {
            contract.saves.push(state);
            Event::SavedArchive
        }
        Action::Load { hash } => {
            let state = contract
                .saves
                .iter()
                .find(|state| state.archive.hash.eq(&hash));

            Event::Loaded(state.cloned())
        }
    };
    gstd::debug!("Event = {:?}, encoded = {:?}", event, event.encode());
    let _msg_id = reply(event).expect("Can't send reply");
}

#[no_mangle]
extern "C" fn state() {
    let state = &state_mut().saves;

    reply(state).expect("failed to encode or reply from `state()`");
}


fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
