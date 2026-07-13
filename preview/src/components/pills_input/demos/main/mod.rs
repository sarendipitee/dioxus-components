use dioxus::prelude::*;
use dioxus_components::combobox::{ComboboxEmpty, ComboboxOption, PillsInput};
use dioxus_primitives::combobox::{Combobox, ComboboxDropdownTarget, ComboboxOptions, ComboboxSearch, Pill};

const TECHNOLOGIES: &[&str] = &["Dioxus", "Rust", "WebAssembly", "Leptos", "Tauri", "Cargo"];

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| vec!["Dioxus".to_string(), "Rust".to_string()]);

    rsx! {
        div {
            Combobox::<String> {
                on_value_change: move |val: Option<String>| {
                    if let Some(v) = val {
                        let mut s = selected();
                        if let Some(pos) = s.iter().position(|x| x == &v) {
                            s.remove(pos);
                        } else {
                            s.push(v);
                        }
                        selected.set(s);
                    }
                },
                PillsInput {
                    for (index, value) in selected().iter().cloned().enumerate() {
                        Pill {
                            key: "{index}",
                            on_remove: Some(Callback::new(move |_| {
                                let mut s = selected();
                                if index < s.len() {
                                    s.remove(index);
                                    selected.set(s);
                                }
                            })),
                            "{value}"
                        }
                    }
                    ComboboxSearch {
                        placeholder: "Add technology...",
                        show_selected_text: false,
                    }
                }
                ComboboxDropdownTarget {
                    ComboboxOptions {
                        ComboboxEmpty { "No technologies found." }
                        for (index, tech) in TECHNOLOGIES.iter().enumerate() {
                            ComboboxOption::<String> {
                                key: "{tech}",
                                index,
                                value: tech.to_string(),
                                text_value: tech.to_string(),
                                {*tech}
                            }
                        }
                    }
                }
            }
            p {
                "Selected: {selected().join(\", \")}"
            }
        }
    }
}
