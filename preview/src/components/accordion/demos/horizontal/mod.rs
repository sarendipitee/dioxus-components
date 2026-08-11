use dioxus::prelude::*;
use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
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
    }
}
