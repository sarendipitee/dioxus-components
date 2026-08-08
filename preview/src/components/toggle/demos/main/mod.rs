use dioxus::prelude::*;
use dioxus_components::toggle::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled_pressed = use_signal(|| Some(false));
    let mut controlled_callback_count = use_signal(|| 0_u32);
    let mut uncontrolled_callback_count = use_signal(|| 0_u32);

    rsx! {
        Toggle {
            id: "toggle-basic",
            "data-testid": "toggle-basic",
            title: "Toggle global attributes",
            "name": "global-toggle",
            "data-audit": "toggle-global-attributes",
            width: "2rem",
            height: "2rem",
            em { "B" }
        }

        section {
            Toggle {
                id: "controlled-toggle",
                "data-testid": "controlled-toggle",
                pressed: controlled_pressed,
                on_pressed_change: move |pressed| {
                    controlled_pressed.set(Some(pressed));
                    controlled_callback_count += 1;
                },
                "Controlled toggle"
            }
            output { id: "controlled-state",
                if controlled_pressed().unwrap_or(false) { "On" } else { "Off" }
            }
            output { id: "controlled-count", "{controlled_callback_count}" }
        }

        Toggle {
            id: "default-pressed-toggle",
            "data-testid": "default-pressed-toggle",
            default_pressed: true,
            on_pressed_change: move |_| uncontrolled_callback_count += 1,
            "Default pressed toggle"
        }
        output { id: "uncontrolled-count", "{uncontrolled_callback_count}" }

        Toggle {
            id: "disabled-off-toggle",
            "data-testid": "disabled-off-toggle",
            disabled: true,
            "Disabled off toggle"
        }

        Toggle {
            id: "disabled-on-toggle",
            "data-testid": "disabled-on-toggle",
            default_pressed: true,
            disabled: true,
            "Disabled on toggle"
        }
    }
}
