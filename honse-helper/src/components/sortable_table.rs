use std::cmp::Ordering;
use std::marker::PhantomData;

use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Center,
}

impl Align {
    pub fn css(&self) -> &'static str {
        match self {
            Align::Left => "left",
            Align::Right => "right",
            Align::Center => "center",
        }
    }
}

/// Column configuration. `compare` provides the sorting logic; `sortable`
/// controls whether the column responds to clicks.
#[derive(Clone)]
pub struct ColumnCfg<T, C>
where
    T: 'static,
    C: PartialEq + 'static,
{
    pub key: C,
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub sortable: bool,
    pub default_dir: SortDir,
    pub align: Align,
    pub compare: fn(&T, &T) -> Ordering,
}

impl<T, C> PartialEq for ColumnCfg<T, C>
where
    T: 'static,
    C: PartialEq + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.label == other.label
            && self.title == other.title
            && self.sortable == other.sortable
            && self.default_dir == other.default_dir
            && self.align == other.align
    }
}

#[derive(Properties)]
pub struct SortableTableProps<T, C>
where
    T: PartialEq + Clone + 'static,
    C: PartialEq + Clone + 'static,
{
    pub rows: Vec<T>,
    pub columns: Vec<ColumnCfg<T, C>>,
    pub default_sort: Option<(C, SortDir)>,
    pub empty_message: &'static str,
    /// Renders a single `<td>` for the row at the given column index.
    pub cell_renderer: fn(&T, usize) -> Html,
}

impl<T, C> PartialEq for SortableTableProps<T, C>
where
    T: PartialEq + Clone + 'static,
    C: PartialEq + Clone + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
            && self.columns == other.columns
            && self.default_sort == other.default_sort
            && self.empty_message == other.empty_message
    }
}

pub enum SortableTableMsg<C> {
    SetSort(Option<(C, SortDir)>),
}

pub struct SortableTable<T, C> {
    _marker: PhantomData<(T, C)>,
    sort: Option<(C, SortDir)>,
}

impl<T, C> Component for SortableTable<T, C>
where
    T: PartialEq + Clone + 'static,
    C: PartialEq + Clone + 'static,
{
    type Message = SortableTableMsg<C>;
    type Properties = SortableTableProps<T, C>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _marker: PhantomData,
            sort: ctx.props().default_sort.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SortableTableMsg::SetSort(next) => {
                self.sort = next;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut rows = props.rows.clone();
        if let Some((key, dir)) = &self.sort {
            if let Some(col) = props.columns.iter().find(|c| &c.key == key) {
                rows.sort_by(|a, b| {
                    let ord = (col.compare)(a, b);
                    match dir {
                        SortDir::Asc => ord,
                        SortDir::Desc => ord.reverse(),
                    }
                });
            }
        }

        let header: Vec<Html> = props
            .columns
            .iter()
            .map(|col| self.header_cell(ctx, col))
            .collect();

        let body: Vec<Html> = if rows.is_empty() {
            vec![html! {
                <tr>
                    <td colspan={props.columns.len().to_string()} style="padding: 24px; text-align: center; color: #64748b;">
                        {props.empty_message}
                    </td>
                </tr>
            }]
        } else {
            rows.iter()
                .map(|row| {
                    html! {
                        <tr style="border-bottom: 1px solid #1e293b;">
                            { for (0..props.columns.len()).map(|i| (props.cell_renderer)(row, i)) }
                        </tr>
                    }
                })
                .collect()
        };

        html! {
            <table style="width: 100%; border-collapse: collapse; font-size: 13px;">
                <thead>
                    <tr style="background: #1e293b; color: #94a3b8; text-transform: uppercase; font-size: 11px; letter-spacing: 0.05em;">
                        { for header }
                    </tr>
                </thead>
                <tbody>
                    { for body }
                </tbody>
            </table>
        }
    }
}

impl<T, C> SortableTable<T, C>
where
    T: PartialEq + Clone + 'static,
    C: PartialEq + Clone + 'static,
{
    fn header_cell(&self, ctx: &Context<Self>, col: &ColumnCfg<T, C>) -> Html {
        let active = self.sort.as_ref().map(|(k, _)| k == &col.key).unwrap_or(false);
        let indicator = if !col.sortable {
            ""
        } else if active {
            match self.sort.as_ref().map(|(_, d)| d).unwrap_or(&SortDir::Asc) {
                SortDir::Asc => "▲",
                SortDir::Desc => "▼",
            }
        } else {
            "↕"
        };
        let cursor = if col.sortable {
            "cursor: pointer; user-select: none;"
        } else {
            ""
        };
        let style = format!(
            "position: sticky; top: 0; background: #1e293b; padding: 8px 12px; text-align: {}; border-bottom: 1px solid #334155; {}",
            col.align.css(),
            cursor,
        );

        let onclick = if col.sortable {
            let key = col.key.clone();
            let current = self.sort.clone();
            let default_dir = col.default_dir;
            let cb = ctx.link().callback(move |_: MouseEvent| {
                let next = match &current {
                    Some((k, d)) if *k == key => Some((key.clone(), match d {
                        SortDir::Asc => SortDir::Desc,
                        SortDir::Desc => SortDir::Asc,
                    })),
                    _ => Some((key.clone(), default_dir)),
                };
                SortableTableMsg::SetSort(next)
            });
            Some(cb)
        } else {
            None
        };

        html! {
            <th title={col.title.unwrap_or_default()} onclick={onclick} style={style}>
                {col.label}
                if !indicator.is_empty() {
                    <span style="margin-left: 4px; color: #f59e0b; font-size: 10px;">{indicator}</span>
                }
            </th>
        }
    }
}
