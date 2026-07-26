use crate::legacy_planner::SparkGroupInfo;

use super::{
    AptitudeLevel, FavouriteIcon, Ground, GroundCondition, RaceGrade, RunningStyle, SparkType,
    SuccessionUmaPosition, UmaRank, WeatherType,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

pub const INDEPENDENT_LEARNER_NICKNAME: i64 = 394;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Veteran {
    pub trainee_id: i64,
    pub scenario: u16,
    #[serde(default)]
    pub trained_chara_id: i64,
    #[serde(default)]
    pub favorite_icon_type: Option<FavouriteIcon>,
    #[serde(default)]
    pub favorite_memo: Option<String>,
    pub hash: u64,
    pub created_at: DateTime,
    pub rank: i64,
    pub trainee_name: String,
    pub stats: UmaStats,
    pub aptitudes: Aptitudes,
    pub sparks: Vec<Spark>,
    pub succession_umas: Vec<SuccessionUma>,
    pub race_results: Vec<RaceResult>,
    pub win_saddle_ids: Vec<i64>,
    pub major_wins: Vec<UmaMajorWins>,
    pub owner_id: u64,
}

impl PartialEq for Veteran {
    fn eq(&self, other: &Self) -> bool {
        self.trainee_id == other.trainee_id
            && self.created_at.to_seconds_from_unix_epoch()
                == other.created_at.to_seconds_from_unix_epoch()
            && self.stats == other.stats
    }
}

impl Eq for Veteran {}

impl Veteran {
    pub fn shared_major_wins_count(&self, both: bool) -> usize {
        self.major_wins
            .iter()
            .filter(|win| {
                if both {
                    win.shared_count > 1
                } else {
                    win.shared_count > 0
                }
            })
            .count()
    }

    pub fn spark_groups(&self, trainee_only: bool) -> BTreeMap<i64, SparkGroupInfo> {
        let mut sparks_map: BTreeMap<i64, SparkGroupInfo> = BTreeMap::new();

        for spark in &self.sparks {
            sparks_map
                .entry(spark.spark_group_id)
                .and_modify(|e| {
                    e.total_stars += spark.stars_count;
                    e.trainee_stars_veteran += spark.stars_count;
                    e.uma_count += 1;
                })
                .or_insert(SparkGroupInfo {
                    spark_group_id: spark.spark_group_id,
                    name: spark.name.clone(),
                    spark_type: spark.spark_type,
                    total_stars: spark.stars_count,
                    trainee_stars_veteran: if trainee_only { 0 } else { spark.stars_count },
                    uma_count: 1,
                });
        }

        if trainee_only {
            return sparks_map;
        }

        for succession_uma in &self.succession_umas {
            let is_parent = matches!(
                succession_uma.position,
                SuccessionUmaPosition::Parent1 | SuccessionUmaPosition::Parent2
            );
            if !is_parent {
                continue;
            }

            for spark in &succession_uma.sparks {
                sparks_map
                    .entry(spark.spark_group_id)
                    .and_modify(|e| {
                        e.total_stars += spark.stars_count;
                        e.uma_count += 1;
                    })
                    .or_insert(SparkGroupInfo {
                        spark_group_id: spark.spark_group_id,
                        name: spark.name.clone(),
                        spark_type: spark.spark_type,
                        total_stars: spark.stars_count,
                        trainee_stars_veteran: 0,
                        uma_count: 1,
                    });
            }
        }

        sparks_map
    }

    pub fn g1_wins(&self) -> usize {
        self.race_results
            .iter()
            .filter(|r| r.race_grade == RaceGrade::G1 && r.result_rank == 1)
            .count()
    }

    pub fn rank_grade(&self) -> UmaRank {
        UmaRank::from_raw(self.rank as u16)
    }

    // pub fn from_mssgpack_and_db(
    // 	mssgpack: &MssgPackTrainedChara,
    // 	favourites_by_trained_chara_id: &BTreeMap<i64, MssgPackFavouriteCharaItem>,
    // 	trainee_storage: &dyn crate::storage::ItemStorage<i64, crate::db::trainees::TraineeDataDbResult>,
    // 	spark_storage: &dyn crate::storage::ItemStorage<i64, crate::db::sparks::SparkDataDbResult>,
    // 	race_storage: &dyn crate::storage::ItemStorage<i64, crate::db::race::RaceDataDbResult>,
    // 	wins_storage: &dyn crate::storage::ItemStorage<i64, crate::db::major_wins::MajorWinsDbDataResult>,
    // 	borrows: bool,
    // ) -> Self {
    // 	let trainee = trainee_storage
    // 		.get(mssgpack.card_id)
    // 		.expect(&format!("Cannot find trainee with id {}", mssgpack.card_id));

    // 	let sparks = Spark::multiple_from_db(&mssgpack.factor_id_array, spark_storage);
    // 	let aptitudes = Aptitudes::from_mssgpack(mssgpack);

    // 	let stats = UmaStats {
    // 		spe: mssgpack.speed,
    // 		sta: mssgpack.stamina,
    // 		pow: mssgpack.power,
    // 		gut: mssgpack.guts,
    // 		wit: mssgpack.wiz,
    // 	};

    // 	let mut hasher = std::collections::hash_map::DefaultHasher::new();
    // 	mssgpack.card_id.hash(&mut hasher);
    // 	mssgpack.factor_id_array.hash(&mut hasher);
    // 	mssgpack.win_saddle_id_array.hash(&mut hasher);

    // 	let hash = hasher.finish();

    // 	let succession_umas = mssgpack
    // 		.succession_chara_array
    // 		.iter()
    // 		.map(|succession_chara| {
    // 			SuccessionUma::from_mssgpack_and_db(
    // 				succession_chara,
    // 				trainee_storage,
    // 				spark_storage,
    // 				wins_storage,
    // 			)
    // 		})
    // 		.collect::<Vec<_>>();

    // 	let mut parent1_wins = Vec::new();
    // 	let mut parent2_wins = Vec::new();

    // 	for succession_uma in succession_umas.iter() {
    // 		if succession_uma.position == SuccessionUmaPosition::Parent1 {
    // 			parent1_wins = succession_uma.win_saddle_ids.clone();
    // 		} else if succession_uma.position == SuccessionUmaPosition::Parent2 {
    // 			parent2_wins = succession_uma.win_saddle_ids.clone();
    // 		}
    // 	}

    // 	let mut major_wins =
    // 		UmaMajorWins::from_saddle_ids(&mssgpack.win_saddle_id_array, wins_storage);

    // 	for parent1_win in parent1_wins.iter() {
    // 		if let Some(win) = major_wins.get_mut(parent1_win) {
    // 			win.shared_count += 1;
    // 		}
    // 	}
    // 	for parent2_win in parent2_wins.iter() {
    // 		if let Some(win) = major_wins.get_mut(parent2_win) {
    // 			win.shared_count += 1;
    // 		}
    // 	}

    // 	let mut race_results = mssgpack
    // 		.race_result_list
    // 		.iter()
    // 		.filter_map(|race_result| {
    // 			let Some(race) = race_storage
    // 				.get(race_result.program_id)
    // 				.map(|race_data| RaceResult {
    // 					program_id: race_result.program_id,
    // 					race_name: race_data.race_name.clone(),
    // 					race_grade: RaceRank::from_raw(race_data.race_grade),
    // 					distance: race_data.distance,
    // 					ground: Ground::from_raw(race_data.ground),
    // 					turn: race_result.turn as i32,
    // 					weather: WeatherType::from_raw(race_result.weather),
    // 					ground_condition: GroundCondition::from_raw(race_result.ground_condition),
    // 					running_style: RunningStyle::from_raw(race_result.running_style),
    // 					popularity: race_result.popularity as i32,
    // 					result_rank: race_result.result_rank as i32,
    // 					result_time: race_result.result_time as i32,
    // 					prize_money: race_result.prize_money as i32,
    // 					race_instance_id: race_data.race_instance_id,
    // 					shared: false,
    // 				})
    // 			else {
    // 				return None;
    // 			};

    // 			Some((race.race_instance_id, race))
    // 		})
    // 		.collect::<BTreeMap<i64, RaceResult>>();

    // 	for major_win in major_wins.values() {
    // 		if major_win.shared_count > 0 {
    // 			major_win.race_instance_ids.iter().for_each(|race_id| {
    // 				let race = race_results.get_mut(race_id).unwrap();
    // 				race.shared = true;
    // 			});
    // 		}
    // 	}

    // 	let favourite = favourites_by_trained_chara_id.get(&mssgpack.trained_chara_id);

    // 	let mut race_results: Vec<RaceResult> = race_results.into_values().collect();
    // 	race_results.sort_by(|a, b| a.turn.cmp(&b.turn));

    // 	let major_wins: Vec<UmaMajorWins> = major_wins.into_values().collect();

    // 	Self {
    // 		trainee_id: mssgpack.card_id,
    // 		scenario: mssgpack.scenario_id as u16,
    // 		trained_chara_id: mssgpack.trained_chara_id,
    // 		favorite_icon_type: favourite
    // 			.and_then(|f| f.icon_type.and_then(|icon| FavouriteIcon::try_from(icon).ok())),
    // 		favorite_memo: favourite.map(|f| f.memo.clone()),
    // 		created_at: parse_datetime(&mssgpack.register_time).unwrap(),
    // 		hash,
    // 		character_id: trainee.character_id,
    // 		rank: mssgpack.rank_score,
    // 		trainee_name: trainee.name.clone(),
    // 		stats,
    // 		aptitudes,
    // 		sparks,
    // 		succession_umas,
    // 		race_results,
    // 		win_saddle_ids: mssgpack.win_saddle_id_array.clone(),
    // 		major_wins,
    // 		owner_id: if borrows {
    // 			mssgpack.viewer_id as u64
    // 		} else {
    // 			mssgpack.owner_viewer_id as u64
    // 		},
    // 	}
    // }

    pub fn is_borrowed(&self) -> bool {
        self.owner_id != 0
    }

    pub fn white_spark_count(&self) -> usize {
        let groups = self.spark_groups(false);
        groups
            .values()
            .filter(|group| group.spark_type.is_white())
            .count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionUma {
    pub trainee_id: i64,
    pub position: SuccessionUmaPosition,
    pub hash: u64,
    pub character_id: i64,
    pub trainee_name: String,
    pub sparks: Vec<Spark>,
    pub win_saddle_ids: Vec<i64>,
    pub major_wins: Vec<UmaMajorWins>,
    pub owner_id: u64,
}

impl PartialEq for SuccessionUma {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl SuccessionUma {
    // pub fn from_mssgpack_and_db(
    // 	mssgpack: &MssgPackSuccessionChara,
    // 	trainee_storage: &dyn crate::storage::ItemStorage<i64, crate::db::trainees::TraineeDataDbResult>,
    // 	spark_storage: &dyn crate::storage::ItemStorage<i64, crate::db::sparks::SparkDataDbResult>,
    // 	wins_storage: &dyn crate::storage::ItemStorage<i64, crate::db::major_wins::MajorWinsDbDataResult>,
    // ) -> Self {
    // 	let trainee = trainee_storage
    // 		.get(mssgpack.card_id)
    // 		.expect(&format!("Cannot find trainee with id {}", mssgpack.card_id));

    // 	let mut hasher = std::collections::hash_map::DefaultHasher::new();
    // 	mssgpack.card_id.hash(&mut hasher);
    // 	mssgpack.factor_id_array.hash(&mut hasher);
    // 	mssgpack.win_saddle_id_array.hash(&mut hasher);

    // 	let position = SuccessionUmaPosition::from_raw(mssgpack.position_id);
    // 	let hash = hasher.finish();
    // 	let sparks = Spark::multiple_from_db(&mssgpack.factor_id_array, spark_storage);
    // 	let major_wins = UmaMajorWins::from_saddle_ids(&mssgpack.win_saddle_id_array, wins_storage)
    // 		.into_values()
    // 		.collect::<Vec<_>>();

    // 	Self {
    // 		trainee_id: mssgpack.card_id,
    // 		position,
    // 		hash,
    // 		character_id: trainee.character_id,
    // 		trainee_name: trainee.name.clone(),
    // 		sparks,
    // 		win_saddle_ids: mssgpack.win_saddle_id_array.clone(),
    // 		major_wins,
    // 		owner_id: mssgpack.owner_viewer_id as u64,
    // 	}
    // }

    pub fn is_borrowed(&self) -> bool {
        self.owner_id != 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct UmaStats {
    pub spe: i32,
    pub sta: i32,
    pub pow: i32,
    pub gut: i32,
    pub wit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aptitudes {
    pub turf: AptitudeLevel,
    pub dirt: AptitudeLevel,
    pub sprint: AptitudeLevel,
    pub mile: AptitudeLevel,
    pub medium: AptitudeLevel,
    pub long: AptitudeLevel,
    pub front: AptitudeLevel,
    pub pace_chaser: AptitudeLevel,
    pub late_surger: AptitudeLevel,
    pub end_closer: AptitudeLevel,
}

impl Aptitudes {
    // pub fn from_mssgpack(mssgpack: &MssgPackTrainedChara) -> Self {
    // 	Self {
    // 		turf: AptitudeLevel::from_raw(mssgpack.proper_ground_turf),
    // 		dirt: AptitudeLevel::from_raw(mssgpack.proper_ground_dirt),
    // 		front: AptitudeLevel::from_raw(mssgpack.proper_running_style_nige),
    // 		pace_chaser: AptitudeLevel::from_raw(mssgpack.proper_running_style_senko),
    // 		late_surger: AptitudeLevel::from_raw(mssgpack.proper_running_style_sashi),
    // 		end_closer: AptitudeLevel::from_raw(mssgpack.proper_running_style_oikomi),
    // 		sprint: AptitudeLevel::from_raw(mssgpack.proper_distance_short),
    // 		mile: AptitudeLevel::from_raw(mssgpack.proper_distance_mile),
    // 		medium: AptitudeLevel::from_raw(mssgpack.proper_distance_middle),
    // 		long: AptitudeLevel::from_raw(mssgpack.proper_distance_long),
    // 	}
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Spark {
    pub spark_id: i64,
    pub spark_group_id: i64,
    pub name: String,
    pub description: String,
    pub stars_count: i8,
    pub spark_type: SparkType,
}

impl Spark {
    // pub fn multiple_from_db(
    // 	spark_ids: &[i64],
    // 	spark_storage: &dyn ItemStorage<i64, SparkDataDbResult>,
    // ) -> Vec<Self> {
    // 	spark_ids
    // 		.iter()
    // 		.map(|spark_id| {
    // 			let spark = spark_storage
    // 				.get(*spark_id)
    // 				.expect(&format!("Cannot find spark with id {}", spark_id));
    // 			Spark {
    // 				spark_id: *spark_id,
    // 				spark_group_id: spark.group_id,
    // 				name: spark.name.clone(),
    // 				description: spark.description.clone(),
    // 				stars_count: spark.stars_count,
    // 				spark_type: SparkType::from_raw(spark.spark_type),
    // 			}
    // 		})
    // 		.collect::<Vec<_>>()
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RaceResult {
    pub program_id: i64,
    pub race_name: String,
    pub race_grade: RaceGrade,
    pub distance: i32,
    pub ground: Ground,
    pub turn: i32,
    pub weather: WeatherType,
    pub ground_condition: GroundCondition,
    pub running_style: RunningStyle,
    pub popularity: i32,
    pub result_rank: i32,
    pub result_time: i32,
    pub prize_money: i32,
    pub race_instance_id: i64,
    pub shared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceInfo {
    pub program_id: i64,
    pub race_id: i64,
    pub race_instance_id: i64,
    pub name: String,
    pub rank: RaceGrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UmaMajorWins {
    pub id: i64,
    pub name: String,
    pub group_id: i64,
    pub race_instance_ids: Vec<i64>,
    pub shared_count: i8,
}

impl UmaMajorWins {
    // pub fn from_saddle_ids(
    // 	ids: &[i64],
    // 	wins_storage: &dyn ItemStorage<i64, MajorWinsDbDataResult>,
    // ) -> BTreeMap<i64, Self> {
    // 	let mut wins = BTreeMap::new();

    // 	for id in ids {
    // 		let win = wins_storage
    // 			.get(*id)
    // 			.expect(&format!("Cannot find major wins with id {}", id));
    // 		let race_instance_ids = win.race_instance_ids.clone();
    // 		wins.insert(
    // 			win.group_id,
    // 			Self {
    // 				id: *id,
    // 				name: win.name.clone(),
    // 				group_id: win.group_id,
    // 				race_instance_ids,
    // 				shared_count: 0,
    // 			},
    // 		);
    // 	}

    // 	wins
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateTime {
    pub unix_seconds: i64,
}

impl DateTime {
    pub fn to_seconds_from_unix_epoch(&self) -> i64 {
        self.unix_seconds
    }
}

pub fn parse_datetime(value: &str) -> Result<DateTime, String> {
    value
        .parse::<i64>()
        .map(|seconds| DateTime {
            unix_seconds: seconds,
        })
        .map_err(|e| format!("Invalid datetime format: {e}"))
}
