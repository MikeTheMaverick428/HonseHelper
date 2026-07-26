use crate::config::{SCENE_MANAGER_CLASS, SCENE_MANAGER_NAMESPACE};
use crate::current_view::KnownView;
use crate::models::user_data::UserData;
use crate::models::{
    card_data::WorkCardDataModel, piece_data::WorkPieceDataModel, race_team,
    support_card_data::SupportCardDataModel, trained_chara_container::TrainedCharaContainerModel,
    trophy_data::WorkTrophyDataModel, work_friend_data::WorkFriendDataModel,
};
use anyhow::{anyhow, Result};
use il2cpp_runtime::{
    Il2CppMetadata, ProcessInfo, ProcessMemory, RuntimeIntrospector, RuntimeModelSpec,
    RuntimeValue, SingletonCandidate, SingletonResolver,
};
use rmpv::Value;

#[derive(Default)]
pub struct WorkerState {
    pub process: Option<ProcessInfo>,
    inspector: Option<RuntimeIntrospector>,
    scene_manager_ptr: Option<u64>,
    next_view_id_offset: Option<u64>,
    current_scene_base_offset: Option<u64>,
    current_view_base_offset: Option<u64>,
    current_view_controller_offset: Option<u64>,
    work_data_manager_ptr: Option<u64>,
    trained_chara_data_offset: Option<u64>,
    friend_data_offset: Option<u64>,
    support_card_data_offset: Option<u64>,
    piece_data_offset: Option<u64>,
    trophy_data_offset: Option<u64>,
    card_data_offset: Option<u64>,
    last_known_view: Option<KnownView>,
    user_data: Option<UserData>,
    temp_data_ptr: Option<u64>,
}

impl WorkerState {
    pub fn ensure_process(&mut self) -> Result<()> {
        if self.inspector.is_some() {
            return Ok(());
        }

        let info = shared::process::find_game_process()?;
        let memory = ProcessMemory::new(info.pid)?;

        self.process = Some(info);
        self.inspector = Some(RuntimeIntrospector::new(memory));
        self.scene_manager_ptr = None;
        self.next_view_id_offset = None;
        self.current_scene_base_offset = None;
        self.current_view_base_offset = None;
        self.current_view_controller_offset = None;
        self.work_data_manager_ptr = None;
        self.trained_chara_data_offset = None;
        self.friend_data_offset = None;
        self.support_card_data_offset = None;
        self.trophy_data_offset = None;
        self.card_data_offset = None;
        self.user_data = None;
        self.temp_data_ptr = None;
        Ok(())
    }

