use std::{
    collections::HashMap,
    hash::{Hash as _, Hasher as _},
    str::FromStr,
};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::{
    db_models::UmaHash,
    models::SuccessionUmaPosition,
    mssgpack_data::{
        MssgPackRaceResult, MssgPackSkill, MssgPackSuccessionChara, MssgPackSupportCard,
        MssgPackTrainedChara,
    },
};

#[derive(Debug, Clone, Default)]
pub struct Veteran {
    pub hash: UmaHash,
    pub trainee_id: i64,
    pub scenario: Option<u16>,
    pub trained_chara_id: Option<i64>,
    pub favorite_icon_type: Option<u16>,
    pub favorite_memo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub rank: u16,
    pub rank_score: u32,
    pub stat_speed: Option<u16>,
    pub stat_stamina: Option<u16>,
    pub stat_power: Option<u16>,
    pub stat_guts: Option<u16>,
    pub stat_wit: Option<u16>,
    pub aptitude_turf: Option<u16>,
    pub aptitude_dirt: Option<u16>,
    pub aptitude_sprint: Option<u16>,
    pub aptitude_mile: Option<u16>,
    pub aptitude_medium: Option<u16>,
    pub aptitude_long: Option<u16>,
    pub aptitude_front: Option<u16>,
    pub aptitude_pace_chaser: Option<u16>,
    pub aptitude_late_surger: Option<u16>,
    pub aptitude_end_closer: Option<u16>,
    pub parent_a: Option<UmaHash>,
    pub parent_b: Option<UmaHash>,
    pub owner_id: Option<u64>,
    pub is_race_data: bool,
    pub is_browser: bool,
    pub min_hash: Option<UmaHash>,
    pub owned: bool,
    pub talent_level: Option<u8>,
    pub rarity: u8,
    pub container_sparks: Vec<i64>,
    pub container_major_wins: Vec<i64>,
    pub updated_at: Option<DateTime<Utc>>,
    pub use_type: i64,
    pub fans: i64,
    pub succession_num: i64,
    pub is_saved: i64,
    pub is_locked: i64,
    pub chara_grade: i64,
    pub veteran_running_style: i64,
    pub nickname_id: i64,
    pub wins: i64,
}

