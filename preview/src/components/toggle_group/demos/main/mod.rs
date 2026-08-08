use dioxus::prelude::*;
use dioxus_components::toggle_group::*;
use std::collections::HashSet;

#[component]
pub fn Demo() -> Element {
    let mut controlled = use_signal(|| HashSet::from([0usize]));
    let mut callback_count = use_signal(|| 0usize);
    let controlled_value = controlled
        .read()
        .iter()
        .copied()
        .next()
        .map(|value| value.to_string())
        .unwrap_or_else(|| "empty".to_string());

    rsx! {
        section {
            h2 { "Multiple selection" }
            ToggleGroup {
                horizontal: true,
                allow_multiple_pressed: true,
                aria_label: "Text formatting",
                id: "multiple-group",
                "data-testid": "multiple-group",
                ToggleItem { index: 0usize, aria_label: "Bold", b { "B" } }
                ToggleItem { index: 1usize, aria_label: "Italic", i { "I" } }
                ToggleItem { index: 2usize, aria_label: "Underline", u { "U" } }
            }
        }

        section {
            h2 { "Controlled single selection" }
            ToggleGroup {
                horizontal: true,
                pressed: controlled(),
                aria_label: "Alignment",
                "data-testid": "controlled-single",
                on_pressed_change: move |next| {
                    controlled.set(next);
                    callback_count += 1;
                },
                ToggleItem { index: 0usize, "Left" }
                ToggleItem { index: 1usize, "Center" }
                ToggleItem { index: 2usize, "Right" }
            }
            output { "data-testid": "controlled-value", "{controlled_value}" }
            output { "data-testid": "callback-count", "{callback_count}" }
        }

        section {
            h2 { "Vertical with disabled item" }
            ToggleGroup {
                horizontal: false,
                aria_label: "Vertical options",
                "data-testid": "vertical-group",
                ToggleItem { index: 0usize, "Top" }
                ToggleItem { index: 1usize, disabled: true, "Middle" }
                ToggleItem { index: 2usize, "Bottom" }
            }
        }

        section {
            h2 { "No loop" }
            ToggleGroup {
                horizontal: true,
                roving_loop: false,
                aria_label: "No loop options",
                "data-testid": "no-loop-group",
                ToggleItem { index: 0usize, "First" }
                ToggleItem { index: 1usize, "Last" }
            }
        }

        section {
            h2 { "Right to left" }
            ToggleGroup {
                horizontal: true,
                rtl: true,
                dir: "rtl",
                aria_label: "RTL options",
                "data-testid": "rtl-group",
                ToggleItem { index: 0usize, "One" }
                ToggleItem { index: 1usize, "Two" }
                ToggleItem { index: 2usize, "Three" }
            }
        }

        section {
            h2 { "Disabled group" }
            ToggleGroup {
                horizontal: true,
                disabled: true,
                aria_label: "Disabled options",
                "data-testid": "disabled-group",
                ToggleItem { index: 0usize, "Disabled one" }
                ToggleItem { index: 1usize, "Disabled two" }
            }
        }
    }
}
