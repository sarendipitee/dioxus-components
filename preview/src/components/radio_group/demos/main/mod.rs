use dioxus::prelude::*;
use dioxus_components::radio_group::*;

#[component]
pub fn Demo() -> Element {
    let mut preferred_color = use_signal(|| Some("blue".to_string()));
    let mut callback_count = use_signal(|| 0usize);

    rsx! {
        form {
            "data-testid": "radio-form",
            p { id: "radio-help", "Selection is submitted with the form." }
            RadioGroup {
                id: "preferred-color-group",
                "data-testid": "preferred-color-group",
                "data-fixture": "controlled-radio-group",
                aria_label: "Preferred color choices",
                aria_describedby: "radio-help",
                label: "Preferred color",
                description: "Choose one available color.",
                value: preferred_color,
                required: true,
                name: "preferred-color",
                on_value_change: move |value| {
                    preferred_color.set(Some(value));
                    callback_count += 1;
                },
                RadioItem { id: "color-blue", value: "blue", index: 0usize, "Blue" }
                RadioItem { id: "color-red", value: "red", index: 1usize, "Red" }
                RadioItem { id: "color-green", value: "green", index: 2usize, disabled: true, "Green" }
            }
            output { "data-testid": "preferred-color-value", {preferred_color().unwrap_or_default()} }
            output { "data-testid": "preferred-color-callback-count", {format!("{}", callback_count())} }
        }
        RadioGroup {
            label: "Disabled sizes",
            disabled: true,
            RadioItem { value: "small", index: 0usize, "Small" }
            RadioItem { value: "large", index: 1usize, "Large" }
        }
        RadioGroup {
            id: "density-group",
            "data-testid": "density-group",
            label: "Density",
            horizontal: true,
            roving_loop: false,
            RadioItem { value: "compact", index: 0usize, "Compact" }
            RadioItem { value: "comfortable", index: 1usize, "Comfortable" }
            RadioItem { value: "spacious", index: 2usize, "Spacious" }
        }
    }
}
