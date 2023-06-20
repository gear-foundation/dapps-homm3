#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct GameState {
    pub day: u32,
    pub current_player: String,
    pub player_states: Vec<PlayerState>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    SaveGameState {
        day: u32,
        current_player: String,
        player_states: Vec<PlayerState>,
    },
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Event {
    SavedGameState,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = GameState;
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
#[repr(u8)]
pub enum PrimarySkill {
    None = u8::MAX,
    Attack = 0,
    Defense,
    SpellPower,
    Knowledge,
    Experience = 4,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
#[repr(u8)]
pub enum SecondarySkill {
    Wrong = u8::MAX - 1_u8,
    Default = u8::MAX,
    Pathfinding = 0_u8,
    Archery,
    Logistics,
    Scouting,
    Diplomacy,
    Navigation,
    Leadership,
    Wisdom,
    Mysticism,
    Luck,
    Ballistics,
    EagleEye,
    Necromancy,
    Estates,
    FireMagic,
    AirMagic,
    WaterMagic,
    EarthMagic,
    Scholar,
    Tactics,
    Artillery,
    Learning,
    Offence,
    Armorer,
    Intelligence,
    Sorcery,
    Resistance,
    FirstAid,
    SkillSize,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
#[repr(u8)]
pub enum FortLevel {
    None = 0_u8,
    Fort = 1_u8,
    Citadel = 2_u8,
    Castle = 3_u8
    ,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
#[repr(u8)]
pub enum HallLevel {
    None = u8::MAX,
    Village = 0_u8,
    Town = 1_u8,
    City = 2_u8,
    Capitol = 3_u8,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub struct SecondarySkillInfo {
    pub skill: SecondarySkill,
    pub value: u8,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub struct Stack {
    pub name: String,
    pub level: i32,
    pub count: u32,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub mana: i32,
    pub sex: u8,
    pub experience_points: i64,
    pub secondary_skills: Vec<SecondarySkillInfo>,
    pub stacks: [Option<Stack>; 7]
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub struct Town {
    pub name: String,
    pub fort_level: FortLevel,
    pub hall_level: HallLevel,
    pub mage_guild_level: i32,
    pub level: i32,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub enum Resource {
    Wood(i64),
    Mercury(i64),
    Ore(i64),
    Sulfur(i64),
    Crystal(i64),
    Gems(i64),
    Gold(i64),
    Mithril(i64),
    WoodAndOre,
    Invalid,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Encode, Decode, TypeInfo)]
pub struct PlayerState {
    pub color: String,
    pub team_id: u32,
    pub is_human: bool,
    pub resources: Vec<Resource>,
    pub heroes: Vec<Hero>,
    pub towns: Vec<Town>,
    pub days_without_castle: Option<u8>,
}
