use dioxus::prelude::*;
use dioxus_components::alert::*;
use dioxus_components::button::{Button, ButtonVariant};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; width: 100%; gap: 1rem;",
            Alert {
                AlertIcon { "*" }
                AlertTitle { "System maintenance window" }
                AlertDescription {
                    "Routine infrastructure work is scheduled for tonight from 11:00 PM to 11:30 PM."
                }
                AlertAction {
                    Button { variant: ButtonVariant::Outline, "View status" }
                }
            }

            Alert { variant: AlertVariant::Destructive,
                AlertIcon { "!" }
                AlertTitle { "Payment failed" }
                AlertDescription {
                    "We could not process the last invoice. Update the billing method to avoid service interruption."
                }
                AlertAction {
                    Button { variant: ButtonVariant::Destructive, "Fix billing" }
                }
            }

            Alert { variant: AlertVariant::Info,
                AlertIcon { "i" }
                AlertTitle { "Profile change pending review" }
                AlertDescription {
                    "Your updated organization details are saved and waiting for approval from an administrator."
                }
            }

            Alert { variant: AlertVariant::Success,
                AlertIcon { "+" }
                AlertTitle { "Backup completed" }
                AlertDescription {
                    "A new encrypted restore point is available and has been replicated to the secondary region."
                }
            }
        }
    }
}
