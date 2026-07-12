use dioxus_components::combobox::*;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut show_svelte = use_signal(|| true);
    let mut show_solid = use_signal(|| true);
    let mut patient_loaded = use_signal(|| false);

    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 20rem;",
            div { style: "display: flex; gap: 0.5rem;",
                button {
                    r#type: "button",
                    tabindex: "-1",
                    "data-overlay-dismiss-ignore": "true",
                    onpointerdown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        show_svelte.toggle();
                    },
                    onmousedown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                    },
                    onkeydown: move |event| {
                        let key = event.key();
                        if matches!(key, Key::Enter) || matches!(key, Key::Character(ch) if ch == " ") {
                            event.prevent_default();
                            show_svelte.toggle();
                        }
                    },
                    "Toggle SvelteKit"
                }
                button {
                    r#type: "button",
                    tabindex: "-1",
                    "data-overlay-dismiss-ignore": "true",
                    onpointerdown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        show_solid.toggle();
                    },
                    onmousedown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                    },
                    onkeydown: move |event| {
                        let key = event.key();
                        if matches!(key, Key::Enter) || matches!(key, Key::Character(ch) if ch == " ") {
                            event.prevent_default();
                            show_solid.toggle();
                        }
                    },
                    "Toggle SolidStart"
                }
                button {
                    r#type: "button",
                    tabindex: "-1",
                    "data-overlay-dismiss-ignore": "true",
                    onpointerdown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        spawn(async move {
                            gloo_timers::future::sleep(Duration::from_millis(150)).await;
                            patient_loaded.set(true);
                        });
                    },
                    onmousedown: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                    },
                    "Load matching patient"
                }
                if patient_loaded() {
                    span { "data-testid": "dynamic-patient-loaded", "Matching patient loaded" }
                }
            }
            Combobox::<String> {
                placeholder: "Select framework...",
                aria_label: "Dynamic framework",
                list_aria_label: "Dynamic frameworks",
                ComboboxEmpty { "No framework found." }
                ComboboxOption::<String> {
                    index: 0usize,
                    value: "next".to_string(),
                    text_value: "Next.js",
                    "Next.js"
                }
                if show_svelte() {
                    ComboboxOption::<String> {
                        index: 1usize,
                        value: "svelte".to_string(),
                        text_value: "SvelteKit",
                        "SvelteKit"
                    }
                }
                if show_solid() {
                    ComboboxOption::<String> {
                        index: 2usize,
                        value: "solid".to_string(),
                        text_value: "SolidStart",
                        "SolidStart"
                    }
                }
                ComboboxOption::<String> {
                    index: 3usize,
                    value: "dioxus".to_string(),
                    text_value: "Dioxus",
                    "Dioxus"
                }
                if patient_loaded() {
                    ComboboxOption::<String> {
                        index: 4usize,
                        value: "ada-lovelace".to_string(),
                        text_value: "Ada Lovelace",
                        "Ada Lovelace"
                    }
                }
            }
        }
    }
}
