use dioxus::prelude::*;
use dioxus_components::alert::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut shell_value = use_signal(|| "release-notes".to_string());
    let mut clearable_shell_val = use_signal(|| "release-notes".to_string());
    let mut clearable_base_val = use_signal(|| "uncontrolled".to_string());
    let mut recommended_email = use_signal(String::new);

    rsx! {
        div {
            style: "display: grid; gap: 1.5rem; max-width: 28rem;",

            Alert {
                variant: AlertVariant::Info,
                title: "Low-Level Component Notice",
                description: "Input and InputBase are internal primitives for building custom form controls. Standard application code should generally use TextInput, NumberInput, or PasswordInput instead of composing raw Input directly.",
            }

            InputBase {
                label: rsx! { "Raw Input Shell (Low-Level)" },
                description: rsx! { "Direct Input container wrapping a raw input element." },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: shell_value,
                    placeholder: "release-notes",
                    oninput: move |e: FormEvent| shell_value.set(e.value()),
                }
            }

            output {
                id: "input-shell-value",
                "Shell value: {shell_value}"
            }

            InputBase {
                label: rsx! { "Labeled shell" },
                description: rsx! { "InputBase adds wrapper metadata around the same shell." },
                left_section: rsx! { span { "#" } },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    placeholder: "project-slug",
                }
            }

            Input {
                right_section: rsx! {
                    InputClearButton {
                        aria_label: "Clear value",
                        disabled: clearable_shell_val.read().is_empty(),
                        onclick: move |_| clearable_shell_val.set(String::new()),
                    }
                },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: clearable_shell_val,
                    placeholder: "clearable-shell",
                    oninput: move |e: FormEvent| clearable_shell_val.set(e.value()),
                }
            }

            InputBase {
                label: rsx! { "Clearable InputBase" },
                description: rsx! { "InputBase scaffold with a clear button in the right section." },
                right_section: rsx! {
                    InputClearButton {
                        aria_label: "Clear value",
                        disabled: clearable_base_val.read().is_empty(),
                        onclick: move |_| clearable_base_val.set(String::new()),
                    }
                },
                input {
                    style: "width: 100%; border: 0; background: transparent; outline: none;",
                    value: clearable_base_val,
                    placeholder: "clearable-base",
                    oninput: move |e: FormEvent| clearable_base_val.set(e.value()),
                }
            }

            TextInput {
                label: rsx! { "Recommended: TextInput" },
                description: rsx! { "High-level, production-ready form control for native text entry." },
                placeholder: "user@example.com",
                value: recommended_email,
                clearable: true,
                oninput: move |e: FormEvent| recommended_email.set(e.value()),
            }
        }
    }
}
