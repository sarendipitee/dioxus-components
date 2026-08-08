use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus::prelude::*;
#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "2rem",
            Accordion {
                id: "single-accordion",
                AccordionItem { index: 0,
                    AccordionTrigger { "Account settings" }
                    AccordionContent { "Update your profile and account preferences." }
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

            Accordion {
                id: "multiple-accordion",
                allow_multiple_open: true,
                collapsible: false,
                AccordionItem { index: 0, default_open: true,
                    AccordionTrigger { "Shipping details" }
                    AccordionContent { "Orders are shipped within two business days." }
                }
                AccordionItem { index: 1,
                    AccordionTrigger { "Returns policy" }
                    AccordionContent { "Returns are accepted within thirty days." }
                }
            }

            Accordion { id: "horizontal-accordion", horizontal: true,
                AccordionItem { index: 0,
                    AccordionTrigger { "Overview" }
                    AccordionContent { "Overview content." }
                }
                AccordionItem { index: 1,
                    AccordionTrigger { "Activity" }
                    AccordionContent { "Activity content." }
                }
            }

            Accordion { id: "disabled-accordion", disabled: true,
                AccordionItem { index: 0,
                    AccordionTrigger { "Disabled accordion" }
                    AccordionContent { "This content cannot be opened." }
                }
            }
        }
    }
}