    pub fn discover_scene_manager(&mut self, max_scan_bytes: usize) -> Result<u64> {
        if let Some(ptr) = self.scene_manager_ptr {
            return Ok(ptr);
        }

        if let Some(info) = self.process.as_ref() {
            if let Ok((scene_manager_ptr, work_data_manager_ptr, temp_data_ptr)) =
                discover_singletons_via_metadata(info.pid)
            {
                if temp_data_ptr.is_some() {
                    self.temp_data_ptr = temp_data_ptr;
                }
                if let Some(ptr) = scene_manager_ptr {
                    self.scene_manager_ptr = Some(ptr);
                    if let Some(work_ptr) = work_data_manager_ptr {
                        self.work_data_manager_ptr = Some(work_ptr);
                    }
                    return Ok(ptr);
                }
            }
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let candidate_ptr = inspector
            .find_first_live_object_by_class(
                SCENE_MANAGER_NAMESPACE,
                SCENE_MANAGER_CLASS,
                max_scan_bytes,
            )?
            .ok_or_else(|| {
                anyhow!(
                    "Could not discover live {}::{} instance",
                    SCENE_MANAGER_NAMESPACE,
                    SCENE_MANAGER_CLASS
                )
            })?;

        let class_ptr = inspector.process_memory().read_pointer(candidate_ptr)?;
        let instance_ptr = SingletonResolver::resolve_mono_singleton_instance(
            inspector.process_memory(),
            class_ptr,
            |_mem, ptr| {
                if ptr >= 0x10000 {
                    Some(ptr)
                } else {
                    None
                }
            },
        )
        .or_else(|_| {
            // Some builds may still expose it as plain Singleton<T>.
            SingletonResolver::resolve_singleton_instance(
                inspector.process_memory(),
                class_ptr,
                |_mem, ptr| {
                    if ptr >= 0x10000 {
                        Some(ptr)
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap_or(candidate_ptr);

        self.scene_manager_ptr = Some(instance_ptr);
        Ok(instance_ptr)
    }

    pub fn ensure_scene_offsets(&mut self, scene_manager_ptr: u64) -> Result<()> {
        if self.next_view_id_offset.is_some()
            && self.current_scene_base_offset.is_some()
            && self.current_view_base_offset.is_some()
        {
            return Ok(());
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let next_view_id_offset = inspector.resolve_runtime_offset_for_object(
            scene_manager_ptr,
            &["_nextViewId", "nextViewId", "NextViewId", "_nextViewID"],
        )?;
        let current_scene_base_offset = inspector.resolve_runtime_offset_for_object(
            scene_manager_ptr,
            &["_currentSceneBase", "currentSceneBase", "CurrentSceneBase"],
        )?;
        let current_view_base_offset = inspector
            .resolve_runtime_offset_for_object(
                scene_manager_ptr,
                &["_currentViewBase", "currentViewBase", "CurrentViewBase"],
            )
            .ok();

        let current_view_controller_offset = inspector
            .resolve_runtime_offset_for_object(
                scene_manager_ptr,
                &[
                    "_currentViewController",
                    "currentViewController",
                    "CurrentViewController",
                ],
            )
            .ok();

        self.next_view_id_offset = Some(next_view_id_offset);
        self.current_scene_base_offset = Some(current_scene_base_offset);
        self.current_view_base_offset = current_view_base_offset;
        self.current_view_controller_offset = current_view_controller_offset;
        Ok(())
    }

    pub fn read_view_state(&mut self) -> Result<Value> {
        let scene_manager_ptr = self
            .scene_manager_ptr
            .ok_or_else(|| anyhow!("SceneManager is not resolved"))?;
        let next_view_id_offset = self
            .next_view_id_offset
            .ok_or_else(|| anyhow!("_nextViewId offset is not resolved"))?;
        let current_scene_base_offset = self
            .current_scene_base_offset
            .ok_or_else(|| anyhow!("_currentSceneBase offset is not resolved"))?;
        let current_view_base_offset = self.current_view_base_offset;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let next_view_id = inspector
            .process_memory()
            .read_i32(scene_manager_ptr + next_view_id_offset)?;

        let current_scene_base_ptr = inspector
            .process_memory()
            .read_pointer(scene_manager_ptr + current_scene_base_offset)?;

        let current_scene_class = if current_scene_base_ptr == 0 {
            String::new()
        } else {
            inspector
                .class_name_for_object(current_scene_base_ptr)
                .unwrap_or_default()
        };

        let current_view_base_ptr = match current_view_base_offset {
            Some(offset) => inspector
                .process_memory()
                .read_pointer(scene_manager_ptr + offset)
                .unwrap_or(0),
            None => 0,
        };

        let current_view_class = if current_view_base_ptr == 0 {
            String::new()
        } else {
            inspector
                .class_name_for_object(current_view_base_ptr)
                .unwrap_or_default()
        };

        let kclass = if !current_view_class.is_empty() {
            current_view_class.clone()
        } else {
            current_scene_class.clone()
        };

        let previous_known_view = self.last_known_view;
        let current_known_view = KnownView::from_raw(next_view_id, &kclass);
        let changed = match (previous_known_view, current_known_view) {
            (Some(prev), Some(curr)) => prev != curr,
            _ => false,
        };

        if let Some(view) = current_known_view {
            self.last_known_view = Some(view);
        }

        let known_view_literal = self.last_known_view.map(KnownView::as_str);

        Ok(Value::Map(vec![
            (
                Value::from("scene_manager_ptr"),
                Value::from(format!("{:#x}", scene_manager_ptr)),
            ),
            (Value::from("next_view_id"), Value::from(next_view_id)),
            (Value::from("kclass"), Value::from(kclass)),
            (
                Value::from("current_scene_base_ptr"),
                Value::from(format!("{:#x}", current_scene_base_ptr)),
            ),
            (
                Value::from("current_scene_class"),
                Value::from(current_scene_class),
            ),
            (
                Value::from("current_view_base_ptr"),
                Value::from(format!("{:#x}", current_view_base_ptr)),
            ),
            (
                Value::from("current_view_class"),
                Value::from(current_view_class),
            ),
            (Value::from("changed"), Value::from(changed)),
            (
                Value::from("last_known_view"),
                known_view_literal.map(Value::from).unwrap_or(Value::Nil),
            ),
        ]))
    }

    pub fn extract_veteran_data(&mut self) -> Result<Value> {
        let work_data_manager_ptr = self.discover_work_data_manager_instance()?;
        let trained_chara_data_ptr = self.read_trained_chara_data_ptr(work_data_manager_ptr)?;
        if trained_chara_data_ptr == 0 {
            return Err(anyhow!("WorkDataManager.TrainedCharaData is null"));
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let value =
            TrainedCharaContainerModel::read_model_value(inspector, trained_chara_data_ptr)?;
        Ok(pass_through_value(value))
    }

    pub fn extract_friend_data(&mut self) -> Result<Value> {
        let work_data_manager_ptr = self.discover_work_data_manager_instance()?;
        let friend_data_ptr = self.read_friend_data_ptr(work_data_manager_ptr)?;
        if friend_data_ptr == 0 {
            return Err(anyhow!("WorkDataManager.FriendData is null"));
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let value = WorkFriendDataModel::read_model_value(inspector, friend_data_ptr)?;
        Ok(pass_through_value(value))
    }

    pub fn extract_support_card_data(&mut self) -> Result<Value> {
        let work_data_manager_ptr = self.discover_work_data_manager_instance()?;
        let support_card_data_ptr = self.read_support_card_data_ptr(work_data_manager_ptr)?;
        if support_card_data_ptr == 0 {
            return Err(anyhow!("WorkDataManager.SupportCardData is null"));
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let value = SupportCardDataModel::read_model_value(inspector, support_card_data_ptr)?;
        Ok(pass_through_value(value))
    }

    fn discover_temp_data(&mut self) -> Option<u64> {
        if let Some(ptr) = self.temp_data_ptr {
            return Some(ptr);
        }

        let inspector = self.inspector.as_mut()?;
        let candidate_ptr = inspector
            .find_first_live_object_by_class("Gallop", "TempData", 256 * 1024 * 1024)
            .ok()?;
        self.temp_data_ptr = candidate_ptr;
        candidate_ptr
    }

    fn discover_work_data_manager_instance(&mut self) -> Result<u64> {
        if let Some(ptr) = self.work_data_manager_ptr {
            return Ok(ptr);
        }

        if let Some(info) = self.process.as_ref() {
            if let Ok((scene_manager_ptr, work_data_manager_ptr, temp_data_ptr)) =
                discover_singletons_via_metadata(info.pid)
            {
                if temp_data_ptr.is_some() {
                    self.temp_data_ptr = temp_data_ptr;
                }
                if let Some(scene_ptr) = scene_manager_ptr {
                    self.scene_manager_ptr = Some(scene_ptr);
                }
                if let Some(ptr) = work_data_manager_ptr {
                    self.work_data_manager_ptr = Some(ptr);
                    return Ok(ptr);
                }
            }
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let candidate_ptr = inspector
            .find_first_live_object_by_class("Gallop", "WorkDataManager", 512 * 1024 * 1024)?
            .ok_or_else(|| anyhow!("Could not discover live Gallop::WorkDataManager instance"))?;

        let class_ptr = inspector.process_memory().read_pointer(candidate_ptr)?;
        let instance_ptr = SingletonResolver::resolve_singleton_instance(
            inspector.process_memory(),
            class_ptr,
            |_mem, ptr| {
                if ptr >= 0x10000 {
                    Some(ptr)
                } else {
                    None
                }
            },
        )
        .unwrap_or(candidate_ptr);

        self.work_data_manager_ptr = Some(instance_ptr);
        Ok(instance_ptr)
    }

    fn read_trained_chara_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.trained_chara_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &[
                    "<TrainedCharaData>k__BackingField",
                    "_trainedCharaData",
                    "trainedCharaData",
                ],
            )?;
            self.trained_chara_data_offset = Some(offset);
        }

        let offset = self
            .trained_chara_data_offset
            .ok_or_else(|| anyhow!("TrainedCharaData offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }

    fn read_friend_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.friend_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &[
                    "<FriendData>k__BackingField",
                    "_friendData",
                    "friendData",
                    "FriendData",
                ],
            )?;
            self.friend_data_offset = Some(offset);
        }

        let offset = self
            .friend_data_offset
            .ok_or_else(|| anyhow!("FriendData offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }

    pub fn extract_trophy_data(&mut self) -> Result<Value> {
        let work_data_manager_ptr = self.discover_work_data_manager_instance()?;
        let trophy_data_ptr = self.read_trophy_data_ptr(work_data_manager_ptr)?;
        if trophy_data_ptr == 0 {
            return Err(anyhow!("WorkDataManager.Trophy is null"));
        }

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let value = WorkTrophyDataModel::read_model_value(inspector, trophy_data_ptr)?;
        Ok(pass_through_value(value))
    }

    pub fn extract_card_data(&mut self) -> Result<Value> {
        let work_data_manager_ptr = self.discover_work_data_manager_instance()?;
        let card_data_ptr = self.read_card_data_ptr(work_data_manager_ptr)?;
        if card_data_ptr == 0 {
            return Err(anyhow!("WorkDataManager.CardData is null"));
        }
        let piece_data_ptr = self.read_piece_data_ptr(work_data_manager_ptr)?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let mut value = WorkCardDataModel::read_model_value(inspector, card_data_ptr)?;

        if piece_data_ptr != 0 {
            match WorkPieceDataModel::read_model_value(inspector, piece_data_ptr) {
                Ok(piece_value) => {
                    // Extract the "pieces" array from the model output map
                    let pieces = match &piece_value {
                        Value::Map(entries) => entries
                            .iter()
                            .find(|(k, _)| k.as_str() == Some("pieces"))
                            .map(|(_, v)| v.clone()),
                        _ => None,
                    };
                    if let (Some(pieces_val), Value::Map(ref mut entries)) = (pieces, &mut value) {
                        entries.push((Value::from("piece_counts"), pieces_val));
                    }
                }
                Err(e) => {
                    eprintln!("[worker] piece data model failed: {e}");
                }
            }
        }

        Ok(pass_through_value(value))
    }

    pub fn get_user_data(&mut self) -> Result<UserData> {
        if let Some(user_data) = self.user_data.as_ref() {
            return Ok(user_data.clone());
        }

        let wdm_ptr = self.discover_work_data_manager_instance()?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        self.user_data = Some(UserData::extract_from_data_manager(inspector, wdm_ptr)?);

        Ok(self.user_data.as_ref().unwrap().clone())
    }

    pub fn extract_race_team_data(&mut self) -> Result<Value> {
        let race_mvc_ptr = self.resolve_race_mvc()?;

        // Detect race type first (before borrowing inspector for extraction)
        let race_type = {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;
            race_team::detect_race_type(inspector, race_mvc_ptr)
        };

        // Resolve single-mode TempData snapshot only for Standard (6)
        let sm_snapshot = if race_type == 6 {
            self.discover_temp_data().and_then(|temp_ptr| {
                let inspector = self.inspector.as_mut()?;
                let sm_ptr = inspector
                    .read_pointer_at(temp_ptr + 0xF8)
                    .ok()
                    .filter(|&p| p != 0)?;
                let race_id_off = inspector
                    .resolve_runtime_offset_for_object(
                        sm_ptr,
                        &["<RaceId>k__BackingField", "RaceId", "_raceId"],
                    )
                    .ok()?;
                let race_id = inspector.decode_obscured_int(sm_ptr + race_id_off).ok()?;
                let weather_off = inspector
                    .resolve_runtime_offset_for_object(
                        sm_ptr,
                        &[
                            "<RaceWeather>k__BackingField",
                            "RaceWeather",
                            "_raceWeather",
                        ],
                    )
                    .ok()?;
                let race_weather = inspector.read_i32_at(sm_ptr + weather_off).ok()?;
                Some((race_id as i64, race_weather as i64))
            })
        } else {
            None
        };

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        let mut result = race_team::extract_race_team_data(inspector, race_mvc_ptr, race_type)?;

        if let Ok(user_data) = self.get_user_data() {
            if let Value::Map(ref mut entries) = result {
                if let Some((_, metadata)) = entries
                    .iter_mut()
                    .find(|(k, _)| k.as_str() == Some("metadata"))
                {
                    if let Value::Map(ref mut meta_entries) = metadata {
                        meta_entries
                            .push((Value::from("viewer_id"), Value::from(user_data.trainer_id)));
                    }
                }
            }
        }

        // Only attach single-mode snapshot data when weather is non-zero
        // (avoids race_weather=0 overwriting the real weather from RaceInfoModel)
        if let Some((race_id, race_weather)) = sm_snapshot {
            if race_weather != 0 {
                if let Value::Map(ref mut entries) = result {
                    if let Some((_, metadata)) = entries
                        .iter_mut()
                        .find(|(k, _)| k.as_str() == Some("metadata"))
                    {
                        if let Value::Map(ref mut meta_entries) = metadata {
                            meta_entries.push((Value::from("race_id"), Value::from(race_id)));
                            meta_entries
                                .push((Value::from("race_weather"), Value::from(race_weather)));
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn resolve_race_mvc(&mut self) -> Result<u64> {
        let max_scan_bytes: usize = 512 * 1024 * 1024;

        let scene_manager_ptr = self.discover_scene_manager(max_scan_bytes)?;
        self.ensure_scene_offsets(scene_manager_ptr)?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        // _currentViewController is the correct field for RaceMainViewController
        if let Some(off) = self.current_view_controller_offset {
            let ptr = inspector.read_pointer_at(scene_manager_ptr + off)?;
            if ptr != 0 {
                return Ok(ptr);
            }
        }

        // Fallback: try current_view_base (should be RaceMainView, not the controller)
        if let Some(off) = self.current_view_base_offset {
            let ptr = inspector.read_pointer_at(scene_manager_ptr + off)?;
            if ptr != 0 {
                if let Ok(ctrl_off) = inspector.resolve_runtime_offset_for_object(
                    ptr,
                    &[
                        "_mainViewController",
                        "mainViewController",
                        "MainViewController",
                        "_raceMainViewController",
                        "raceMainViewController",
                    ],
                ) {
                    let ctrl_ptr = inspector.read_pointer_at(ptr + ctrl_off)?;
                    if ctrl_ptr != 0 {
                        return Ok(ctrl_ptr);
                    }
                }
            }
        }

        // Fallback: try current_scene_base
        if let Some(off) = self.current_scene_base_offset {
            let ptr = inspector.read_pointer_at(scene_manager_ptr + off)?;
            if ptr != 0 {
                for candidates in &[
                    &[
                        "_mainViewController",
                        "mainViewController",
                        "MainViewController",
                    ] as &[&str],
                    &["_raceMainViewController", "raceMainViewController"],
                ] {
                    if let Ok(ctrl_off) =
                        inspector.resolve_runtime_offset_for_object(ptr, candidates)
                    {
                        let ctrl_ptr = inspector.read_pointer_at(ptr + ctrl_off)?;
                        if ctrl_ptr != 0 {
                            return Ok(ctrl_ptr);
                        }
                    }
                }
            }
        }

        Err(anyhow!(
            "Could not discover RaceMainViewController. Are you on a race view? (view_id=400 or 4000)"
        ))
    }

    fn read_support_card_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.support_card_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &[
                    "<SupportCardData>k__BackingField",
                    "_supportCardData",
                    "supportCardData",
                    "SupportCardData",
                ],
            )?;
            self.support_card_data_offset = Some(offset);
        }

        let offset = self
            .support_card_data_offset
            .ok_or_else(|| anyhow!("SupportCardData offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }

    fn read_trophy_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.trophy_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &["<Trophy>k__BackingField", "_trophy", "trophy", "Trophy"],
            )?;
            self.trophy_data_offset = Some(offset);
        }

        let offset = self
            .trophy_data_offset
            .ok_or_else(|| anyhow!("Trophy offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }

    fn read_card_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.card_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &[
                    "<CardData>k__BackingField",
                    "_cardData",
                    "cardData",
                    "CardData",
                ],
            )?;
            self.card_data_offset = Some(offset);
        }

        let offset = self
            .card_data_offset
            .ok_or_else(|| anyhow!("CardData offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }

    fn read_piece_data_ptr(&mut self, work_data_manager_ptr: u64) -> Result<u64> {
        if self.piece_data_offset.is_none() {
            let inspector = self
                .inspector
                .as_mut()
                .ok_or_else(|| anyhow!("Process is not connected"))?;

            let offset = inspector.resolve_runtime_offset_for_object(
                work_data_manager_ptr,
                &[
                    "<PieceData>k__BackingField",
                    "_pieceData",
                    "pieceData",
                    "PieceData",
                ],
            )?;
            self.piece_data_offset = Some(offset);
        }

        let offset = self
            .piece_data_offset
            .ok_or_else(|| anyhow!("PieceData offset not resolved"))?;

        let inspector = self
            .inspector
            .as_mut()
            .ok_or_else(|| anyhow!("Process is not connected"))?;

        inspector
            .process_memory()
            .read_pointer(work_data_manager_ptr + offset)
    }
}

fn pass_through_value(value: RuntimeValue) -> Value {
    value
}

fn discover_singletons_via_metadata(pid: u32) -> Result<(Option<u64>, Option<u64>, Option<u64>)> {
    let mut memory = ProcessMemory::new(pid)?;
    let metadata = Il2CppMetadata::find_in_process(&mut memory, pid)?;

    let mut candidates = metadata.discover_singleton_candidates(&mut memory)?;
    if let Ok(mono_candidates) = metadata.discover_mono_singleton_candidates(&mut memory) {
        candidates.extend(mono_candidates);
    }

    // TempData doesn't inherit from Singleton<T> but has a static _instance field;
    // resolve it via metadata and add as a synthetic candidate.
    if let Ok(temp_ptr) =
        metadata.resolve_singleton_by_class_name(&mut memory, "Gallop", "TempData")
    {
        candidates.push(SingletonCandidate {
            namespace: "Gallop".to_string(),
            class_name: "TempData".to_string(),
            full_name: "Gallop::TempData".to_string(),
            generic_class_index: -1,
            singleton_class_ptr: 0,
            instance_ptr: Some(temp_ptr),
        });
    }

    let mut scene_manager_ptr = None;
    let mut work_data_manager_ptr = None;
    let mut temp_data_ptr = None;

    for candidate in candidates {
        let Some(instance_ptr) = candidate.instance_ptr else {
            continue;
        };
        if instance_ptr == 0 {
            continue;
        }

        if candidate.namespace == SCENE_MANAGER_NAMESPACE
            && candidate.class_name == SCENE_MANAGER_CLASS
            && scene_manager_ptr.is_none()
        {
            scene_manager_ptr = Some(instance_ptr);
        }

        if candidate.namespace == "Gallop"
            && candidate.class_name == "WorkDataManager"
            && work_data_manager_ptr.is_none()
        {
            work_data_manager_ptr = Some(instance_ptr);
        }

        if candidate.namespace == "Gallop"
            && candidate.class_name == "TempData"
            && temp_data_ptr.is_none()
        {
            temp_data_ptr = Some(instance_ptr);
        }

        if scene_manager_ptr.is_some() && work_data_manager_ptr.is_some() && temp_data_ptr.is_some()
        {
            break;
        }
    }

    if scene_manager_ptr.is_none() && work_data_manager_ptr.is_none() {
        return Err(anyhow!(
            "Could not resolve {}::{} or Gallop::WorkDataManager via singleton metadata",
            SCENE_MANAGER_NAMESPACE,
            SCENE_MANAGER_CLASS
        ));
    }

    Ok((scene_manager_ptr, work_data_manager_ptr, temp_data_ptr))
}
