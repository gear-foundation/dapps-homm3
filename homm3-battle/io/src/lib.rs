#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;
use homm3_gamestate_io::{Hero, Stack};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct BattleStatistics {
    pub rounds: Vec<RoundInfo>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct RoundInfo {
    number: u32,
    winner_color: String,
    loser_color: String,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    Simulate(BattleInfo),
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Event {
    BattleResult(BattleInfo),
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = BattleStatistics;
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct BattleInfo {
    pub stacks: Vec<Stack>,
    pub sides: [BattleSide; 2],
    pub round: i32,
    pub active_stack: i32,
    pub terrain_type: Terrain,
}
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct BattleSide {
    pub color: String,
    pub hero: Hero,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Terrain {
    NativeTerrain,
    AnyTerrain,
    None,
    FirstRegularTerrain,
    Dirt,
    Sand,
    Grass,
    Snow,
    Swamp,
    Rough,
    Subterranean,
    Lava,
    Water,
    Rock,
    OriginalRegularTerrainCount,
}
