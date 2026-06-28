use dioxus::prelude::*;
use dioxus_components::button::Button;
use dioxus_components::popover::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[derive(Clone, Copy, PartialEq)]
struct Placement {
    side: ContentSide,
    align: ContentAlign,
}

impl Placement {
    const fn new(side: ContentSide, align: ContentAlign) -> Self {
        Self { side, align }
    }
}

fn placement_label(p: Placement) -> &'static str {
    match (p.side, p.align) {
        (ContentSide::Top, ContentAlign::Start) => "top-start",
        (ContentSide::Top, ContentAlign::Center) => "top-center",
        (ContentSide::Top, ContentAlign::End) => "top-end",
        (ContentSide::Right, ContentAlign::Start) => "right-start",
        (ContentSide::Right, ContentAlign::Center) => "right-center",
        (ContentSide::Right, ContentAlign::End) => "right-end",
        (ContentSide::Bottom, ContentAlign::Start) => "bottom-start",
        (ContentSide::Bottom, ContentAlign::Center) => "bottom-center",
        (ContentSide::Bottom, ContentAlign::End) => "bottom-end",
        (ContentSide::Left, ContentAlign::Start) => "left-start",
        (ContentSide::Left, ContentAlign::Center) => "left-center",
        (ContentSide::Left, ContentAlign::End) => "left-end",
    }
}

#[component]
pub fn Demo() -> Element {
    let mut active = use_signal(|| Placement::new(ContentSide::Bottom, ContentAlign::Center));
    let mut open = use_signal(|| true);

    let mut set_placement = move |p: Placement| {
        active.set(p);
        open.set(false);
        spawn(async move {
            open.set(true);
        });
    };

    let btn = move |side: ContentSide, align: ContentAlign| {
        let p = Placement::new(side, align);
        let is_active = active() == p;
        rsx! {
            Button {
                r#type: "button",
                "data-style": if is_active { "default" } else { "outline" },
                "data-size": "sm",
                onclick: move |_| set_placement(p),
                "{placement_label(p)}"
            }
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.375rem; align-items: center; user-select: none;",
            // Top row
            div { style: "display: flex; gap: 0.375rem;",
                {btn(ContentSide::Top, ContentAlign::Start)}
                {btn(ContentSide::Top, ContentAlign::Center)}
                {btn(ContentSide::Top, ContentAlign::End)}
            }

            // Middle row: left buttons | center reference | right buttons
            div { style: "display: flex; gap: 0.375rem; align-items: center;",
                div { style: "display: flex; flex-direction: column; gap: 0.375rem;",
                    {btn(ContentSide::Left, ContentAlign::Start)}
                    {btn(ContentSide::Left, ContentAlign::Center)}
                    {btn(ContentSide::Left, ContentAlign::End)}
                }

                div {
                    style: "width: 140px; height: 96px; display: flex; align-items: center; justify-content: center; border: 1px dashed var(--surface-border); border-radius: var(--radius);",
                    PopoverRoot {
                        open: open(),
                        on_open_change: move |v| open.set(v),
                        PopoverTrigger {
                            div {
                                style: "width: 3.5rem; height: 3.5rem; background: var(--surface-muted); border: 1px solid var(--surface-border); border-radius: var(--radius); display: flex; align-items: center; justify-content: center; font-size: 0.6875rem; font-weight: 600; color: var(--surface-muted-fg); cursor: pointer;",
                                "trigger"
                            }
                        }
                        PopoverContent {
                            side: active().side,
                            align: active().align,
                            div {
                                style: "padding: 0.25rem 0.625rem; white-space: nowrap; font-size: 0.8125rem; font-weight: 500;",
                                "{placement_label(active())}"
                            }
                        }
                    }
                }

                div { style: "display: flex; flex-direction: column; gap: 0.375rem;",
                    {btn(ContentSide::Right, ContentAlign::Start)}
                    {btn(ContentSide::Right, ContentAlign::Center)}
                    {btn(ContentSide::Right, ContentAlign::End)}
                }
            }

            // Bottom row
            div { style: "display: flex; gap: 0.375rem;",
                {btn(ContentSide::Bottom, ContentAlign::Start)}
                {btn(ContentSide::Bottom, ContentAlign::Center)}
                {btn(ContentSide::Bottom, ContentAlign::End)}
            }
        }
    }
}
