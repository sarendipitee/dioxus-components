use dioxus::prelude::*;
use crate::component_styles;
use dioxus_icons::lucide::BadgeCheck;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[component_styles("./style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "primary",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }
}

#[component]
pub fn Badge(
    #[props(default = BadgeVariant::Primary)] variant: BadgeVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span {
        class: Styles::dx_badge,
        "data-variant": variant.class(),
    });
    let attributes = merge_attributes(vec![base, attributes]);
    rsx! {
        span { ..attributes,{children} }
    }
}

#[component]
/// Renders the verified badge icon using the surrounding foreground color.
pub fn VerifiedIcon() -> Element {
    rsx! {
        BadgeCheck { size: "12px", stroke: "currentColor" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn BadgeWithStyleProperties() -> Element {
        rsx! {
            Badge {
                background: "var(--success);",
                color: "var(--success-fg)",
                "Verified"
            }
        }
    }

    #[test]
    fn badge_preserves_multiple_style_properties() {
        let mut dom = VirtualDom::new(BadgeWithStyleProperties);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("background:var(--success);"), "{html}");
        assert!(html.contains("color:var(--success-fg);"), "{html}");
    }
}
