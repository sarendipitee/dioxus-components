use dioxus::prelude::*;
use dioxus_components::accordion::{
    Accordion, AccordionChevronPosition, AccordionContent, AccordionItem, AccordionTrigger,
};
use dioxus_icons::lucide::{CircleMinus, CirclePlus, Info, ShieldCheck};

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.5rem; width: min(100%, 36rem);",
            section {
                h3 { "Icons and custom chevrons" }
                Accordion { id: "custom-slots-accordion", chevron_icon_size: 20_u32,
                    AccordionItem { index: 0,
                        AccordionTrigger {
                            icon: rsx! { Info { size: "18", "data-testid": "leading-icon" } },
                            chevron: rsx! { CirclePlus { size: "20", "data-testid": "custom-chevron" } },
                            "Can I replace trigger visuals?"
                        }
                        AccordionContent {
                            "Use the icon slot for leading content and the chevron slot for a custom expand indicator."
                        }
                    }
                    AccordionItem { index: 1,
                        AccordionTrigger {
                            icon: rsx! { ShieldCheck { size: "18" } },
                            chevron: rsx! { CircleMinus { size: "20" } },
                            "Can each item use a different indicator?"
                        }
                        AccordionContent {
                            "Each trigger owns its icon and chevron content."
                        }
                    }
                }
            }

            section {
                h3 { "Chevron placement and rotation" }
                Accordion {
                    id: "left-chevron-accordion",
                    chevron_position: AccordionChevronPosition::Left,
                    disable_chevron_rotation: true,
                    AccordionItem { index: 0,
                        AccordionTrigger { "Left aligned without rotation" }
                        AccordionContent {
                            "Position and rotation behavior are configured once on the accordion."
                        }
                    }
                }
            }
        }
    }
}
