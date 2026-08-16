use crate::styles::{spark_item::*, Style};
use shared::veteran_browser::SparkGroupRow;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SparkItemProps {
    pub spark: SparkGroupRow,
    #[prop_or(false)]
    pub highlighted: bool,
}

fn color_class(spark_type: i64) -> &'static str {
    match spark_type {
        1 => "spark-stat",
        2 => "spark-apt",
        3 => "spark-unique",
        _ => "spark-other",
    }
}

/// Splits spark rows into color groups. Blue/pink/green rows are color-distinguished
/// already; white sparks are split into separate Skill / Race / Scenario / Event rows
/// so they can be visually separated (second field = white sub-section). Empty groups
/// are skipped.
pub fn spark_group_rows(sparks: &[SparkGroupRow]) -> Vec<(bool, Vec<SparkGroupRow>)> {
    let mut rows = Vec::new();
    for (is_white, ty) in [
        (false, 1i64),
        (false, 2),
        (false, 3),
        (true, 4),
        (true, 5),
        (true, 6),
        (true, 7),
    ] {
        let items: Vec<SparkGroupRow> = sparks
            .iter()
            .filter(|s| s.spark_type == ty)
            .cloned()
            .collect();
        if !items.is_empty() {
            rows.push((is_white, items));
        }
    }
    rows
}

#[function_component]
pub fn SparkItem(props: &SparkItemProps) -> Html {
    let s = &props.spark;
    let cc = color_class(s.spark_type);
    let hc = if props.highlighted {
        " spark-highlighted"
    } else {
        ""
    };

    html! {
        <div class={format!("spark-item {}{}", cc, hc)}>
            <span class={SparkItemNameStyle::CLASS_NAME}>{ &s.name }</span>
            if s.veteran_level_sum > 0 {
                <span class={SparkItemVeteranStyle::CLASS_NAME}>{"("}{ s.veteran_level_sum }{"★)"}</span>
            }
            <span class={SparkItemTotalStyle::CLASS_NAME}>{ s.level_sum }{"★"}</span>
            { if s.uma_count > 1 { html! { <span class={SparkItemUmasStyle::CLASS_NAME}>{"×"}{ s.uma_count }</span> } } else { html! {} } }
        </div>
    }
}
