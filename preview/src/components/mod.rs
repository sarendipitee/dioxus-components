use super::{
    ComponentDemoData, ComponentType, ComponentVariantDemoData, HighlightedCode, PropMetadata,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ComponentCategory {
    Forms,
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
        | "date_picker" | "date_input" | "schedule" | "color_picker" | "color_input"
        | "time_picker" | "time_input" => ComponentCategory::Forms,
        "combobox" | "autocomplete" | "multi_select" | "pills_input" | "tags_input" => {
            ComponentCategory::Combobox
        }
        "navbar" | "sidebar" | "tabs" | "pagination" | "menubar" | "toolbar" | "context_menu"
        | "dropdown_menu" | "table_of_contents" => ComponentCategory::Navigation,
        "dialog" | "alert_dialog" | "sheet" | "popover" | "tooltip" | "hover_card" => {
            ComponentCategory::Overlays
        }
        "toast" | "progress" | "skeleton" | "badge" => ComponentCategory::Feedback,
        "accordion" | "collapsible" => ComponentCategory::Disclosure,
        "avatar" | "card" | "separator" | "aspect_ratio" | "item" | "drag_and_drop_list"
        | "virtual_list" | "scroll_area" | "split_pane" | "data_table" => {
            ComponentCategory::DataDisplay
        }
        _ => ComponentCategory::DataDisplay,
    }
}

macro_rules! examples {
    ($($name:ident $(($kind:ident))? $([$($variant:ident),*])?),* $(,)?) => {
        $(
            pub(crate) mod $name {
                #[allow(unused)]
                pub use dioxus_components::*;
                pub(crate) mod variants {
                    pub(crate) mod main;
                    $(
                        $(
                            pub(crate) mod $variant;
                        )*
                    )?
                }
            }
        )*
        pub(crate) static DEMOS: &[ComponentDemoData] = &[
            $(
                examples!(@demo $name $( $kind )? $([$($variant),*])?),
            )*
        ];
    };

    (@kind) => { ComponentType::Normal };
    (@kind normal) => { ComponentType::Normal };
    (@kind block) => { ComponentType::Block };
    (@variant_name r#static) => { "static" };
    (@variant_name $variant:ident) => { stringify!($variant) };
    (@variant_path r#static) => { "static" };
    (@variant_path $variant:ident) => { stringify!($variant) };

    // Normal components: no variant-level css_highlighted
    (@demo $name:ident $([$($variant:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Normal,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            props: include!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/props.rs")),
            component: HighlightedCode {
                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.html")),
            },
            style: HighlightedCode {
                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.html")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.html")),
                    },
                    css_highlighted: None,
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: examples!(@variant_name $variant),
                            rs_highlighted: HighlightedCode {
                                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", examples!(@variant_path $variant), "/mod.rs.html")),
                            },
                            css_highlighted: None,
                            component: $name::variants::$variant::Demo,
                        },
                    )*
                )?
            ],
        }
    };

    // Block components: rendered in iframe, with shared demo.css
    (@demo $name:ident block $([$($variant:ident),*])?) => {
        ComponentDemoData {
            name: stringify!($name),
            r#type: ComponentType::Block,
            description: include_str!(concat!(
                env!("OUT_DIR"),
                "/",
                stringify!($name),
                "/description.txt"
            )),
            docs: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/docs.html")),
            props: include!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/props.rs")),
            component: HighlightedCode {
                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/component.rs.html")),
            },
            style: HighlightedCode {
                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/style.css.html")),
            },
            variants: &[
                ComponentVariantDemoData {
                    name: "main",
                    rs_highlighted: HighlightedCode {
                        html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/main/mod.rs.html")),
                    },
                    css_highlighted: Some(HighlightedCode {
                        html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.html")),
                    }),
                    component: $name::variants::main::Demo,
                },
                $(
                    $(
                        ComponentVariantDemoData {
                            name: examples!(@variant_name $variant),
                            rs_highlighted: HighlightedCode {
                                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/", examples!(@variant_path $variant), "/mod.rs.html")),
                            },
                            css_highlighted: Some(HighlightedCode {
                                html: include_str!(concat!(env!("OUT_DIR"), "/", stringify!($name), "/variants/demo.css.html")),
                            }),
                            component: $name::variants::$variant::Demo,
                        },
                    )*
                )?
            ],
        }
    };
}

examples!(
    accordion,
    alert_dialog,
    aspect_ratio,
    avatar,
    badge,
    button[size, icon],
    calendar[simple, internationalized, range, multi_month, unavailable_dates],
    card,
    checkbox,
    collapsible,
    color_input,
    color_picker,
    autocomplete,
    combobox[controlled, disabled, dynamic, autocomplete, multi_select, tags_input, virtualized],
    context_menu,
    date_input,
    date_picker[internationalized, range, multi_month, unavailable_dates],
    data_table[controlled, server_backed, expansion, virtualized],
    dialog,
    drag_and_drop_list[removable],
    dropdown_menu,
    hover_card,
    input[sections, composition],
    item[variant, size, image, group],
    label,
    menubar,
    multi_select,
    navbar,
    pagination,
    pills_input,
    popover,
    progress,
    radio_group,
    scroll_area,
    schedule[
        controlled,
        week,
        responsive,
        r#static,
        internationalized,
        day,
        month,
        year,
        drag_and_drop,
        external_drop,
        resize,
        slot_selection,
        custom_header,
        custom_event,
        recurring,
        multi_view
    ],
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
    tabs[manual, vertical, controlled],
    table_of_contents,
    text_input[description, error, size, sections],
    textarea[outline, fade, ghost, bottom_section, autosize, resize],
    tags_input,
    time_input,
    time_picker[clearable, seconds_12_hour, duration],
    toast,
    toggle,
    toggle_group,
    toolbar,
    tooltip,
    virtual_list[random_heights],
);
