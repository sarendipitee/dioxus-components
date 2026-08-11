use dioxus::prelude::*;
use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { id: "single-accordion", style: "width: 100%; max-width: 32rem;",
            AccordionItem { index: 0,
                AccordionTrigger { "Account settings" }
                AccordionContent { style: "--accordion-content-padding: 1.5rem;", "Update your profile and account preferences." }
            }
            AccordionItem { index: 1,
                AccordionTrigger { "Billing" }
                AccordionContent { "Review invoices and payment methods." }
            }
            AccordionItem { index: 2, disabled: true,
                AccordionTrigger { "Archived projects" }
                AccordionContent { "Archived project settings are unavailable." }
            }
            AccordionItem { index: 3,
                AccordionTrigger { "Notifications" }
                AccordionContent { "Choose when notifications are delivered." }
            }
        }
    }
}