#[derive(Debug, Clone, Default)]
pub struct Parent {
    pub hash: UmaHash,
    pub trainee_id: i64,
    pub rank: u16,
    pub rarity: u8,
    pub talent_level: Option<u8>,
    pub trained_chara_id: i64,
    pub parent_a: Option<UmaHash>,
    pub parent_b: Option<UmaHash>,
    pub owner_id: Option<u64>,
    pub owned: bool,
    pub container_sparks: Vec<i64>,
    pub container_major_wins: Vec<i64>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Parent {
    fn from_mssgpack_with_position(
        parent: &MssgPackSuccessionChara,
        veteran_viewer_id: i64,
    ) -> (SuccessionUmaPosition, Self) {
        let position = SuccessionUmaPosition::from_raw(parent.position_id);
        let hash = hash_succession_chara(parent);
        let owner_viewer_id = if parent.owner_viewer_id == 0 {
            veteran_viewer_id
        } else {
            parent.owner_viewer_id
        };
        let parent = Self {
            hash,
            trainee_id: parent.card_id,
            rank: parent.rank as u16,
            rarity: parent.rarity as u8,
            talent_level: Some(parent.talent_level as u8),
            trained_chara_id: parent.card_id,
            parent_a: None,
            parent_b: None,
            owner_id: Some(owner_viewer_id as u64),
            owned: parent.owner_viewer_id == 0,
            container_sparks: parent.factor_id_array.clone(),
            container_major_wins: parent.win_saddle_id_array.clone(),
            updated_at: None,
        };
        (position, parent)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UmaGroup {
    pub veteran: Veteran,
    pub parent_a: Parent,
    pub parent_b: Parent,
    pub grandparent_aa: Option<Parent>,
    pub grandparent_ab: Option<Parent>,
    pub grandparent_ba: Option<Parent>,
    pub grandparent_bb: Option<Parent>,
    pub support_card_list: Option<Vec<MssgPackSupportCard>>,
    pub total_skills: Option<usize>,
    pub skills: Vec<MssgPackSkill>,
    pub fans: Option<i64>,
    pub sparks_sum: HashMap<i64, u16>,
    pub sparks_count: HashMap<i64, u8>,
    pub wins_count: HashMap<i64, u8>,
    pub race_result_list: Vec<MssgPackRaceResult>,
    pub nickname_id_array: Vec<i64>,
}

impl UmaGroup {
    pub fn from_trained_chara_mssgpack(
        trained_chara: &MssgPackTrainedChara,
    ) -> Result<Self, String> {
        let mut parent_a = None;
        let mut parent_b = None;
        let mut grandparent_aa = None;
        let mut grandparent_ab = None;
        let mut grandparent_ba = None;
        let mut grandparent_bb = None;

        for parent in trained_chara.succession_chara_array.iter() {
            let (parent_position, parent) =
                Parent::from_mssgpack_with_position(parent, trained_chara.viewer_id);
            match parent_position {
                SuccessionUmaPosition::Parent1 => parent_a = Some(parent),
                SuccessionUmaPosition::Parent2 => parent_b = Some(parent),
                SuccessionUmaPosition::Grandparent1Parent1 => grandparent_aa = Some(parent),
                SuccessionUmaPosition::Grandparent1Parent2 => grandparent_ab = Some(parent),
                SuccessionUmaPosition::Grandparent2Parent1 => grandparent_ba = Some(parent),
                SuccessionUmaPosition::Grandparent2Parent2 => grandparent_bb = Some(parent),
            }
        }

        let mut parent_a =
            parent_a.ok_or_else(|| missing_succession_parent_error(trained_chara, "Parent1"))?;
        let mut parent_b =
            parent_b.ok_or_else(|| missing_succession_parent_error(trained_chara, "Parent2"))?;

        let mut veteran = Veteran::from_trained_chara_mssgpack(trained_chara);
        let pa_hash = parent_a.hash;
        let pb_hash = parent_b.hash;
        let gpaa_hash = grandparent_aa.as_ref().map(|gp| gp.hash);
        let gpab_hash = grandparent_ab.as_ref().map(|gp| gp.hash);
        let gpba_hash = grandparent_ba.as_ref().map(|gp| gp.hash);
        let gpbb_hash = grandparent_bb.as_ref().map(|gp| gp.hash);

        veteran.parent_a = Some(pa_hash);
        veteran.parent_b = Some(pb_hash);

        parent_a.parent_a = gpaa_hash;
        parent_a.parent_b = gpab_hash;
        parent_b.parent_a = gpba_hash;
        parent_b.parent_b = gpbb_hash;

        let mut sparks_sum = HashMap::new();
        let mut sparks_count = HashMap::new();
        let mut wins_count = HashMap::new();

        Self::gather_sums(
            &mut sparks_sum,
            &mut sparks_count,
            &mut wins_count,
            &veteran.container_sparks,
            &veteran.container_major_wins,
        );

        let family_tree = [&parent_a, &parent_b];

        for member in family_tree.iter() {
            Self::gather_sums(
                &mut sparks_sum,
                &mut sparks_count,
                &mut wins_count,
                &member.container_sparks,
                &member.container_major_wins,
            );
        }

        Ok(Self {
            veteran,
            parent_a,
            parent_b,
            grandparent_aa,
            grandparent_ab,
            grandparent_ba,
            grandparent_bb,
            support_card_list: Some(trained_chara.support_card_list.clone()),
            total_skills: Some(trained_chara.skill_array.len()),
            skills: trained_chara.skill_array.clone(),
            fans: Some(trained_chara.fans),
            sparks_sum,
            sparks_count,
            wins_count,
            race_result_list: trained_chara.race_result_list.clone(),
            nickname_id_array: trained_chara.nickname_id_array.clone(),
        })
    }

    pub fn gather_sums(
        sparks_sum: &mut HashMap<i64, u16>,
        sparks_count: &mut HashMap<i64, u8>,
        wins_count: &mut HashMap<i64, u8>,
        spark_ids: &[i64],
        win_ids: &[i64],
    ) {
        for spark in spark_ids.iter() {
            let (base_id, spark_level) = (spark / 100, spark % 100);

            // Accumulate the level sum
            *sparks_sum.entry(base_id).or_insert(0u16) += spark_level as u16;

            // Track how many family members have this spark type
            *sparks_count.entry(base_id).or_insert(0u8) += 1;
        }
        for win in win_ids.iter() {
            // Track how many family members have this win type
            *wins_count.entry(*win).or_insert(0u8) += 1;
        }
    }
}

fn missing_succession_parent_error(
    trained_chara: &MssgPackTrainedChara,
    missing_position: &str,
) -> String {
    format!(
        "trained_chara_id {} is missing succession entry for {}",
        trained_chara.trained_chara_id, missing_position
    )
}

impl Veteran {
    pub fn from_trained_chara_mssgpack(trained_chara: &MssgPackTrainedChara) -> Self {
        let hash_main = hash_trained_chara(trained_chara);
        let hash_min = hash_uma_entity(trained_chara.card_id, &trained_chara.factor_id_array);
        let created_at = parse_worker_datetime(&trained_chara.create_time)
            .or_else(|| parse_worker_datetime(&trained_chara.register_time))
            .unwrap_or_else(Utc::now);

        Self {
            hash: hash_main,
            min_hash: Some(hash_min),
            trainee_id: trained_chara.card_id,
            scenario: Some(trained_chara.scenario_id as u16),
            trained_chara_id: Some(trained_chara.trained_chara_id),
            favorite_icon_type: None,
            favorite_memo: None,
            created_at,
            rank: trained_chara.rank as u16,
            rank_score: trained_chara.rank_score as u32,
            stat_speed: Some(trained_chara.speed as u16),
            stat_stamina: Some(trained_chara.stamina as u16),
            stat_power: Some(trained_chara.power as u16),
            stat_guts: Some(trained_chara.guts as u16),
            stat_wit: Some(trained_chara.wiz as u16),
            aptitude_turf: Some(trained_chara.proper_ground_turf as u16),
            aptitude_dirt: Some(trained_chara.proper_ground_dirt as u16),
            aptitude_sprint: Some(trained_chara.proper_distance_short as u16),
            aptitude_mile: Some(trained_chara.proper_distance_mile as u16),
            aptitude_medium: Some(trained_chara.proper_distance_middle as u16),
            aptitude_long: Some(trained_chara.proper_distance_long as u16),
            aptitude_front: Some(trained_chara.proper_running_style_nige as u16),
            aptitude_pace_chaser: Some(trained_chara.proper_running_style_senko as u16),
            aptitude_late_surger: Some(trained_chara.proper_running_style_sashi as u16),
            aptitude_end_closer: Some(trained_chara.proper_running_style_oikomi as u16),
            parent_a: None,
            parent_b: None,
            owner_id: if trained_chara.owner_viewer_id == 0 {
                Some(trained_chara.viewer_id as u64)
            } else {
                Some(trained_chara.owner_viewer_id as u64)
            },
            owned: trained_chara.owner_viewer_id == 0,
            talent_level: Some(trained_chara.talent_level as u8),
            rarity: trained_chara.rarity as u8,
            container_sparks: trained_chara.factor_id_array.clone(),
            container_major_wins: trained_chara.win_saddle_id_array.clone(),
            is_race_data: false,
            is_browser: false,
            updated_at: None,
            use_type: trained_chara.use_type,
            fans: trained_chara.fans,
            succession_num: trained_chara.succession_num,
            is_saved: trained_chara.is_saved,
            is_locked: trained_chara.is_locked,
            chara_grade: trained_chara.chara_grade,
            veteran_running_style: trained_chara.running_style,
            nickname_id: trained_chara.nickname_id,
            wins: trained_chara.wins,
        }
    }
}

fn parse_worker_datetime(value: &str) -> Option<DateTime<Utc>> {
    if value.trim().is_empty() {
        return None;
    }

    DateTime::from_str(value)
        .ok()
        .map(|dt: DateTime<Utc>| dt)
        .or_else(|| {
            DateTime::parse_from_rfc3339(value)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                .ok()
                .map(|dt| Utc.from_utc_datetime(&dt))
        })
}

pub fn hash_uma_entity(trainee_id: i64, factor_id_array: &[i64]) -> UmaHash {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    trainee_id.hash(&mut hasher);

    let mut sorted_factors = factor_id_array.to_vec();
    sorted_factors.sort();
    sorted_factors.hash(&mut hasher);

    let hash = hasher.finish();
    UmaHash::from(hash)
}

fn hash_trained_chara(mssgpack: &MssgPackTrainedChara) -> UmaHash {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    mssgpack.card_id.hash(&mut hasher);

    let mut sorted_factors = mssgpack.factor_id_array.clone();
    sorted_factors.sort();
    sorted_factors.hash(&mut hasher);

    mssgpack.create_time.hash(&mut hasher);
    mssgpack.rank.hash(&mut hasher);
    mssgpack.rarity.hash(&mut hasher);
    mssgpack.talent_level.hash(&mut hasher);
    mssgpack.rank_score.hash(&mut hasher);
    mssgpack.speed.hash(&mut hasher);
    mssgpack.stamina.hash(&mut hasher);
    mssgpack.power.hash(&mut hasher);
    mssgpack.guts.hash(&mut hasher);
    mssgpack.wiz.hash(&mut hasher);

    let hash = hasher.finish();
    UmaHash::from(hash)
}

fn hash_succession_chara(mssgpack: &MssgPackSuccessionChara) -> UmaHash {
    hash_uma_entity(mssgpack.card_id, &mssgpack.factor_id_array)
}
