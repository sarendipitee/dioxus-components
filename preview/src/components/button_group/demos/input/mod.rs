use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::components::button_group::ButtonGroup;
use dioxus_components::input::*;
use dioxus_icons::lucide::Search;

#[component]
pub fn Demo() -> Element {
    let mut search = use_signal(String::new);

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1rem",
            ButtonGroup {
                "data-testid": "search-group",
                TextInput {
                    "data-testid": "search-input",
                    wrapper_attributes: attributes!(div {
                        style: "flex: 1; min-width: 14rem;"
                    }),
                    left_section: rsx! { Search { size: 16 } },
                    value: "{search}",
                    oninput: move |event: FormEvent| search.set(event.value()),
                    placeholder: "Search…",
                }
                Button {
                    variant: ButtonVariant::Default,
                    "Search"
                }
            }
            output { "data-testid": "search-value", "Query: {search}" }
        }
    }
}
