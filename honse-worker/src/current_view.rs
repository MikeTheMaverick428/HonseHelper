use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrentViewModel {
    pub current_known_view: Option<KnownView>,
    pub current_scene_base_ptr: Option<String>, // "0x..." hex string
    pub current_scene_class: Option<String>,
    pub current_view_base_ptr: Option<String>, // "0x..." hex string
    pub current_view_class: Option<String>,
    pub changed: bool,
    pub last_known_view: Option<KnownView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnownView {
    HomeHub,
    SingleModeSelect,
    Career,
    Race,
    TeamStadium,
    FriendList,
    VeteranView,
    SupportCardList,
    DailyLegendRace,
    DailyLegendSelection,
}

impl KnownView {
    pub fn from_raw(view_id: i32, _kclass: &str) -> Option<Self> {
        match view_id {
            100 => Some(Self::HomeHub),
            1000 => Some(Self::SingleModeSelect),
            1101 => Some(Self::Career),
            400 => Some(Self::Race),
            4000 => Some(Self::TeamStadium),
            5100 => Some(Self::VeteranView),
            5200 => Some(Self::FriendList),
            5510 => Some(Self::SupportCardList),
            5620 => Some(Self::DailyLegendRace),
            5650 => Some(Self::DailyLegendSelection),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::HomeHub => "HomeHub",
            Self::SingleModeSelect => "SingleModeSelect",
            Self::Career => "Career",
            Self::Race => "Race",
            Self::TeamStadium => "TeamStadium",
            Self::FriendList => "FriendList",
            Self::VeteranView => "VeteranView",
            Self::SupportCardList => "SupportCardList",
            Self::DailyLegendRace => "DailyLegendRace",
            Self::DailyLegendSelection => "DailyLegendSelection",
        }
    }
}
