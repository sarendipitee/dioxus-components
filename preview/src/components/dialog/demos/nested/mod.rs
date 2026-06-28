use crate::components::dialog::*;
use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::select::*;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[component]
pub fn Demo() -> Element {
    let mut outer_open = use_signal(|| false);
    let mut inner_open = use_signal(|| false);

    rsx! {
        Button { onclick: move |_| outer_open.set(true), "Open Dialog" }
        Dialog {
            open: outer_open(),
            on_open_change: move |v| outer_open.set(v),
            title: "Manage task",
            width: "40vw",
            description: "Review this task and optionally open the priority settings dialog.",
            footer: rsx! {
                Button { onclick: move |_| outer_open.set(false), "Close" }
            },
            div {
                margin_top: "5rem",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| inner_open.set(true),
                    "Set Priority"
                }
            }
            // Inner dialog + select nested inside outer dialog
            Dialog {
                open: inner_open(),
                on_open_change: move |v| inner_open.set(v),
                title: "Set priority",
                description: "Choose a priority level for this task.",
                footer: rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| inner_open.set(false),
                        "Cancel"
                    }
                    Button { onclick: move |_| inner_open.set(false), "Apply" }
                },
                Select::<Option<Priority>> {
                    width: "12rem",
                    SelectGroup {
                        SelectGroupLabel { "Priority" }
                        {Priority::iter().enumerate().map(|(i, p)| rsx! {
                            SelectOption::<Option<Priority>> {
                                index: i,
                                value: p,
                                text_value: "{p}",
                                "{p}"
                            }
                        })}
                        SelectOption::<Option<Priority>> {
                            index: Priority::COUNT,
                            value: None,
                            text_value: "None",
                            "None"
                        }
                    }
                }
            }
        }
    }
}
