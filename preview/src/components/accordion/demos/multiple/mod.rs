use dioxus::prelude::*;
use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { id: "multiple-accordion", style: "width: 100%; max-width: 32rem;", allow_multiple_open: true, collapsible: false,
            AccordionItem { index: 0, default_open: true,
                AccordionTrigger { "Shipping details" }
                AccordionContent { "Orders are shipped within two business days." }
            }
            AccordionItem { index: 1,
                AccordionTrigger { "Returns policy" }
                AccordionContent { "Returns are accepted within thirty days." }
            }
        }
    }
}
