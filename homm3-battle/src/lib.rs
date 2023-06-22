#![no_std]

use gstd::{errors::Result as GstdResult, msg, prelude::*, MessageId};
use homm3_battle_io::*;

#[derive(Debug, Default)]
pub struct Contract {
    statistics: BattleStatistics,
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
        Action::Simulate(battle_info) => {
            contract.estimate_battle(battle_info);
        }
    };
}

#[no_mangle]
extern "C" fn state() {
    let state = &state_mut().statistics.rounds;
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

impl Contract {
    fn estimate_battle(&mut self, battle_info: BattleInfo) {
        let mut battle_result: BattleInfo = battle_info.clone();
        gstd::debug!("battle_info: {:?}", battle_info);
        battle_result.sides[1].hero.stacks = Default::default();
        gstd::debug!("battle_result: {:?}", battle_result);
        reply(Event::BattleResult(battle_result)).expect("Can't reply BattleResult");
    }
}
