use crate::styles::{
    detail_modal::{SparkColorRowStyle, SparkDetailListStyle, SparkWhiteRowStyle},
    spark_item::{
        SparkAptStyle, SparkHighlightedStyle, SparkItemNameStyle, SparkItemStyle,
        SparkItemTotalStyle, SparkItemUmasStyle, SparkItemVeteranStyle, SparkOtherStyle,
        SparkStatStyle, SparkUniqueStyle,
    },
    Style,
};
use shared::{filters::Filter, legacy_planner::SparkGroupInfo, models::SparkType};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SparksListProps {
    pub spark_groups: Vec<SparkGroupInfo>,
    #[prop_or_default]
    pub active_spark_filters: Vec<Filter>,
}

#[function_component]
pub fn SparksList(props: &SparksListProps) -> Html {
    let mut rows: Vec<(bool, Vec<SparkGroupInfo>)> = Vec::new();
    for (is_white, ty) in [
        (false, SparkType::Stat),
        (false, SparkType::Aptitude),
        (false, SparkType::Unique),
        (true, SparkType::Skill),
        (true, SparkType::Race),
        (true, SparkType::Scenario),
        (true, SparkType::Event),
    ] {
        let items: Vec<SparkGroupInfo> = props
            .spark_groups
            .iter()
            .filter(|s| s.spark_type == ty)
            .cloned()
            .collect();
        if !items.is_empty() {
            rows.push((is_white, items));
        }
    }

    if props.spark_groups.is_empty() {
        return html! {
            <div class={SparkDetailListStyle::CLASS_NAME}>
                <p>{"No spark data."}</p>
            </div>
        };
    }

    html! {
        <div class={SparkDetailListStyle::CLASS_NAME}>
            {for rows.iter().map(|(is_white, sparks)| {
                html! {
                    <div class={classes!(SparkColorRowStyle::CLASS_NAME, is_white.then_some(SparkWhiteRowStyle::CLASS_NAME))}>
                        {sparks.iter().map(|spark_info| html! {
                            <SparkDisplay key={spark_info.spark_group_id} spark_info={spark_info.clone()} active_spark_filters={props.active_spark_filters.clone()} />
                        }).collect::<Html>()}
                    </div>
                }
            })}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct SparkProps {
    pub spark_info: SparkGroupInfo,
    #[prop_or_default]
    pub active_spark_filters: Vec<Filter>,
}

#[function_component]
pub fn SparkDisplay(props: &SparkProps) -> Html {
    let spark_type = props.spark_info.spark_type;
    let color_class = match spark_type {
        SparkType::Stat => SparkStatStyle::CLASS_NAME,
        SparkType::Aptitude => SparkAptStyle::CLASS_NAME,
        SparkType::Unique => SparkUniqueStyle::CLASS_NAME,
        _ => SparkOtherStyle::CLASS_NAME,
    };

    let is_highlighted = props.active_spark_filters.iter().any(|filter| {
        match filter {
            Filter::Spark(f) => f.matches(&props.spark_info, false),
            Filter::WhiteSpark(f) => f.matches(&props.spark_info, false),
            _ => false,
        }
    });

    let highlight_class = if is_highlighted {
        format!(" {}", SparkHighlightedStyle::CLASS_NAME)
    } else {
        String::new()
    };

    let info = &props.spark_info;
    let cc = color_class;
    let hc = highlight_class;

    html! {
        <div class={format!("{} {}{}", SparkItemStyle::CLASS_NAME, cc, hc)}>
            <span class={SparkItemNameStyle::CLASS_NAME}>{ &info.name }</span>
            if info.trainee_stars_veteran > 0 {
                <span class={SparkItemVeteranStyle::CLASS_NAME}>{"("}{ info.trainee_stars_veteran }{"★)"}</span>
            }
            <span class={SparkItemTotalStyle::CLASS_NAME}>{ info.total_stars }{"★"}</span>
            { if info.uma_count > 1 { html! { <span class={SparkItemUmasStyle::CLASS_NAME}>{"×"}{ info.uma_count }</span> } } else { html! {} } }
        </div>
    }
}
