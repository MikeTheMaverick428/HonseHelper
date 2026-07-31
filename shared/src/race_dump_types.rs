use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::{GroundCondition, RaceDistance, ReplayEvent, ReplayFrame};
use crate::veteran_browser::{
    deserialize_i64_from_string, serialize_i64_as_string, SortConfig, TagRow,
};

// ── RaceDumpSummary ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpSummary {
    pub id: i64,
    pub capture_time: String,
    pub race_type: i64,
    pub race_instance_id: Option<i64>,
    pub race_id: Option<i64>,
    pub distance: Option<i64>,
    pub track_id: Option<i64>,
    pub ground_type: Option<i64>,
    pub season: Option<i64>,
    pub weather: Option<i64>,
    pub ground_condition: Option<i64>,
    pub turn: Option<i64>,
    pub inout: Option<i64>,
    #[serde(default)]
    pub champions_id: Option<i64>,
    #[serde(default)]
    pub league_type: Option<i64>,
    #[serde(default)]
    pub round: Option<i64>,
    pub participant_count: i64,
    pub player_participant_count: i64,
    pub player_participants: Vec<String>,
    pub race_name: Option<String>,
    pub track_name: Option<String>,
    #[serde(default)]
    pub tags: Vec<super::veteran_browser::TagRow>,
}

// ── RaceDumpParticipant ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpParticipant {
    pub horse_index: i64,
    pub post_number: i64,
    pub chara_name: Option<String>,
    pub is_player: i64,
    pub finish_order: Option<i64>,
    pub finish_time: Option<f64>,
    pub running_style: Option<i64>,
    pub speed: Option<i64>,
    pub stamina: Option<i64>,
    pub pow: Option<i64>,
    pub guts: Option<i64>,
    pub wiz: Option<i64>,
    #[serde(default)]
    pub viewer_id: Option<i64>,
    #[serde(default)]
    #[serde(
        serialize_with = "crate::veteran_browser::serialize_option_i64_as_string",
        deserialize_with = "crate::veteran_browser::deserialize_option_i64_from_string"
    )]
    pub veteran_hash: Option<i64>,
}

// ── RaceDumpDetail ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpDetail {
    pub summary: RaceDumpSummary,
    pub participants: Vec<RaceDumpParticipant>,
    pub frames: Vec<ReplayFrame>,
    pub events: Vec<ReplayEvent>,
}

// ── RaceType ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum RaceType {
    None = 0,
    PvP = 1,
    Tutorial = 2,
    Story = 3,
    StoryCondition = 4,
    Champions = 5,
    Single = 6,
    SingleModeScenarioTeamRace = 7,
    RoomMatch = 8,
    Practice = 9,
    Daily = 10,
    TeamBuilding = 11,
    Legend = 12,
    ChallengeMatch = 13,
    TeamStadium = 14,
}

impl RaceType {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::PvP => "PvP",
            Self::Tutorial => "Tutorial",
            Self::Story => "Story",
            Self::StoryCondition => "StoryCondition",
            Self::Champions => "Champions",
            Self::Single => "Single",
            Self::SingleModeScenarioTeamRace => "ScenarioTeamRace",
            Self::RoomMatch => "RoomMatch",
            Self::Practice => "Practice",
            Self::Daily => "Daily",
            Self::TeamBuilding => "TeamBuilding",
            Self::Legend => "Legend",
            Self::ChallengeMatch => "ChallengeMatch",
            Self::TeamStadium => "TeamStadium",
        }
    }
}

impl std::fmt::Display for RaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl TryFrom<i64> for RaceType {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::PvP),
            2 => Ok(Self::Tutorial),
            3 => Ok(Self::Story),
            4 => Ok(Self::StoryCondition),
            5 => Ok(Self::Champions),
            6 => Ok(Self::Single),
            7 => Ok(Self::SingleModeScenarioTeamRace),
            8 => Ok(Self::RoomMatch),
            9 => Ok(Self::Practice),
            10 => Ok(Self::Daily),
            11 => Ok(Self::TeamBuilding),
            12 => Ok(Self::Legend),
            13 => Ok(Self::ChallengeMatch),
            14 => Ok(Self::TeamStadium),
            _ => Err(format!("unknown RaceType value: {v}")),
        }
    }
}

impl From<RaceType> for i64 {
    fn from(t: RaceType) -> Self {
        t as i64
    }
}

impl Serialize for RaceType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(*self as i64)
    }
}

