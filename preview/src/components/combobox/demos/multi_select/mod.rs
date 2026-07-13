use dioxus::prelude::*;
use dioxus_components::combobox::{ComboboxEmpty, ComboboxOption, MultiSelect};

#[component]
pub fn Demo() -> Element {
    let mut values = use_signal(|| Some(vec!["mushroom".to_string()]));
    let toppings: &[(&str, &str)] = &[
        ("pepperoni", "Pepperoni"),
        ("mushroom", "Mushroom"),
        ("onion", "Onion"),
        ("olive", "Olive"),
        ("jalapeno", "Jalapeno"),
    ];

    rsx! {
        div {
            MultiSelect::<String> {
                values,
                on_values_change: move |next| values.set(Some(next)),
                max_values: 3usize,
                render_value: |value: String| rsx! { "{value}" },
                placeholder: "Pick toppings...",
                ComboboxEmpty { "No toppings found." }
                for (index, (value, label)) in toppings.iter().enumerate() {
                    ComboboxOption::<String> {
                        index,
                        value: value.to_string(),
                        text_value: label.to_string(),
                        {*label}
                    }
                }
            }
            p {
                "Selected: {values().unwrap_or_default().join(\", \")}"
            }
        }
    }
}
