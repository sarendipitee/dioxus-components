use dioxus::prelude::*;
use dioxus_components::progress::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| Some(25.0));
    let mut max = use_signal(|| 100.0);

    rsx! {
        div {
            style: "display: grid; gap: 1rem;",
            div {
                style: "display: flex; flex-wrap: wrap; gap: 0.5rem;",
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(100.0);
                        value.set(Some(75.0));
                    },
                    "Set 75"
                }
                button {
                    r#type: "button",
                    onclick: move |_| value.set(None),
                    "Set indeterminate"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(100.0);
                        value.set(Some(125.0));
                    },
                    "Set above max"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(100.0);
                        value.set(Some(-25.0));
                    },
                    "Set below min"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(0.0);
                        value.set(Some(75.0));
                    },
                    "Set max zero"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(-10.0);
                        value.set(Some(5.0));
                    },
                    "Set max negative"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        max.set(200.0);
                        value.set(Some(50.0));
                    },
                    "Restore custom max/value"
                }
            }

            Progress {
                id: "progress-fixture",
                "data-testid": "progress-fixture",
                "data-progress-fixture": "controlled",
                class: "progress-fixture",
                title: "Controlled progress fixture",
                aria_label: "Progressbar Demo",
                aria_valuetext: "Progress fixture value",
                value: value(),
                max: max(),
                "Progress fixture content"
            }
        }
    }
}
