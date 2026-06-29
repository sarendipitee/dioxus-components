use dioxus::prelude::*;
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::card::*;
use dioxus_components::input::TextInput;
use dioxus_components::label::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem; width: 100%; max-width: 24rem;",
            Card {
                CardHeader {
                    title: "Login to your account",
                    description: "Enter your email below to login to your account",
                    CardAction {
                        Button { variant: ButtonVariant::Ghost, "Sign Up" }
                    }
                }
                CardContent {
                    form { id: "login-form",
                        div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                            div { style: "display: grid; gap: 0.5rem;",
                                Label { html_for: "email", "Email" }
                                TextInput {
                                    id: "email",
                                    r#type: "email",
                                    placeholder: "m@example.com",
                                }
                            }
                            div { style: "display: grid; gap: 0.5rem;",
                                div { style: "display: flex; align-items: center;",
                                    Label { html_for: "password", "Password" }
                                    a {
                                        href: "#",
                                        style: "margin-left: auto; font-size: var(--text-sm); color: var(--fg-faint); text-decoration: underline; text-underline-offset: 4px;",
                                        "Forgot your password?"
                                    }
                                }
                                TextInput { id: "password", r#type: "password" }
                            }
                        }
                    }
                }
                CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                    Button {
                        r#type: "submit",
                        form: "login-form",
                        style: "width: 100%;",
                        "Login"
                    }
                    Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Login with Google" }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Team workspace" }
                    CardDescription { "Invite collaborators and manage access." }
                }
            }
        }
    }
}
