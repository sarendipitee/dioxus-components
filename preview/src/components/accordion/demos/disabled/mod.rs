use dioxus::prelude::*;
use dioxus_components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { id: "disabled-accordion", disabled: true,
            AccordionItem { index: 0,
                AccordionTrigger { "Disabled accordion" }
                AccordionContent { "This content cannot be opened." }
            }
        }
    }
}
