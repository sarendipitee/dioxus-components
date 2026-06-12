use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| "staging".to_string());

    rsx! {
        div {
            style: "display: grid; gap: 1rem; max-width: 24rem;",
            InputBase {
                label: rsx! { "Environment" },
                description: rsx! { "InputBase provides ids, described-by wiring, and shell state." },
                error: rsx! { "Only lowercase letters are allowed." },
                left_section: rsx! { span { "env" } },
                CustomInputControl {
                    value: value,
                    oninput: move |event: FormEvent| value.set(event.value()),
                }
            }
        }
    }
}

#[component]
fn CustomInputControl(value: Signal<String>, oninput: EventHandler<FormEvent>) -> Element {
    let control = use_input_control_context().expect("InputBase should provide control context");

    rsx! {
        input {
            id: control.id,
            style: "width: 100%; border: 0; background: transparent; outline: none;",
            value: value,
            "aria-describedby": control.described_by.clone(),
            "aria-invalid": control.invalid,
            oninput: move |event| oninput.call(event),
        }
    }
}
