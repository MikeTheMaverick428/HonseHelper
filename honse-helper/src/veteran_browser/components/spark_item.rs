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
