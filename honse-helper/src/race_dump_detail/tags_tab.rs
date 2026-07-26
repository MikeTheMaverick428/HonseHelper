use crate::components::tag_modal::TagModal;
use crate::styles::{
    detail_modal::{DetailTabStyle, TabBtnStyle},
    tag_modal::{TagPillListStyle, TagPillStyle},
    Style,
};
use crate::tauri_bridge::invoke_tauri_command;
use serde_json::json;
use shared::veteran_browser::TagRow;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagsTabProps {
    pub race_dump_id: i64,
    pub initial_tags: Vec<TagRow>,
}

#[function_component]
pub fn TagsTab(props: &TagsTabProps) -> Html {
    let race_dump_tags = use_state(|| props.initial_tags.clone());
    let tag_modal_open = use_state(|| false);
    let tag_search_results = use_state(Vec::<TagRow>::new);

    {
        let race_dump_tags = race_dump_tags.clone();
        let race_dump_id = props.race_dump_id;
        use_effect_with((), move |_| {
            let race_dump_tags = race_dump_tags.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_race_dump_tags", json!({"raceDumpId": race_dump_id}))
                        .await
                {
                    if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                        race_dump_tags.set(tags);
                    }
                }
            });
            || ()
        });
    }

    let on_open_tag_modal = {
        let tag_modal_open = tag_modal_open.clone();
        let race_dump_tags = race_dump_tags.clone();
        let tag_search_results = tag_search_results.clone();
        let race_dump_id = props.race_dump_id;
        Callback::from(move |_| {
            let race_dump_tags = race_dump_tags.clone();
            let tag_search_results = tag_search_results.clone();
            let tag_modal_open = tag_modal_open.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("get_race_dump_tags", json!({"raceDumpId": race_dump_id}))
                        .await
                {
                    if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                        race_dump_tags.set(tags);
                    }
                }
                tag_search_results.set(Vec::new());
                tag_modal_open.set(true);
            });
        })
    };

    let on_close_tag_modal = {
        let tag_modal_open = tag_modal_open.clone();
        Callback::from(move |_| tag_modal_open.set(false))
    };

    let on_tag_search = {
        let tag_search_results = tag_search_results.clone();
        Callback::from(move |query: String| {
            if query.trim().is_empty() {
                tag_search_results.set(Vec::new());
                return;
            }
            let tag_search_results = tag_search_results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(result) =
                    invoke_tauri_command("search_tags", json!({"query": query})).await
                {
                    if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                        tag_search_results.set(tags);
                    }
                }
            });
        })
    };

    let on_tag_save = {
        let race_dump_tags = race_dump_tags.clone();
        let race_dump_id = props.race_dump_id;
        Callback::from(move |saved_tags: Vec<TagRow>| {
            let race_dump_tags = race_dump_tags.clone();
            let current = (*race_dump_tags).clone();
            wasm_bindgen_futures::spawn_local(async move {
                for tag in &current {
                    if !saved_tags.iter().any(|t| t.id == tag.id) {
                        let _ = invoke_tauri_command(
                            "untag_race_dump",
                            json!({"tagId": tag.id, "raceDumpId": race_dump_id}),
                        )
                        .await;
                    }
                }
                for tag in &saved_tags {
                    if current.iter().any(|t| t.id == tag.id && t.id != 0) {
                        continue;
                    }
                    if tag.id == 0 {
                        if let Ok(result) =
                            invoke_tauri_command("add_tag", json!({"tagValue": tag.tag_value}))
                                .await
                        {
                            if let Ok(new_tag) = serde_json::from_value::<TagRow>(result) {
                                let _ = invoke_tauri_command(
                                    "tag_race_dump",
                                    json!({"tagId": new_tag.id, "raceDumpId": race_dump_id}),
                                )
                                .await;
                            }
                        }
                    } else {
                        let _ = invoke_tauri_command(
                            "tag_race_dump",
                            json!({"tagId": tag.id, "raceDumpId": race_dump_id}),
                        )
                        .await;
                    }
                }
                if let Ok(result) =
                    invoke_tauri_command("get_race_dump_tags", json!({"raceDumpId": race_dump_id}))
                        .await
                {
                    if let Ok(tags) = serde_json::from_value::<Vec<TagRow>>(result) {
                        race_dump_tags.set(tags);
                    }
                }
                let _ = invoke_tauri_command(
                    "emit_race_dump_tags_changed",
                    json!({"raceDumpId": race_dump_id}),
                )
                .await;
            });
        })
    };

    html! {
        <>
            <div class={DetailTabStyle::CLASS_NAME} style="padding: 20px;">
                <h3>{"Tags"}</h3>
                if !(*race_dump_tags).is_empty() {
                    <div class={TagPillListStyle::CLASS_NAME}>
                        { for (*race_dump_tags).iter().map(|tag| {
                            html! {
                                <span class={TagPillStyle::CLASS_NAME}>
                                    { &tag.tag_value }
                                </span>
                            }
                        }) }
                    </div>
                } else {
                    <p style="color: #64748b; font-size: 13px;">{"No tags yet."}</p>
                }
                <button class={TabBtnStyle::CLASS_NAME}
                    onclick={on_open_tag_modal}>
                    {"+ Manage Tags"}
                </button>
            </div>

            <TagModal
                show={*tag_modal_open}
                title={"Manage Tags".to_string()}
                current_tags={(*race_dump_tags).clone()}
                search_results={(*tag_search_results).clone()}
                on_search={on_tag_search}
                on_save={on_tag_save}
                on_close={on_close_tag_modal}
            />
        </>
    }
}
