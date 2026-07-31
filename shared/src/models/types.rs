use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportCardType {
    Speed,
    Stamina,
    Power,
    Guts,
    Wisdom,
    Friend,
    Group,
    Unknown,
}

// impl From<i64> for SupportCardType {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			1 => SupportCardType::Speed,
// 			2 => SupportCardType::Stamina,
// 			3 => SupportCardType::Power,
// 			4 => SupportCardType::Guts,
// 			5 => SupportCardType::Wisdom,
// 			6 => SupportCardType::Friend,
// 			7 => SupportCardType::Group,
// 			_ => SupportCardType::Unknown,
// 		}
// 	}
// }

impl SupportCardType {
    pub fn from_raw(value: i64) -> Self {
        match value {
            1 => SupportCardType::Speed,
            2 => SupportCardType::Stamina,
            3 => SupportCardType::Power,
            4 => SupportCardType::Guts,
            5 => SupportCardType::Wisdom,
            6 => SupportCardType::Friend,
            7 => SupportCardType::Group,
            _ => SupportCardType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportCardRarity {
    R,
    Sr,
    Ssr,
    Unknown,
}

// impl From<i64> for SupportCardRarity {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			1 => SupportCardRarity::R,
// 			2 => SupportCardRarity::Sr,
// 			3 => SupportCardRarity::Ssr,
// 			_ => SupportCardRarity::Unknown,
// 		}
// 	}
// }

impl SupportCardRarity {
    pub fn from_raw(value: i64) -> Self {
        match value {
            1 => SupportCardRarity::R,
            2 => SupportCardRarity::Sr,
            3 => SupportCardRarity::Ssr,
            _ => SupportCardRarity::Unknown,
        }
    }

    pub fn max_level(&self) -> i64 {
        match self {
            SupportCardRarity::R => 40,
            SupportCardRarity::Sr => 45,
            SupportCardRarity::Ssr => 50,
            SupportCardRarity::Unknown => 50,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FavouriteIcon {
    Carrot = 0,
    Rice = 1,
    Juice = 2,
    Brownie = 3,
    Cake = 4,
    Diamond = 5,
    Spade = 6,
    Heart = 7,
    Trefl = 8,
    PinkShoe = 9,
    GreenShoe = 10,
    YellowShoe = 11,
    BlueShoe = 12,
    RedShoe = 13,
    HandShake = 14,
}

impl FavouriteIcon {
    pub fn label(&self) -> &'static str {
        match self {
            FavouriteIcon::Carrot => "Carrot",
            FavouriteIcon::Rice => "Rice",
            FavouriteIcon::Juice => "Juice",
            FavouriteIcon::Brownie => "Brownie",
            FavouriteIcon::Cake => "Cake",
            FavouriteIcon::Diamond => "Diamond",
            FavouriteIcon::Spade => "Spade",
            FavouriteIcon::Heart => "Heart",
            FavouriteIcon::Trefl => "Trefl",
            FavouriteIcon::PinkShoe => "Pink Shoe",
            FavouriteIcon::GreenShoe => "Green Shoe",
            FavouriteIcon::YellowShoe => "Yellow Shoe",
            FavouriteIcon::BlueShoe => "Blue Shoe",
            FavouriteIcon::RedShoe => "Red Shoe",
            FavouriteIcon::HandShake => "Hand Shake",
        }
    }
}

impl TryFrom<i16> for FavouriteIcon {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FavouriteIcon::Carrot),
            1 => Ok(FavouriteIcon::Rice),
            2 => Ok(FavouriteIcon::Juice),
            3 => Ok(FavouriteIcon::Brownie),
            4 => Ok(FavouriteIcon::Cake),
            5 => Ok(FavouriteIcon::Diamond),
            6 => Ok(FavouriteIcon::Spade),
            7 => Ok(FavouriteIcon::Heart),
            8 => Ok(FavouriteIcon::Trefl),
            9 => Ok(FavouriteIcon::PinkShoe),
            10 => Ok(FavouriteIcon::GreenShoe),
            11 => Ok(FavouriteIcon::YellowShoe),
            12 => Ok(FavouriteIcon::BlueShoe),
            13 => Ok(FavouriteIcon::RedShoe),
            14 => Ok(FavouriteIcon::HandShake),
            _ => Err(format!("Invalid FavouriteIcon value: {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SuccessionUmaPosition {
    Parent1,
    Parent2,
    Grandparent1Parent1,
    Grandparent1Parent2,
    Grandparent2Parent1,
    Grandparent2Parent2,
}

impl Hash for SuccessionUmaPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}

// impl From<i64> for SuccessionUmaPosition {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			10 => SuccessionUmaPosition::Parent1,
// 			20 => SuccessionUmaPosition::Parent2,
// 			11 => SuccessionUmaPosition::Grandparent1Parent1,
// 			12 => SuccessionUmaPosition::Grandparent1Parent2,
// 			21 => SuccessionUmaPosition::Grandparent2Parent1,
// 			22 => SuccessionUmaPosition::Grandparent2Parent2,
// 			_ => panic!("Invalid SuccessionUmaPosition value: {}", value),
// 		}
// 	}
// }

impl SuccessionUmaPosition {
    pub fn from_raw(value: i64) -> Self {
        match value {
            10 => SuccessionUmaPosition::Parent1,
            20 => SuccessionUmaPosition::Parent2,
            11 => SuccessionUmaPosition::Grandparent1Parent1,
            12 => SuccessionUmaPosition::Grandparent1Parent2,
            21 => SuccessionUmaPosition::Grandparent2Parent1,
            22 => SuccessionUmaPosition::Grandparent2Parent2,
            _ => panic!("Invalid SuccessionUmaPosition value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i8)]
pub enum AptitudeLevel {
    H = 0,
    G = 1,
    F = 2,
    E = 3,
    D = 4,
    C = 5,
    B = 6,
    A = 7,
    S = 8,
}

impl AptitudeLevel {
    pub fn increase(&mut self) {
        *self = match self {
            AptitudeLevel::H => AptitudeLevel::G,
            AptitudeLevel::G => AptitudeLevel::F,
            AptitudeLevel::F => AptitudeLevel::E,
            AptitudeLevel::E => AptitudeLevel::D,
            AptitudeLevel::D => AptitudeLevel::C,
            AptitudeLevel::C => AptitudeLevel::B,
            AptitudeLevel::B => AptitudeLevel::A,
            AptitudeLevel::A => AptitudeLevel::S,
            AptitudeLevel::S => AptitudeLevel::S,
        }
    }
}

impl ToString for AptitudeLevel {
    fn to_string(&self) -> String {
        match self {
            AptitudeLevel::S => "S".to_string(),
            AptitudeLevel::A => "A".to_string(),
            AptitudeLevel::B => "B".to_string(),
            AptitudeLevel::C => "C".to_string(),
            AptitudeLevel::D => "D".to_string(),
            AptitudeLevel::E => "E".to_string(),
            AptitudeLevel::F => "F".to_string(),
            AptitudeLevel::G => "G".to_string(),
            AptitudeLevel::H => "H".to_string(),
        }
    }
}

// impl From<i64> for AptitudeLevel {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			8 => AptitudeLevel::S,
// 			7 => AptitudeLevel::A,
// 			6 => AptitudeLevel::B,
// 			5 => AptitudeLevel::C,
// 			4 => AptitudeLevel::D,
// 			3 => AptitudeLevel::E,
// 			2 => AptitudeLevel::F,
// 			1 => AptitudeLevel::G,
// 			0 => AptitudeLevel::H,
// 			_ => panic!("Invalid AptitudeLevel value: {}", value),
// 		}
// 	}
// }

impl AptitudeLevel {
    pub fn from_raw(value: i64) -> Self {
        match value {
            8 => AptitudeLevel::S,
            7 => AptitudeLevel::A,
            6 => AptitudeLevel::B,
            5 => AptitudeLevel::C,
            4 => AptitudeLevel::D,
            3 => AptitudeLevel::E,
            2 => AptitudeLevel::F,
            1 => AptitudeLevel::G,
            0 => AptitudeLevel::H,
            _ => panic!("Invalid AptitudeLevel value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RaceGrade {
    G1 = 100,
    G2 = 200,
    G3 = 300,
    Op = 400,
    PreOp = 700,
    Maiden = 800,
    Debut = 900,
}

impl From<i32> for RaceGrade {
    fn from(value: i32) -> Self {
        match value {
            100 => Self::G1,
            200 => Self::G2,
            300 => Self::G3,
            400 => Self::Op,
            700 => Self::PreOp,
            800 => Self::Maiden,
            900 => Self::Debut,
            _ => Self::Maiden,
        }
    }
}

impl RaceGrade {
    pub fn from_raw(value: i32) -> Self {
        match value {
            100 => RaceGrade::G1,
            200 => RaceGrade::G2,
            300 => RaceGrade::G3,
            400 => RaceGrade::Op,
            700 => RaceGrade::PreOp,
            800 => RaceGrade::Maiden,
            900 => RaceGrade::Debut,
            _ => RaceGrade::Maiden,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Ground {
    Turf = 1,
    Dirt = 2,
}

impl From<u8> for Ground {
    fn from(value: u8) -> Self {
        match value {
            2 => Ground::Dirt,
            _ => Ground::Turf,
        }
    }
}

impl Ground {
    pub fn from_raw(value: i32) -> Self {
        match value {
            1 => Ground::Turf,
            2 => Ground::Dirt,
            _ => Ground::Turf,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeatherType {
    Sunny = 0,
    Cloudy = 1,
    Rainy = 2,
    Snowy = 3,
}

// impl From<i64> for WeatherType {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			0 => WeatherType::Sunny,
// 			1 => WeatherType::Cloudy,
// 			2 => WeatherType::Rainy,
// 			3 => WeatherType::Snowy,
// 			_ => WeatherType::Sunny,
// 		}
// 	}
// }

impl WeatherType {
    pub fn from_raw(value: i64) -> Self {
        match value {
            0 => WeatherType::Sunny,
            1 => WeatherType::Cloudy,
            2 => WeatherType::Rainy,
            3 => WeatherType::Snowy,
            _ => WeatherType::Sunny,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroundCondition {
    Firm = 1,
    Good = 2,
    Soft = 3,
    Heavy = 4,
}

// impl From<i64> for GroundCondition {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			1 => GroundCondition::Firm,
// 			2 => GroundCondition::Good,
// 			3 => GroundCondition::Soft,
// 			4 => GroundCondition::Heavy,
// 			_ => GroundCondition::Good,
// 		}
// 	}
// }

impl GroundCondition {
    pub fn from_raw(value: i64) -> Self {
        match value {
            1 => GroundCondition::Firm,
            2 => GroundCondition::Good,
            3 => GroundCondition::Soft,
            4 => GroundCondition::Heavy,
            _ => GroundCondition::Good,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RunningStyle {
    Front = 1,
    PaceChaser = 2,
    LateSurger = 3,
    EndCloser = 4,
}

// impl From<i64> for RunningStyle {
// 	fn from(value: i64) -> Self {
// 		match value {
// 			1 => RunningStyle::Front,
// 			2 => RunningStyle::PaceChaser,
// 			3 => RunningStyle::LateSurger,
// 			4 => RunningStyle::EndCloser,
// 			_ => RunningStyle::Front,
// 		}
// 	}
// }

impl RunningStyle {
    pub fn from_raw(value: i64) -> Self {
        match value {
            1 => RunningStyle::Front,
            2 => RunningStyle::PaceChaser,
            3 => RunningStyle::LateSurger,
            4 => RunningStyle::EndCloser,
            _ => RunningStyle::Front,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UmaRank {
    G = 1,
    Gplus = 2,
    F = 3,
    Fplus = 4,
    E = 5,
    Eplus = 6,
    D = 7,
    Dplus = 8,
    C = 9,
    Cplus = 10,
    B = 11,
    Bplus = 12,
    A = 13,
    Aplus = 14,
    S = 15,
    Splus = 16,
    SS = 17,
    SSplus = 18,
    UG = 19,
    UG1 = 20,
    UG2 = 21,
    UG3 = 22,
    UG4 = 23,
    UG5 = 24,
    UG6 = 25,
    UG7 = 26,
    UG8 = 27,
    UG9 = 28,
    UF = 29,
    UF1 = 30,
    UF2 = 31,
    UF3 = 32,
    UF4 = 33,
    UF5 = 34,
    UF6 = 35,
    UF7 = 36,
    UF8 = 37,
    UF9 = 38,
    UE = 39,
    UE1 = 40,
    UE2 = 41,
    UE3 = 42,
    UE4 = 43,
    UE5 = 44,
    UE6 = 45,
    UE7 = 46,
    UE8 = 47,
    UE9 = 48,
    UD = 49,
    UD1 = 50,
    UD2 = 51,
    UD3 = 52,
    UD4 = 53,
    UD5 = 54,
    UD6 = 55,
    UD7 = 56,
    UD8 = 57,
    UD9 = 58,
    UC = 59,
    UC1 = 60,
    UC2 = 61,
    UC3 = 62,
    UC4 = 63,
    UC5 = 64,
    UC6 = 65,
    UC7 = 66,
    UC8 = 67,
    UC9 = 68,
    UB = 69,
    UB1 = 70,
    UB2 = 71,
    UB3 = 72,
    UB4 = 73,
    UB5 = 74,
    UB6 = 75,
    UB7 = 76,
    UB8 = 77,
    UB9 = 78,
    UA = 79,
    UA1 = 80,
    UA2 = 81,
    UA3 = 82,
    UA4 = 83,
    UA5 = 84,
    UA6 = 85,
    UA7 = 86,
    UA8 = 87,
    UA9 = 88,
    US = 89,
    US1 = 90,
    US2 = 91,
    US3 = 92,
    US4 = 93,
    US5 = 94,
    US6 = 95,
    US7 = 96,
    US8 = 97,
    US9 = 98,
    UNKNOWN = 999,
}

const RANK_SCORE_MAXES: [u32; 98] = [
    299, 599, 899, 1299, 1799, 2299, 2899, 3499, 4899, 6499,
    8199, 9999, 12099, 14499, 15899, 17499, 19199, 19599, 19999, 20399,
    20799, 21199, 21599, 22099, 22499, 22999, 23399, 23899, 24299, 24799,
    25299, 25799, 26299, 26799, 27299, 27799, 28299, 28799, 29399, 29899,
    30399, 30999, 31499, 32099, 32699, 33199, 33799, 34399, 34999, 35599,
    36199, 36799, 37499, 38099, 38699, 39399, 39999, 40699, 41299, 41999,
    42699, 43399, 43999, 44699, 45399, 46199, 46899, 47599, 48299, 48999,
    49799, 50499, 51299, 51999, 52799, 53599, 54399, 55199, 55899, 56699,
    57499, 58399, 59199, 59999, 60799, 61699, 62499, 63399, 64199, 65099,
    65999, 66799, 67699, 68599, 69499, 70399, 71399, 99999,
];

impl UmaRank {
    pub fn from_score(score: u32) -> Self {
        for (i, &max) in RANK_SCORE_MAXES.iter().enumerate() {
            if score <= max {
                return Self::from_raw((i + 1) as u16);
            }
        }
        Self::UNKNOWN
    }
}

impl Into<u16> for UmaRank {
    fn into(self) -> u16 {
        self as u16
    }
}

impl UmaRank {
    pub fn label(&self) -> &'static str {
        match self {
            UmaRank::G => "G",
            UmaRank::Gplus => "G+",
            UmaRank::F => "F",
            UmaRank::Fplus => "F+",
            UmaRank::E => "E",
            UmaRank::Eplus => "E+",
            UmaRank::D => "D",
            UmaRank::Dplus => "D+",
            UmaRank::C => "C",
            UmaRank::Cplus => "C+",
            UmaRank::B => "B",
            UmaRank::Bplus => "B+",
            UmaRank::A => "A",
            UmaRank::Aplus => "A+",
            UmaRank::S => "S",
            UmaRank::Splus => "S+",
            UmaRank::SS => "SS",
            UmaRank::SSplus => "SS+",
            UmaRank::UG => "UG",
            UmaRank::UG1 => "UG1",
            UmaRank::UG2 => "UG2",
            UmaRank::UG3 => "UG3",
            UmaRank::UG4 => "UG4",
            UmaRank::UG5 => "UG5",
            UmaRank::UG6 => "UG6",
            UmaRank::UG7 => "UG7",
            UmaRank::UG8 => "UG8",
            UmaRank::UG9 => "UG9",
            UmaRank::UF => "UF",
            UmaRank::UF1 => "UF1",
            UmaRank::UF2 => "UF2",
            UmaRank::UF3 => "UF3",
            UmaRank::UF4 => "UF4",
            UmaRank::UF5 => "UF5",
            UmaRank::UF6 => "UF6",
            UmaRank::UF7 => "UF7",
            UmaRank::UF8 => "UF8",
            UmaRank::UF9 => "UF9",
            UmaRank::UE => "UE",
            UmaRank::UE1 => "UE1",
            UmaRank::UE2 => "UE2",
            UmaRank::UE3 => "UE3",
            UmaRank::UE4 => "UE4",
            UmaRank::UE5 => "UE5",
            UmaRank::UE6 => "UE6",
            UmaRank::UE7 => "UE7",
            UmaRank::UE8 => "UE8",
            UmaRank::UE9 => "UE9",
            UmaRank::UD => "UD",
            UmaRank::UD1 => "UD1",
            UmaRank::UD2 => "UD2",
            UmaRank::UD3 => "UD3",
            UmaRank::UD4 => "UD4",
            UmaRank::UD5 => "UD5",
            UmaRank::UD6 => "UD6",
            UmaRank::UD7 => "UD7",
            UmaRank::UD8 => "UD8",
            UmaRank::UD9 => "UD9",
            UmaRank::UC => "UC",
            UmaRank::UC1 => "UC1",
            UmaRank::UC2 => "UC2",
            UmaRank::UC3 => "UC3",
            UmaRank::UC4 => "UC4",
            UmaRank::UC5 => "UC5",
            UmaRank::UC6 => "UC6",
            UmaRank::UC7 => "UC7",
            UmaRank::UC8 => "UC8",
            UmaRank::UC9 => "UC9",
            UmaRank::UB => "UB",
            UmaRank::UB1 => "UB1",
            UmaRank::UB2 => "UB2",
            UmaRank::UB3 => "UB3",
            UmaRank::UB4 => "UB4",
            UmaRank::UB5 => "UB5",
            UmaRank::UB6 => "UB6",
            UmaRank::UB7 => "UB7",
            UmaRank::UB8 => "UB8",
            UmaRank::UB9 => "UB9",
            UmaRank::UA => "UA",
            UmaRank::UA1 => "UA1",
            UmaRank::UA2 => "UA2",
            UmaRank::UA3 => "UA3",
            UmaRank::UA4 => "UA4",
            UmaRank::UA5 => "UA5",
            UmaRank::UA6 => "UA6",
            UmaRank::UA7 => "UA7",
            UmaRank::UA8 => "UA8",
            UmaRank::UA9 => "UA9",
            UmaRank::US => "US",
            UmaRank::US1 => "US1",
            UmaRank::US2 => "US2",
            UmaRank::US3 => "US3",
            UmaRank::US4 => "US4",
            UmaRank::US5 => "US5",
            UmaRank::US6 => "US6",
            UmaRank::US7 => "US7",
            UmaRank::US8 => "US8",
            UmaRank::US9 => "US9",
            UmaRank::UNKNOWN => "Unknown",
        }
    }

    pub fn from_raw(value: u16) -> Self {
        match value {
            1 => UmaRank::G,
            2 => UmaRank::Gplus,
            3 => UmaRank::F,
            4 => UmaRank::Fplus,
            5 => UmaRank::E,
            6 => UmaRank::Eplus,
            7 => UmaRank::D,
            8 => UmaRank::Dplus,
            9 => UmaRank::C,
            10 => UmaRank::Cplus,
            11 => UmaRank::B,
            12 => UmaRank::Bplus,
            13 => UmaRank::A,
            14 => UmaRank::Aplus,
            15 => UmaRank::S,
            16 => UmaRank::Splus,
            17 => UmaRank::SS,
            18 => UmaRank::SSplus,
            19 => UmaRank::UG,
            20 => UmaRank::UG1,
            21 => UmaRank::UG2,
            22 => UmaRank::UG3,
            23 => UmaRank::UG4,
            24 => UmaRank::UG5,
            25 => UmaRank::UG6,
            26 => UmaRank::UG7,
            27 => UmaRank::UG8,
            28 => UmaRank::UG9,
            29 => UmaRank::UF,
            30 => UmaRank::UF1,
            31 => UmaRank::UF2,
            32 => UmaRank::UF3,
            33 => UmaRank::UF4,
            34 => UmaRank::UF5,
            35 => UmaRank::UF6,
            36 => UmaRank::UF7,
            37 => UmaRank::UF8,
            38 => UmaRank::UF9,
            39 => UmaRank::UE,
            40 => UmaRank::UE1,
            41 => UmaRank::UE2,
            42 => UmaRank::UE3,
            43 => UmaRank::UE4,
            44 => UmaRank::UE5,
            45 => UmaRank::UE6,
            46 => UmaRank::UE7,
            47 => UmaRank::UE8,
            48 => UmaRank::UE9,
            49 => UmaRank::UD,
            50 => UmaRank::UD1,
            51 => UmaRank::UD2,
            52 => UmaRank::UD3,
            53 => UmaRank::UD4,
            54 => UmaRank::UD5,
            55 => UmaRank::UD6,
            56 => UmaRank::UD7,
            57 => UmaRank::UD8,
            58 => UmaRank::UD9,
            59 => UmaRank::UC,
            60 => UmaRank::UC1,
            61 => UmaRank::UC2,
            62 => UmaRank::UC3,
            63 => UmaRank::UC4,
            64 => UmaRank::UC5,
            65 => UmaRank::UC6,
            66 => UmaRank::UC7,
            67 => UmaRank::UC8,
            68 => UmaRank::UC9,
            69 => UmaRank::UB,
            70 => UmaRank::UB1,
            71 => UmaRank::UB2,
            72 => UmaRank::UB3,
            73 => UmaRank::UB4,
            74 => UmaRank::UB5,
            75 => UmaRank::UB6,
            76 => UmaRank::UB7,
            77 => UmaRank::UB8,
            78 => UmaRank::UB9,
            79 => UmaRank::UA,
            80 => UmaRank::UA1,
            81 => UmaRank::UA2,
            82 => UmaRank::UA3,
            83 => UmaRank::UA4,
            84 => UmaRank::UA5,
            85 => UmaRank::UA6,
            86 => UmaRank::UA7,
            87 => UmaRank::UA8,
            88 => UmaRank::UA9,
            89 => UmaRank::US,
            90 => UmaRank::US1,
            91 => UmaRank::US2,
            92 => UmaRank::US3,
            93 => UmaRank::US4,
            94 => UmaRank::US5,
            95 => UmaRank::US6,
            96 => UmaRank::US7,
            97 => UmaRank::US8,
            98 => UmaRank::US9,
            _ => UmaRank::UNKNOWN,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(i8)]
pub enum SparkType {
    Stat = 1,
    Aptitude = 2,
    Unique = 3,
    Skill = 4,
    Race = 5,
    Scenario = 6,
    Event = 7,
}

impl SparkType {
    pub fn label(self) -> &'static str {
        match self {
            SparkType::Stat => "Stat",
            SparkType::Aptitude => "Aptitude",
            SparkType::Unique => "Unique",
            SparkType::Skill => "Skill",
            SparkType::Race => "Race",
            SparkType::Scenario => "Scenario",
            SparkType::Event => "Event",
        }
    }

    pub fn is_blue(self) -> bool {
        matches!(self, SparkType::Stat)
    }

    pub fn is_pink(self) -> bool {
        matches!(self, SparkType::Aptitude)
    }

    pub fn is_green(self) -> bool {
        matches!(self, SparkType::Unique)
    }

    pub fn is_white(self) -> bool {
        !self.is_blue() && !self.is_pink() && !self.is_green()
    }

    pub fn from_raw(value: i64) -> Self {
        match value {
            1 => SparkType::Stat,
            2 => SparkType::Aptitude,
            3 => SparkType::Unique,
            4 => SparkType::Skill,
            5 => SparkType::Race,
            6 => SparkType::Scenario,
            7 => SparkType::Event,
            _ => SparkType::Skill,
        }
    }

    pub fn into_raw(self) -> i64 {
        self as i64
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportCardEffectType {
    None = 0,
    SpecialTagEffectUp = 1,
    MotivationUp = 2,
    TrainingSpeedUp = 3,
    TrainingStaminaUp = 4,
    TrainingPowerUp = 5,
    TrainingGutzUp = 6,
    TrainingWizUp = 7,
    TrainingEffectUp = 8,
    InitialSpeedUp = 9,
    InitialStaminaUp = 10,
    InitialPowerUp = 11,
    InitialGutzUp = 12,
    InitialWizUp = 13,
    InitialEvaluationUp = 14,
    RaceStatusUp = 15,
    RaceFanUp = 16,
    SkillTipsLvUp = 17,
    SkillTipsEventRateUp = 18,
    GoodTrainingRateUp = 19,
    EventRecoveryAmountUp = 25,
    EventEffectUp = 26,
    TrainingFailureRateDown = 27,
    TrainingHPConsumptionDown = 28,
    SkillPointBonus = 30,
    WizRecoveryUp = 31,
}

impl SupportCardEffectType {
    pub fn from_raw(value: i64) -> Self {
        match value {
            0 => Self::None,
            1 => Self::SpecialTagEffectUp,
            2 => Self::MotivationUp,
            3 => Self::TrainingSpeedUp,
            4 => Self::TrainingStaminaUp,
            5 => Self::TrainingPowerUp,
            6 => Self::TrainingGutzUp,
            7 => Self::TrainingWizUp,
            8 => Self::TrainingEffectUp,
            9 => Self::InitialSpeedUp,
            10 => Self::InitialStaminaUp,
            11 => Self::InitialPowerUp,
            12 => Self::InitialGutzUp,
            13 => Self::InitialWizUp,
            14 => Self::InitialEvaluationUp,
            15 => Self::RaceStatusUp,
            16 => Self::RaceFanUp,
            17 => Self::SkillTipsLvUp,
            18 => Self::SkillTipsEventRateUp,
            19 => Self::GoodTrainingRateUp,
            25 => Self::EventRecoveryAmountUp,
            26 => Self::EventEffectUp,
            27 => Self::TrainingFailureRateDown,
            28 => Self::TrainingHPConsumptionDown,
            30 => Self::SkillPointBonus,
            31 => Self::WizRecoveryUp,
            _ => Self::None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SpecialTagEffectUp => "Friendship bonus",
            Self::MotivationUp => "Mood up",
            Self::TrainingSpeedUp => "Training Speed",
            Self::TrainingStaminaUp => "Training Stamina",
            Self::TrainingPowerUp => "Training Power",
            Self::TrainingGutzUp => "Training Guts",
            Self::TrainingWizUp => "Training Wisdom",
            Self::TrainingEffectUp => "Training Effect",
            Self::InitialSpeedUp => "Initial Speed",
            Self::InitialStaminaUp => "Initial Stamina",
            Self::InitialPowerUp => "Initial Power",
            Self::InitialGutzUp => "Initial Guts",
            Self::InitialWizUp => "Initial Wisdom",
            Self::InitialEvaluationUp => "Initial friendship gauge",
            Self::RaceStatusUp => "Race Bonus",
            Self::RaceFanUp => "Race Fans",
            Self::SkillTipsLvUp => "Skill Hint Level",
            Self::SkillTipsEventRateUp => "Skill Hint Rate",
            Self::GoodTrainingRateUp => "Specialty priority",
            Self::EventRecoveryAmountUp => "Event Recovery",
            Self::EventEffectUp => "Event effectiveness",
            Self::TrainingFailureRateDown => "Training Failure Rate",
            Self::TrainingHPConsumptionDown => "Training energy reduction",
            Self::SkillPointBonus => "Skill Point Bonus",
            Self::WizRecoveryUp => "Wit energy recovery",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::None => "(unused)",
            Self::SpecialTagEffectUp => "Friendship training bonus increase",
            Self::MotivationUp => "Bonus to mood effects during training",
            Self::TrainingSpeedUp => "Speed stat gain per training",
            Self::TrainingStaminaUp => "Stamina stat gain per training",
            Self::TrainingPowerUp => "Power stat gain per training",
            Self::TrainingGutzUp => "Guts stat gain per training",
            Self::TrainingWizUp => "Wisdom stat gain per training",
            Self::TrainingEffectUp => "Overall training multiplier",
            Self::InitialSpeedUp => "Starting speed stat",
            Self::InitialStaminaUp => "Starting stamina stat",
            Self::InitialPowerUp => "Starting power stat",
            Self::InitialGutzUp => "Starting guts stat",
            Self::InitialWizUp => "Starting wisdom stat",
            Self::InitialEvaluationUp => "Starting friendship with given support",
            Self::RaceStatusUp => "Bonus to stat gains after race",
            Self::RaceFanUp => "Bonus fan gain from races",
            Self::SkillTipsLvUp => "Skill hint level",
            Self::SkillTipsEventRateUp => "Skill hint event frequency",
            Self::GoodTrainingRateUp => "Increased chance to show up on matching training",
            Self::EventRecoveryAmountUp => "Bonus HP recovery from card events",
            Self::EventEffectUp => "Bonus to effect size from card events",
            Self::TrainingFailureRateDown => "Reduces failure chance during training",
            Self::TrainingHPConsumptionDown => "Reduces energy spent during training",
            Self::SkillPointBonus => "Skill point gain bonus",
            Self::WizRecoveryUp => "Energy recovery on wit friendship training",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewardType {
    Bond = 0,
    BondLong = 1,
    Speed = 2,
    Stamina = 3,
    Power = 4,
    Guts = 5,
    Wit = 6,
    SkillPts = 7,
    Energy = 8,
    MaxEnergy = 9,
    Motivation = 10,
    Skill = 11,
    SkillBundle = 12,
    ScenarioExp = 13,
    Hearts = 14,
    BondRace = 15,
    BondChara = 16,
    Brief = 17,
    BriefPositive = 18,
    Dash = 19,
    FiveStars = 20,
    Nl = 21,
    Sg = 22,
    TtlGauge = 23,
    TtlGaugeAll = 24,
    Ha = 25,
    Rs = 26,
    RaceResult = 27,
    Place = 28,
    FanEvent = 29,
}

impl RewardType {
    pub fn from_raw(value: i64) -> Self {
        match value {
            0 => Self::Bond,
            1 => Self::BondLong,
            2 => Self::Speed,
            3 => Self::Stamina,
            4 => Self::Power,
            5 => Self::Guts,
            6 => Self::Wit,
            7 => Self::SkillPts,
            8 => Self::Energy,
            9 => Self::MaxEnergy,
            10 => Self::Motivation,
            11 => Self::Skill,
            12 => Self::SkillBundle,
            13 => Self::ScenarioExp,
            14 => Self::Hearts,
            15 => Self::BondRace,
            16 => Self::BondChara,
            17 => Self::Brief,
            18 => Self::BriefPositive,
            19 => Self::Dash,
            20 => Self::FiveStars,
            21 => Self::Nl,
            22 => Self::Sg,
            23 => Self::TtlGauge,
            24 => Self::TtlGaugeAll,
            25 => Self::Ha,
            26 => Self::Rs,
            27 => Self::RaceResult,
            28 => Self::Place,
            29 => Self::FanEvent,
            _ => Self::Bond,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Bond => "Bond",
            Self::BondLong => "Bond (Long)",
            Self::Speed => "Speed",
            Self::Stamina => "Stamina",
            Self::Power => "Power",
            Self::Guts => "Guts",
            Self::Wit => "Wit",
            Self::SkillPts => "Skill Points",
            Self::Energy => "Energy",
            Self::MaxEnergy => "Max Energy",
            Self::Motivation => "Motivation",
            Self::Skill => "Skill Hint",
            Self::SkillBundle => "Skill Bundle",
            Self::ScenarioExp => "Scenario Exp",
            Self::Hearts => "Hearts",
            Self::BondRace => "Bond (Race)",
            Self::BondChara => "Bond (Chara)",
            Self::Brief => "Brief",
            Self::BriefPositive => "Brief Positive",
            Self::Dash => "Dash",
            Self::FiveStars => "All stats",
            Self::Nl => "Nl",
            Self::Sg => "Sg",
            Self::TtlGauge => "TTL Gauge",
            Self::TtlGaugeAll => "TTL Gauge All",
            Self::Ha => "Ha",
            Self::Rs => "Random stat",
            Self::RaceResult => "Race Result",
            Self::Place => "Place",
            Self::FanEvent => "Fan Event",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Bond => "Friendship points with support card",
            Self::BondLong => "Long-term friendship points",
            Self::Speed => "Speed stat points",
            Self::Stamina => "Stamina stat points",
            Self::Power => "Power stat points",
            Self::Guts => "Guts stat points",
            Self::Wit => "Wit stat points",
            Self::SkillPts => "Skill points",
            Self::Energy => "Energy (HP) recovery or loss",
            Self::MaxEnergy => "Maximum energy adjustment",
            Self::Motivation => "Motivation level change",
            Self::Skill => "Skill hint (skill_id identifies the skill)",
            Self::SkillBundle => "Skill bundle (flag)",
            Self::ScenarioExp => "Scenario experience (flag)",
            Self::Hearts => "Hearts indicator (flag)",
            Self::BondRace => "Bond points from race",
            Self::BondChara => "Bond points from character",
            Self::Brief => "Brief event (flag)",
            Self::BriefPositive => "Brief positive event (flag)",
            Self::Dash => "Dash event (flag)",
            Self::FiveStars => "Five stars indicator (flag)",
            Self::Nl => "Nl indicator",
            Self::Sg => "Sg indicator (flag)",
            Self::TtlGauge => "TTL gauge change",
            Self::TtlGaugeAll => "TTL gauge change (all)",
            Self::Ha => "Ha indicator (flag)",
            Self::Rs => "Rs indicator (flag)",
            Self::RaceResult => "Race result indicator",
            Self::Place => "Place indicator",
            Self::FanEvent => "Fan event indicator",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScenarioStatus {
    NightOwl = 1,
    Slacker = 2,
    SlowMetabolism = 4,
    PracticePoor = 6,
    FastLearner = 7,
    Charming = 8,
    HotTopic = 9,
    PracticePerfectCircle = 10,
    PracticePerfectDoubleCircle = 11,
    UnderTheWeather = 12,
    ShiningBrightly = 13,
    FanPromiseHokkaido = 14,
    FanPromiseHokuto = 15,
    FanPromiseNakayama = 16,
    FanPromiseKansai = 17,
    FanPromiseKokura = 18,
    NotReady = 19,
    LegsOfGlass = 20,
    OminiousPortent = 21,
    FanPromiseKawasaki = 22,
    PurePassionTeamSirius = 100,
    PurePassionHeirsToTheThrone = 101,
}

impl ScenarioStatus {
    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            1 => Some(Self::NightOwl),
            2 => Some(Self::Slacker),
            4 => Some(Self::SlowMetabolism),
            6 => Some(Self::PracticePoor),
            7 => Some(Self::FastLearner),
            8 => Some(Self::Charming),
            9 => Some(Self::HotTopic),
            10 => Some(Self::PracticePerfectCircle),
            11 => Some(Self::PracticePerfectDoubleCircle),
            12 => Some(Self::UnderTheWeather),
            13 => Some(Self::ShiningBrightly),
            14 => Some(Self::FanPromiseHokkaido),
            15 => Some(Self::FanPromiseHokuto),
            16 => Some(Self::FanPromiseNakayama),
            17 => Some(Self::FanPromiseKansai),
            18 => Some(Self::FanPromiseKokura),
            19 => Some(Self::NotReady),
            20 => Some(Self::LegsOfGlass),
            21 => Some(Self::OminiousPortent),
            22 => Some(Self::FanPromiseKawasaki),
            100 => Some(Self::PurePassionTeamSirius),
            101 => Some(Self::PurePassionHeirsToTheThrone),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::NightOwl => "Night Owl",
            Self::Slacker => "Slacker",
            Self::SlowMetabolism => "Slow Metabolism",
            Self::PracticePoor => "Practice Poor",
            Self::FastLearner => "Fast Learner",
            Self::Charming => "Charming",
            Self::HotTopic => "Hot Topic",
            Self::PracticePerfectCircle => "Practice: Perfect",
            Self::PracticePerfectDoubleCircle => "Practice: Perfect◎",
            Self::UnderTheWeather => "Under the Weather",
            Self::ShiningBrightly => "Shining Brightly",
            Self::FanPromiseHokkaido => "Fan Promise (Hokkaido)",
            Self::FanPromiseHokuto => "Fan Promise (Hokuto)",
            Self::FanPromiseNakayama => "Fan Promise (Nakayama)",
            Self::FanPromiseKansai => "Fan Promise (Kansai)",
            Self::FanPromiseKokura => "Fan Promise (Kokura)",
            Self::NotReady => "Not Ready",
            Self::LegsOfGlass => "Legs of Glass",
            Self::OminiousPortent => "Ominious Portent",
            Self::FanPromiseKawasaki => "Fan Promise (Kawasaki)",
            Self::PurePassionTeamSirius => "Team Sirius",
            Self::PurePassionHeirsToTheThrone => "Heirs to the Throne",
        }
    }

    pub fn negative(self) -> bool {
        matches!(
            self,
            Self::NightOwl
                | Self::Slacker
                | Self::SlowMetabolism
                | Self::PracticePoor
                | Self::UnderTheWeather
                | Self::NotReady
                | Self::LegsOfGlass
                | Self::OminiousPortent
        )
    }
}
