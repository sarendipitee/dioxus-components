use dioxus::prelude::*;
use dioxus_components::switch::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled_checked = use_signal(|| false);
    let mut callback_count = use_signal(|| 0usize);
    let mut controlled_disabled = use_signal(|| false);
    let mut controlled_required = use_signal(|| true);

    rsx! {
        div {
            display: "grid",
            padding: "20px",
            gap: "15px",
            form {
                "data-testid": "switch-form",
                display: "grid",
                gap: "8px",
                label { r#for: "email-notifications-switch", "Email notifications" }
                p {
                    id: "controlled-switch-description",
                    "Receive an email when account activity needs your attention."
                }
                Switch {
                    id: "email-notifications-switch",
                    "data-testid": "controlled-switch",
                    "data-fixture": "controlled-switch",
                    aria_label: "Email notifications",
                    aria_describedby: "controlled-switch-description",
                    checked: controlled_checked(),
                    disabled: controlled_disabled(),
                    required: controlled_required(),
                    name: "email-notifications",
                    value: "enabled",
                    on_checked_change: move |new_checked| {
                        controlled_checked.set(new_checked);
                        callback_count += 1;
                    },
                }
                output {
                    "data-testid": "controlled-switch-state",
                    {if controlled_checked() { "checked" } else { "unchecked" }}
                }
                output {
                    "data-testid": "controlled-switch-callback-count",
                    {format!("{}", callback_count())}
                }
            }
            button {
                r#type: "button",
                onclick: move |_| controlled_disabled.toggle(),
                "Disable controlled switch"
            }
            button {
                r#type: "button",
                onclick: move |_| controlled_required.toggle(),
                "Make controlled switch optional"
            }
            Switch {
                "data-testid": "uncontrolled-switch",
                aria_label: "Automatic updates",
                default_checked: true,
            }
            Switch {
                "data-testid": "disabled-unchecked-switch",
                aria_label: "Disabled unchecked switch",
                disabled: true,
            }
            Switch {
                "data-testid": "disabled-checked-switch",
                aria_label: "Disabled checked switch",
                default_checked: true,
                disabled: true,
            }
        }
    }
}
