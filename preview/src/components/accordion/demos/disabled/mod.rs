use dioxus::prelude::*;
use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { id: "disabled-accordion", style: "width: 100%; max-width: 32rem;", disabled: true,
            AccordionItem { index: 0,
                AccordionTrigger { "Disabled accordion" }
                AccordionContent { "This content cannot be opened." }
            }
        }
    }
}
