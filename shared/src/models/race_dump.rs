use std::ops::Sub;

use serde::{Deserialize, Serialize};

use crate::models::RunningStyle;

const EPSILON: f64 = 0.001;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawFrame {
    pub time: f64,
    #[serde(rename = "horseDataArray")]
    pub horse_data_array: Vec<RawHorseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayFrame {
    pub time: f64,
    pub horse_data_array: Vec<ReplayHorseData>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RawHorseData {
    pub distance: f64,
    #[serde(rename = "lane_position")]
    pub lane_position: f64,
    pub speed: f64,
    pub hp: f64,
    #[serde(rename = "temptationMode")]
    pub temptation_mode: i64,
    #[serde(rename = "blockFrontHorseIndex")]
    pub block_front_horse_index: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReplayHorseData {
    pub distance: f64,
    pub lane_position: f64,
    pub speed: f64,
    pub hp: f64,
    pub is_tempted: bool,
    pub is_blocked: bool,
}

impl PartialEq for ReplayHorseData {
    fn eq(&self, other: &Self) -> bool {
        self.distance.sub(other.distance).abs() < EPSILON
            && self.lane_position.sub(other.lane_position).abs() < EPSILON
            && self.speed.sub(other.speed).abs() < EPSILON
            && self.hp.sub(other.hp).abs() < EPSILON
            && self.is_tempted == other.is_tempted
            && self.is_blocked == other.is_blocked
    }
}

impl Eq for ReplayHorseData {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEvent {
    pub frame_time: f64,
    #[serde(rename = "type")]
    pub event_type: i64,
    pub param: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub frame_time: f64,
    pub horse_idx: Option<i64>,
    pub event_data: Option<ReplayEventData>,
}

impl PartialEq for ReplayEvent {
    fn eq(&self, other: &Self) -> bool {
        self.frame_time.sub(other.frame_time).abs() < EPSILON
            && self.horse_idx == other.horse_idx
            && self.event_data == other.event_data
    }
}

impl Eq for ReplayEvent {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplayEventData {
    Score,
    Skill(String),
    CompTop,
    CompFight,
    RelCons,
    StamBrk,
    CompSpurt,
    StamKeep,
    SecLead,
}

impl ReplayEventData {
    pub const TYPE_ID_SCORE: i64 = 0;
    pub const TYPE_ID_SKILL: i64 = 3;
    pub const TYPE_ID_COMP_TOP: i64 = 4;
    pub const TYPE_ID_COMP_FIGHT: i64 = 5;
    pub const TYPE_ID_REL_CONS: i64 = 6;
    pub const TYPE_ID_STAM_BRK: i64 = 7;
    pub const TYPE_ID_COMP_SPURT: i64 = 8;
    pub const TYPE_ID_STAM_KEEP: i64 = 9;
    pub const TYPE_ID_SEC_LEAD: i64 = 10;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceDumpCharacter {
    pub character_name: String,
    pub trainee_name: Option<String>,
    pub is_player: bool,
    pub stat_speed: u32,
    pub stat_stamina: u32,
    pub stat_power: u32,
    pub stat_guts: u32,
    pub stat_wit: u32,
    pub trainee_id: Option<u64>,
    pub post_number: u32,
    pub finish_order: u32,
    pub popularity: u8,
    pub finish_time: f64,
    pub running_style: RunningStyle,
    pub viewer_id: Option<u64>,
    pub team_id: Option<u8>,
}
