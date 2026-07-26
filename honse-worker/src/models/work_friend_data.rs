use crate::models::friend_entry::FriendEntryModel;
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct WorkFriendDataModel;

pub const KEY_FOLLOW_LIST: &str = "follow_list";
pub const KEY_FOLLOWER_LIST: &str = "follower_list";
pub const KEY_RECOMMEND_LIST: &str = "recommend_list";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for WorkFriendDataModel {
    fn model_name() -> &'static str {
        "WorkFriendData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_FOLLOW_LIST,
                emit: true,
                required: true,
                candidates: &["_followList", "followList", "FollowList"],
                reader: FieldReaderKind::PointerList(FriendEntryModel::read_model_value),
            },
            FieldSpec {
                key: KEY_FOLLOWER_LIST,
                emit: true,
                required: false,
                candidates: &["_followerList", "followerList", "FollowerList"],
                reader: FieldReaderKind::PointerList(FriendEntryModel::read_model_value),
            },
            FieldSpec {
                key: KEY_RECOMMEND_LIST,
                emit: true,
                required: false,
                candidates: &["_recommendList", "recommendList", "RecommendList"],
                reader: FieldReaderKind::PointerList(FriendEntryModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
