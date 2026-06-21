use super::{
    ComponentDemoData, ComponentDemoEntryData, ComponentType, HighlightedCode, PropMetadata,
};
use dioxus::prelude::{asset, manganis};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ComponentCategory {
    Forms,
    Schedule,
    Combobox,
    Navigation,
    Overlays,
    Feedback,
    Disclosure,
    DataDisplay,
}

impl ComponentCategory {
    pub const ALL: &'static [Self] = &[
        Self::Forms,
        Self::Schedule,
        Self::Combobox,
        Self::Navigation,
        Self::Overlays,
        Self::Feedback,
        Self::Disclosure,
        Self::DataDisplay,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Forms => "Forms",
            Self::Schedule => "Schedule",
            Self::Combobox => "Combobox",
            Self::Navigation => "Navigation",
            Self::Overlays => "Overlays",
            Self::Feedback => "Feedback",
            Self::Disclosure => "Disclosure",
            Self::DataDisplay => "Data display",
        }
    }
}

pub fn category_of(name: &str) -> ComponentCategory {
    match name {
        "button" | "input" | "text_input" | "textarea" | "label" | "checkbox" | "switch"
        | "radio_group" | "toggle" | "toggle_group" | "select" | "slider" | "calendar"
        | "date_picker" | "date_input" | "color_picker" | "color_input" | "time_picker"
        | "time_input" | "file_drop_zone" => ComponentCategory::Forms,
        "schedule"
        | "schedule_day_view"
        | "schedule_week_view"
        | "schedule_month_view"
        | "schedule_year_view"
        | "schedule_mobile_month_view"
        | "schedule_recurring"
        | "schedule_events" => ComponentCategory::Schedule,
        "combobox" | "autocomplete" | "multi_select" | "pills_input" | "tags_input" => {
            ComponentCategory::Combobox
        }
        "navbar" | "sidebar" | "tabs" | "pagination" | "menubar" | "toolbar" | "context_menu"
        | "dropdown_menu" | "table_of_contents" => ComponentCategory::Navigation,
        "dialog" | "alert_dialog" | "sheet" | "popover" | "tooltip" | "hover_card" => {
            ComponentCategory::Overlays
        }
        "toast" | "progress" | "skeleton" | "badge" | "alert" => ComponentCategory::Feedback,
        "accordion" | "collapsible" => ComponentCategory::Disclosure,
        "avatar" | "card" | "separator" | "aspect_ratio" | "item" | "drag_and_drop_list"
        | "virtual_list" | "scroll_area" | "split_pane" | "data_table" => {
            ComponentCategory::DataDisplay
        }
        _ => ComponentCategory::DataDisplay,
    }
}

/// Display label for a component in the sidebar and on its page header.
///
/// Schedule sub-pages use curated labels (e.g. `DayView`); everything else falls back to the
/// underscore-to-space form of its registry name.
pub fn label_of(name: &str) -> String {
    match name {
        "schedule" => "Schedule".to_string(),
        "schedule_day_view" => "DayView".to_string(),
        "schedule_week_view" => "WeekView".to_string(),
        "schedule_month_view" => "MonthView".to_string(),
        "schedule_year_view" => "YearView".to_string(),
        "schedule_mobile_month_view" => "MobileMonthView".to_string(),
        "schedule_recurring" => "Recurring events".to_string(),
        "schedule_events" => "Events data".to_string(),
        _ => name.replace('_', " "),
    }
}

/// The installable registry name for a component page.
///
/// Schedule sub-pages are documentation views of the single installable `schedule` component,
/// so they all resolve to `schedule` for the `dx components add` command.
pub fn install_name(name: &str) -> &str {
    if name.starts_with("schedule") {
        "schedule"
    } else {
        name
    }
}

/// Whether a component appears as a card in the home catalog gallery.
///
/// Schedule sub-pages live only under the Schedule sidebar grouping, so they are excluded from
/// the catalog (which keeps a single `schedule` card).
pub fn in_catalog(name: &str) -> bool {
    !matches!(
        name,
        "schedule_day_view"
            | "schedule_week_view"
            | "schedule_month_view"
            | "schedule_year_view"
            | "schedule_mobile_month_view"
            | "schedule_recurring"
            | "schedule_events"
    )
}

