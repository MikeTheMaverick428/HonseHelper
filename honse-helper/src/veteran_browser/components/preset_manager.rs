use yew::prelude::*;

use super::custom_select::CustomSelect;
use super::searchable_select::SelectOption;
use crate::styles::{
    filter_panel::FilterInputStyle, legacy_planner::SecondaryBtnStyle, preset_manager::*, Style,
};

#[derive(Properties, PartialEq)]
pub struct PresetManagerProps {
    pub presets: Vec<String>,
    pub on_load: Callback<String>,
    pub on_save: Callback<String>,
    pub on_delete: Callback<String>,
}

#[function_component]
pub fn PresetManager(props: &PresetManagerProps) -> Html {
    let selected = use_state(String::new);
    let save_name = use_state(String::new);
    let show_save = use_state(|| false);

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |val: String| {
            selected.set(val);
        })
    };

    let on_load = {
        let selected = selected.clone();
        let on_load = props.on_load.clone();
        Callback::from(move |_| {
            let name = (*selected).clone();
            if !name.is_empty() {
                on_load.emit(name);
            }
        })
    };

    let on_delete = {
        let selected = selected.clone();
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| {
            let name = (*selected).clone();
            if !name.is_empty() {
                on_delete.emit(name);
            }
        })
    };

    let toggle_save = {
        let show_save = show_save.clone();
        Callback::from(move |_| {
            let mut current = (*show_save).clone();
            current = !current;
            show_save.set(current);
        })
    };

    let on_save_name_input = {
        let save_name = save_name.clone();
        Callback::from(move |e: InputEvent| {
            save_name.set(
                e.target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };

    let on_save_confirm = {
        let save_name = save_name.clone();
        let on_save = props.on_save.clone();
        let show_save = show_save.clone();
        Callback::from(move |_| {
            let name = (*save_name).clone();
            if !name.is_empty() {
                on_save.emit(name);
                show_save.set(false);
            }
        })
    };

    let preset_selected = {
        let v = selected.to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    };

    html! {
        <div class={PresetManagerStyle::CLASS_NAME}>
            <CustomSelect
                options={props.presets.iter().map(|name| SelectOption {
                    value: name.clone(),
                    label: name.clone(),
                }).collect::<Vec<_>>()}
                selected={preset_selected}
                on_change={on_select}
                placeholder={"Presets"}
                class={"preset-select"}
            />
            <button onclick={on_load} disabled={(*selected).is_empty()}>{"Load"}</button>
            <button onclick={toggle_save}>{"Save As"}</button>
            <button class={SecondaryBtnStyle::CLASS_NAME} onclick={on_delete} disabled={(*selected).is_empty()}>{"Delete"}</button>

            { if *show_save {
                html! {
                    <div class={PresetSaveRowStyle::CLASS_NAME}>
                        <input type="text" class={FilterInputStyle::CLASS_NAME} placeholder="Preset name..."
                            value={(*save_name).clone()}
                            oninput={on_save_name_input} />
                        <button onclick={on_save_confirm}>{"Save"}</button>
                    </div>
                }
            } else { html! {} } }
        </div>
    }
}
