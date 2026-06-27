use dioxus::prelude::*;
use dioxus_components::alert::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; width: 100%; gap: 1rem;",
            Alert {
                title: "System maintenance window",
                description: "Routine infrastructure work is scheduled for tonight from 11:00 PM to 11:30 PM.",
                AlertAction {
                    Button { variant: ButtonVariant::Outline, "View status" }
                }
            }

            Alert {
                variant: AlertVariant::Destructive,
                title: "Payment failed",
                description: "We could not process the last invoice. Update the billing method to avoid service interruption.",
                AlertAction {
                    Button { variant: ButtonVariant::Destructive, "Fix billing" }
                }
            }

            Alert {
                variant: AlertVariant::Info,
                title: "Profile change pending review",
                description: "Your updated organization details are saved and waiting for approval from an administrator.",
            }

            Alert {
                variant: AlertVariant::Success,
                title: "Backup completed",
                description: "A new encrypted restore point is available and has been replicated to the secondary region.",
            }
        }
    }
}