macro_rules! examples {
    ($($name:ident $(($kind:ident))? $([$($demo:ident),*])?),* $(,)?) => {
        $(
            pub(crate) mod $name {
                #[allow(unused)]
                pub use dioxus_components::*;
                pub(crate) mod demos {
                    pub(crate) mod main;
                    $(
                        $(
                            pub(crate) mod $demo;
                        )*
                    )?
                }
            }
        )*
        pub(crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                examples!(@demo $name $( $kind )? $([$($demo),*])?),
            )*
        ];
    };

    (@kind) => { ComponentType::Normal };
    (@kind normal) => { ComponentType::Normal };
    (@kind block) => { ComponentType::Block };
    (@demo_name r#static) => { "static" };
    (@demo_name $demo:ident) => { stringify!($demo) };
    (@demo_path r#static) => { "static" };
    (@demo_path $demo:ident) => { stringify!($demo) };
    (@main_demo_description $name:ident) => {
        include_str!(concat!(
            env!("OUT_DIR"),
            "/",
            stringify!($name),
            "/demos/main/description.html"
        ))
    };
    (@demo_description $name:ident $demo:ident) => {
        include_str!(concat!(
            env!("OUT_DIR"),
            "/",
            stringify!($name),
            "/demos/",
            examples!(@demo_path $demo),
            "/description.html"
        ))
    };
    (@demo_source_asset $name:ident r#static) => {
        asset!(concat!("/assets/generated/", stringify!($name), "/demos/static/mod.rs.html"))
    };
    (@demo_source_asset $name:ident $demo:ident) => {
        asset!(concat!("/assets/generated/", stringify!($name), "/demos/", stringify!($demo), "/mod.rs.html"))
    };

    // Normal components: no demo-level css_highlighted
    (@demo $name:ident $([$($demo:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Normal,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs_path: asset!(concat!("/assets/generated/", stringify!($name), "/docs.html")),
            props: include!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/props.rs")),
            component: HighlightedCode {
                asset: asset!(concat!("/assets/generated/", stringify!($name), "/component.rs.html")),
            },
            style: HighlightedCode {
                asset: asset!(concat!("/assets/generated/", stringify!($name), "/style.css.html")),
            },
            demos: &[
                ComponentDemoEntryData {
                    name: "main",
                    description_html: examples!(@main_demo_description $name),
                    rs_highlighted: HighlightedCode {
                        asset: asset!(concat!("/assets/generated/", stringify!($name), "/demos/main/mod.rs.html")),
                    },
                    css_highlighted: None,
                    component: $name::demos::main::Demo,
                },
                $(
                    $(
                        ComponentDemoEntryData {
                            name: examples!(@demo_name $demo),
                            description_html: examples!(@demo_description $name $demo),
                            rs_highlighted: HighlightedCode {
                                asset: examples!(@demo_source_asset $name $demo),
                            },
                            css_highlighted: None,
                            component: $name::demos::$demo::Demo,
                        },
                    )*
                )?
            ],
        }
    };

    // Block components: rendered in iframe, with shared demo.css
    (@demo $name:ident block $([$($demo:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Block,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs_path: asset!(concat!("/assets/generated/", stringify!($name), "/docs.html")),
            props: include!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/props.rs")),
            component: HighlightedCode {
                asset: asset!(concat!("/assets/generated/", stringify!($name), "/component.rs.html")),
            },
            style: HighlightedCode {
                asset: asset!(concat!("/assets/generated/", stringify!($name), "/style.css.html")),
            },
            demos: &[
                ComponentDemoEntryData {
                    name: "main",
                    description_html: examples!(@main_demo_description $name),
                    rs_highlighted: HighlightedCode {
                        asset: asset!(concat!("/assets/generated/", stringify!($name), "/demos/main/mod.rs.html")),
                    },
                    css_highlighted: Some(HighlightedCode {
                        asset: asset!(concat!("/assets/generated/", stringify!($name), "/demos/demo.css.html")),
                    }),
                    component: $name::demos::main::Demo,
                },
                $(
                    $(
                        ComponentDemoEntryData {
                            name: examples!(@demo_name $demo),
                            description_html: examples!(@demo_description $name $demo),
                            rs_highlighted: HighlightedCode {
                                asset: examples!(@demo_source_asset $name $demo),
                            },
                            css_highlighted: Some(HighlightedCode {
                                asset: asset!(concat!("/assets/generated/", stringify!($name), "/demos/demo.css.html")),
                            }),
                            component: $name::demos::$demo::Demo,
                        },
                    )*
                )?
            ],
        }
    };
}

examples!(
    accordion,
    alert,
    alert_dialog[destructive],
    aspect_ratio,
    avatar,
    badge,
    button[size, icon],
    calendar[simple, internationalized, range, multi_month, unavailable_dates],
    card,
    checkbox[label_description, element_label, disabled, indeterminate],
    collapsible,
    color_input,
    color_picker,
    autocomplete,
    combobox[controlled, disabled, dynamic, autocomplete, multi_select, tags_input, virtualized],
    context_menu,
    date_input,
    date_picker[internationalized, range, multi_month, unavailable_dates],
    data_table[controlled, server_backed, expansion, virtualized, density, selectable],
    dialog[scrollable, form],
    drag_and_drop_list[removable],
    dropdown_menu[checkable, structure, presentation, nested_submenus],
    file_drop_zone[
        images_only,
        max_size,
        max_count,
        single_file,
        disabled,
        loading,
        open_button,
        rejected,
        custom_content
    ],
    hover_card,
    input[sections, composition],
    item[variant, size, image, group],
    label,
    menubar,
    multi_select,
    navbar,
    pagination[controlled],
    pills_input,
    popover,
    progress,
    radio_group,
    scroll_area,
    schedule[
        controlled,
        r#static,
        internationalized,
        custom_header,
        custom_event,
        multi_view,
        multi_day,
        drag_and_drop,
        external_drop,
        resize,
        slot_selection
    ],
    schedule_day_view,
    schedule_month_view,
    schedule_week_view,
    schedule_year_view,
    schedule_mobile_month_view,
    schedule_recurring,
    schedule_events,
    select[multi],
    separator,
    sheet,
    sidebar(block)[floating, inset],
    skeleton,
    split_pane[
        vertical,
        multi_pane,
        controlled,
        constraints,
        nested,
        snap,
        custom_divider,
        persistence
    ],
    slider[dynamic_range, range],
    switch,
    tabs[manual, vertical, controlled, pills],
    table_of_contents,
    text_input[description, error, size, sections],
    textarea[outline, fade, ghost, bottom_section, autosize, resize],
    tags_input,
    time_input[with_seconds, presets, with_picker],
    time_picker[clearable, seconds_12_hour, duration, presets],
    toast[with_description, permanent, custom_duration, with_action, loading],
    toggle,
    toggle_group,
    toolbar,
    tooltip,
    virtual_list[random_heights],
);
