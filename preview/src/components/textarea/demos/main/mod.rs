use dioxus::prelude::*;
use dioxus_components::textarea::*;

#[component]
pub fn Demo() -> Element {
    let mut controlled_value = use_signal(|| "initial controlled value".to_string());
    let mut controlled_input_count = use_signal(|| 0usize);
    let mut form_status = use_signal(|| "Awaiting submission".to_string());
    let mut focus_status = use_signal(|| "Neither focused nor blurred".to_string());
    let mut reactive_label = use_signal(|| "Textarea reactive attributes: inactive".to_string());
    let mut reactive_active = use_signal(|| false);

    rsx! {
        div {
            display: "grid",
            gap: "1.5rem",
            max_width: "34rem",

            Textarea {
                id: "textarea-semantic-invalid",
                label: "Feedback",
                description: "Tell us what could be improved.",
                error: "Feedback is currently invalid.",
                required: true,
                placeholder: "Describe the issue",
                "data-testid": "textarea-semantic-invalid",
            }

            div { display: "grid", gap: ".5rem",
                Textarea {
                    id: "textarea-controlled",
                    label: "Controlled textarea",
                    value: controlled_value,
                    oninput: move |event: FormEvent| {
                        controlled_value.set(event.value());
                        controlled_input_count += 1;
                    },
                    "data-testid": "textarea-controlled",
                }
                button {
                    id: "textarea-controlled-update",
                    r#type: "button",
                    onclick: move |_| controlled_value.set("programmatic controlled update".to_string()),
                    "Set controlled value"
                }
                output {
                    id: "textarea-controlled-status",
                    "data-testid": "textarea-controlled-status",
                    "Input callbacks: {controlled_input_count}; value: {controlled_value}"
                }
            }

            Textarea {
                id: "textarea-uncontrolled",
                label: "Uncontrolled textarea",
                "data-testid": "textarea-uncontrolled",
                "Default uncontrolled content"
            }

            div { display: "grid", gap: ".5rem",
                Textarea {
                    id: "textarea-disabled",
                    label: "Disabled textarea",
                    disabled: true,
                    "data-testid": "textarea-disabled",
                    "Disabled content"
                }
                Textarea {
                    id: "textarea-readonly",
                    label: "Read-only textarea",
                    readonly: true,
                    "data-testid": "textarea-readonly",
                    "Read-only content"
                }
            }

            form {
                id: "textarea-form",
                onsubmit: move |event| {
                    event.prevent_default();
                    form_status.set("Submitted required textarea".to_string());
                },
                onreset: move |_| form_status.set("Reset required textarea".to_string()),
                div { display: "grid", gap: ".5rem",
                    Textarea {
                        id: "textarea-required-form",
                        label: "Required form textarea",
                        description: "This value is submitted as feedback.",
                        required: true,
                        name: "feedback",
                        "data-testid": "textarea-required-form",
                    }
                    div { display: "flex", gap: ".5rem",
                        button { id: "textarea-form-submit", r#type: "submit", "Submit textarea" }
                        button { id: "textarea-form-reset", r#type: "reset", "Reset textarea" }
                    }
                    output { id: "textarea-form-status", "data-testid": "textarea-form-status", "{form_status}" }
                }
            }

            Textarea {
                id: "textarea-constraints",
                label: "Constrained textarea",
                description: "Enter between 3 and 12 characters across 4 rows.",
                minlength: "3",
                maxlength: "12",
                rows: "4",
                "data-testid": "textarea-constraints",
                "four"
            }

            div { display: "grid", gap: ".5rem",
                Textarea {
                    id: "textarea-global-attributes",
                    label: "Global attributes",
                    class: "textarea-global-contract",
                    aria_label: "Textarea with forwarded global attributes",
                    "data-testid": "textarea-global-attributes",
                    "data-contract": "global-attributes",
                    "Global attributes"
                }
                Textarea {
                    id: "textarea-reactive-attributes",
                    label: "Reactive attributes",
                    aria_label: reactive_label,
                    "data-testid": "textarea-reactive-attributes",
                    "data-active": if reactive_active() { "true" } else { "false" },
                    "Reactive attributes"
                }
                button {
                    id: "textarea-reactive-toggle",
                    r#type: "button",
                    onclick: move |_| {
                        let active = !reactive_active();
                        reactive_active.set(active);
                        reactive_label.set(if active {
                            "Textarea reactive attributes: active".to_string()
                        } else {
                            "Textarea reactive attributes: inactive".to_string()
                        });
                    },
                    "Toggle reactive attributes"
                }
            }

            div { display: "grid", gap: ".5rem",
                Textarea {
                    id: "textarea-focus-status",
                    label: "Focus status textarea",
                    onfocus: move |_| focus_status.set("Focused".to_string()),
                    onblur: move |_| focus_status.set("Blurred".to_string()),
                    "data-testid": "textarea-focus-status",
                }
                output { id: "textarea-focus-output", "data-testid": "textarea-focus-output", "{focus_status}" }
            }
        }
    }
}
