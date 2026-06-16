use dioxus_components::switch::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            padding: "20px",
            gap: "15px",
            Switch {
                checked: checked(),
                aria_label: "Switch Demo",
                on_checked_change: move |new_checked| {
                    checked.set(new_checked);
                },
            }
        }
    }
}
