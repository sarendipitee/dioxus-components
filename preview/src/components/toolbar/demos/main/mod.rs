use dioxus::prelude::*;
use dioxus_components::toolbar::*;

#[component]
fn ToggleToolbarButton(
    index: usize,
    is_on: bool,
    on_click: Callback<()>,
    children: Element,
) -> Element {
    rsx! {
        ToolbarButton {
            index,
            on_click,
            aria_pressed: is_on,
            "data-state": if is_on { "on" } else { "off" },
            background: if is_on { "var(--surface-selected)" } else { "" },
            color: if is_on { "var(--fg)" } else { "" },
            {children}
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut is_bold = use_signal(|| false);
    let mut is_italic = use_signal(|| false);
    let mut is_underline = use_signal(|| false);
    let mut text_align = use_signal(|| "left".to_string());
    let mut vertical_toolbar_status = use_signal(|| "No document action selected".to_string());

    rsx! {
        div { class: "dx-toolbar-demo",
            Toolbar {
                aria_label: "Text formatting",
                "data-testid": "formatting-toolbar",
                title: "Formatting controls",
                ToolbarGroup {
                    role: "group",
                    aria_label: "Text style",
                    ToggleToolbarButton { index: 0usize, is_on: is_bold(), on_click: move |_| is_bold.toggle(), "Bold" }
                    ToggleToolbarButton { index: 1usize, is_on: is_italic(), on_click: move |_| is_italic.toggle(), "Italic" }
                    ToggleToolbarButton { index: 2usize, is_on: is_underline(), on_click: move |_| is_underline.toggle(), "Underline" }
                }
                ToolbarSeparator {}
                ToolbarGroup {
                    role: "group",
                    aria_label: "Alignment",
                    ToggleToolbarButton { index: 3usize, is_on: text_align() == "left", on_click: move |_| text_align.set("left".to_string()), "Align Left" }
                    ToggleToolbarButton { index: 4usize, is_on: text_align() == "center", on_click: move |_| text_align.set("center".to_string()), "Align Center" }
                    ToggleToolbarButton { index: 5usize, is_on: text_align() == "right", on_click: move |_| text_align.set("right".to_string()), "Align Right" }
                }
            }
            p {
                max_width: "30rem",
                text_align: "{text_align}",
                font_weight: if is_bold() { "bold" } else { "normal" },
                font_style: if is_italic() { "italic" } else { "normal" },
                text_decoration: if is_underline() { "underline" } else { "none" },
                "The quick brown fox jumps over the lazy dog."
            }
            output { "data-testid": "formatting-status", aria_live: "polite", "Bold: {is_bold()}, Italic: {is_italic()}, Underline: {is_underline()}, Alignment: {text_align}" }

            Toolbar {
                aria_label: "Document actions",
                horizontal: false,
                "data-testid": "vertical-toolbar",
                title: "Document actions",
                ToolbarButton { index: 0usize, on_click: move |_| vertical_toolbar_status.set("Saved".to_string()), "Save" }
                ToolbarButton { index: 1usize, disabled: true, on_click: move |_| vertical_toolbar_status.set("Deleted".to_string()), "Delete" }
                ToolbarButton { index: 2usize, on_click: move |_| vertical_toolbar_status.set("Shared".to_string()), "Share" }
                ToolbarGroup {
                    role: "group",
                    aria_label: "Resources",
                    a { href: "#help", "Help" }
                }
            }
            output { "data-testid": "vertical-toolbar-status", aria_live: "polite", "{vertical_toolbar_status}" }
        }
    }
}
