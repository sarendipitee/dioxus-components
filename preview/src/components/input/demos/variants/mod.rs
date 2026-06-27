use dioxus::prelude::*;
use dioxus_components::input::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            style: "display: grid; gap: 1.5rem; max-width: 24rem;",
            div {
                style: "display: grid; gap: 0.75rem;",
                Input {
                    variant: InputVariant::Default,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Default variant",
                    }
                }
                Input {
                    variant: InputVariant::Filled,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Filled variant",
                    }
                }
                Input {
                    variant: InputVariant::Unstyled,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Unstyled variant",
                    }
                }
            }
            div {
                style: "display: grid; gap: 0.75rem;",
                Input {
                    size: InputSize::Sm,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Small",
                    }
                }
                Input {
                    size: InputSize::Md,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Medium",
                    }
                }
                Input {
                    size: InputSize::Lg,
                    input {
                        style: "width: 100%; border: 0; background: transparent; outline: none;",
                        placeholder: "Large",
                    }
                }
            }
        }
    }
}
