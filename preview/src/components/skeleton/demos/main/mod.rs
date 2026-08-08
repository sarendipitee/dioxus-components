use dioxus_components::skeleton::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 2rem;",
            SkeletonInfoDemo {}
            SkeletonCardDemo {}
        }
    }
}

#[component]
fn SkeletonInfoDemo() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            button {
                id: "skeleton-toggle",
                r#type: "button",
                onclick: move |_| loading.set(!loading()),
                if loading() { "Show loaded profile" } else { "Show loading profile" }
            }
            output { id: "skeleton-status", aria_live: "polite",
                if loading() { "Profile is loading" } else { "Profile loaded" }
            }
            div {
                id: "skeleton-loading-region",
                aria_label: "Profile loading preview",
                role: "region",
                aria_busy: loading(),
                style: "display: flex; align-items: center; gap: 1rem;",
                if loading() {
                    Skeleton {
                        id: "skeleton-avatar",
                        aria_hidden: "true",
                        title: "Circular avatar placeholder",
                        "data-shape": "circle",
                        style: "width: 3rem; height: 3rem; border-radius: 50%;"
                    }
                    div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                        Skeleton {
                            id: "skeleton-primary-line",
                            aria_hidden: "true",
                            style: "width: 11.625rem; height: 1rem;"
                        }
                        Skeleton {
                            id: "skeleton-secondary-line",
                            aria_hidden: "true",
                            style: "width: 8.5rem; height: 1rem;"
                        }
                    }
                } else {
                    div {
                        strong { "Ada Lovelace" }
                        div { "Computing pioneer" }
                    }
                }
            }
        }
    }
}

#[component]
fn SkeletonCardDemo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Skeleton { aria_hidden: "true", style: "width: 15rem; height: 8rem; border-radius: 0.75rem;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { aria_hidden: "true", style: "width: 15.625rem; height: 1rem;" }
                Skeleton { aria_hidden: "true", style: "width: 12.5rem; height: 1rem;" }
            }
        }
    }
}