impl<'de> Deserialize<'de> for RaceType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i64::deserialize(d)?;
        Self::try_from(v).map_err(serde::de::Error::custom)
    }
}

// ── Weather ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum Weather {
    None = 0,
    Sunny = 1,
    Rainy = 2,
    Snow = 3,
    Cloudy = 4,
    Star = 5,
    Firework = 6,
}

impl Weather {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Sunny => "Sunny",
            Self::Rainy => "Rainy",
            Self::Snow => "Snow",
            Self::Cloudy => "Cloudy",
            Self::Star => "Star",
            Self::Firework => "Firework",
        }
    }
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl TryFrom<i64> for Weather {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Sunny),
            2 => Ok(Self::Rainy),
            3 => Ok(Self::Snow),
            4 => Ok(Self::Cloudy),
            5 => Ok(Self::Star),
            6 => Ok(Self::Firework),
            _ => Err(format!("unknown Weather value: {v}")),
        }
    }
}

impl From<Weather> for i64 {
    fn from(w: Weather) -> Self {
        w as i64
    }
}

impl Serialize for Weather {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(*self as i64)
    }
}

impl<'de> Deserialize<'de> for Weather {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i64::deserialize(d)?;
        Self::try_from(v).map_err(serde::de::Error::custom)
    }
}

// ── GroundType ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum GroundType {
    Undefined = 0,
    Turf = 1,
    Dirt = 2,
}

impl GroundType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Undefined => "Undefined",
            Self::Turf => "Turf",
            Self::Dirt => "Dirt",
        }
    }
}

impl std::fmt::Display for GroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl TryFrom<i64> for GroundType {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Undefined),
            1 => Ok(Self::Turf),
            2 => Ok(Self::Dirt),
            _ => Err(format!("unknown GroundType value: {v}")),
        }
    }
}

impl From<GroundType> for i64 {
    fn from(g: GroundType) -> Self {
        g as i64
    }
}

impl Serialize for GroundType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(*self as i64)
    }
}

impl<'de> Deserialize<'de> for GroundType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i64::deserialize(d)?;
        Self::try_from(v).map_err(serde::de::Error::custom)
    }
}

// ── Season ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum Season {
    Random = 0,
    Spring = 1,
    Summer = 2,
    Fall = 3,
    Winter = 4,
    CherryBlossom = 5,
}

impl Season {
    pub fn label(self) -> &'static str {
        match self {
            Self::Random => "Random",
            Self::Spring => "Spring",
            Self::Summer => "Summer",
            Self::Fall => "Fall",
            Self::Winter => "Winter",
            Self::CherryBlossom => "CherryBlossom",
        }
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl TryFrom<i64> for Season {
    type Error = String;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Random),
            1 => Ok(Self::Spring),
            2 => Ok(Self::Summer),
            3 => Ok(Self::Fall),
            4 => Ok(Self::Winter),
            5 => Ok(Self::CherryBlossom),
            _ => Err(format!("unknown Season value: {v}")),
        }
    }
}

impl From<Season> for i64 {
    fn from(s: Season) -> Self {
        s as i64
    }
}

impl Serialize for Season {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(*self as i64)
    }
}

impl<'de> Deserialize<'de> for Season {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i64::deserialize(d)?;
        Self::try_from(v).map_err(serde::de::Error::custom)
    }
}

// ── Browser Types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpBrowserQuery {
    pub filters: Vec<RaceDumpFilter>,
    pub sort: SortConfig,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type", content = "value")]
pub enum RaceDumpFilter {
    RaceType(RaceType),
    DistanceMeters {
        min: Option<i64>,
        max: Option<i64>,
    },
    Distance(RaceDistance),
    GroundType(GroundType),
    Season(Season),
    Weather(Weather),
    GroundCondition(GroundCondition),
    Character(i64),
    Trainee(i64),
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    VeteranHash(i64),
    HasTag(String),
    CaptureDate(crate::date_time::DateTimeRange),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpPageItem {
    pub summary: RaceDumpSummary,
    pub race_name: Option<String>,
    pub tags: Vec<TagRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaceDumpFilterOptions {
    pub race_types: Vec<(i64, String)>,
    pub seasons: Vec<(i64, String)>,
    pub weather_types: Vec<(i64, String)>,
    pub ground_types: Vec<(i64, String)>,
    pub ground_conditions: Vec<(i64, String)>,
    pub characters: Vec<(i64, String)>,
    pub trainees: Vec<(i64, String)>,
    pub tags: Vec<String>,
    pub distance_min: i64,
    pub distance_max: i64,
}
