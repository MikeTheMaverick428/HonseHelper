use crate::styles::{
    detail_modal::{SparkColorRowStyle, SparkDetailListStyle},
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
    let all_sparks_vec = &props.spark_groups;

    let mut blue_sparks: Vec<SparkGroupInfo> = Vec::new();
    let mut pink_sparks: Vec<SparkGroupInfo> = Vec::new();
    let mut green_sparks: Vec<SparkGroupInfo> = Vec::new();
    let mut white_sparks: Vec<SparkGroupInfo> = Vec::new();

    for spark in all_sparks_vec {
        match spark.spark_type {
            SparkType::Stat => blue_sparks.push(spark.clone()),
            SparkType::Aptitude => pink_sparks.push(spark.clone()),
            SparkType::Unique => green_sparks.push(spark.clone()),
            _ => white_sparks.push(spark.clone()),
        }
    }

    if all_sparks_vec.is_empty() {
        return html! {
            <div class={SparkDetailListStyle::CLASS_NAME}>
                <p>{"No spark data."}</p>
            </div>
        };
    }

    html! {
        <div class={SparkDetailListStyle::CLASS_NAME}>
            {if !blue_sparks.is_empty() {
                html! {
                    <div class={SparkColorRowStyle::CLASS_NAME}>
                        {blue_sparks.iter().map(|spark_info| html! {
                            <SparkDisplay key={spark_info.spark_group_id} spark_info={spark_info.clone()} active_spark_filters={props.active_spark_filters.clone()} />
                        }).collect::<Html>()}
                    </div>
                }
            } else {
                html! {}
            }}
            {if !pink_sparks.is_empty() {
                html! {
                    <div class={SparkColorRowStyle::CLASS_NAME}>
                        {pink_sparks.iter().map(|spark_info| html! {
                            <SparkDisplay key={spark_info.spark_group_id} spark_info={spark_info.clone()} active_spark_filters={props.active_spark_filters.clone()} />
                        }).collect::<Html>()}
                    </div>
                }
            } else {
                html! {}
            }}
            {if !green_sparks.is_empty() {
                html! {
                    <div class={SparkColorRowStyle::CLASS_NAME}>
                        {green_sparks.iter().map(|spark_info| html! {
                            <SparkDisplay key={spark_info.spark_group_id} spark_info={spark_info.clone()} active_spark_filters={props.active_spark_filters.clone()} />
                        }).collect::<Html>()}
                    </div>
                }
            } else {
                html! {}
            }}
            {if !white_sparks.is_empty() {
                html! {
                    <div class={SparkColorRowStyle::CLASS_NAME}>
                        {white_sparks.iter().map(|spark_info| html! {
                            <SparkDisplay key={spark_info.spark_group_id} spark_info={spark_info.clone()} active_spark_filters={props.active_spark_filters.clone()} />
                        }).collect::<Html>()}
                    </div>
                }
            } else {
                html! {}
            }}
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
