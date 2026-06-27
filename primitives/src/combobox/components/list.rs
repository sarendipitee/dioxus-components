//! Combobox options components.

use std::rc::Rc;

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{
    dioxus_attributes::attributes,
    floating::{style_prop, use_position},
    listbox::{use_listbox_container_with_open, use_listbox_id},
    merge_attributes, ContentAlign, ContentSide,
};

/// Props for [`ComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionsProps {
    /// Optional id for the list element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children, typically [`ComboboxOption`](super::option::ComboboxOption)s
    /// and an optional [`ComboboxEmpty`](super::empty::ComboboxEmpty).
    pub children: Element,
}

/// Listbox that contains the visible options.
#[component]
pub fn ComboboxOptions(props: ComboboxOptionsProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());
    let id = use_listbox_id(props.id, ctx.selectable.list_id);
    let listbox = use_listbox_container_with_open(id, ctx.selectable, open);
    let render = listbox.render;

    // Floating-element positioning. The list drops below the combobox target
    // (side=Bottom, align=Start); flip handles upward placement near the bottom
    // viewport edge and shift slides it into view. The reference element is the
    // combobox target registered in the store (search input or custom target). The
    // listbox only mounts while open, so both refs settle on first open.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(
        ctx.store.target_mount_ref(),
        floating_ref,
        ContentSide::Bottom,
        ContentAlign::Start,
    );
    let style = pos.style;
    let is_positioned = pos.is_positioned;
    let resolved_side = pos.side;
    let resolved_align = pos.align;
    let floating_active = pos.floating_active;
    let position = use_memo(move || style_prop(&style.read(), "position"));
    let top = use_memo(move || style_prop(&style.read(), "top"));
    let left = use_memo(move || style_prop(&style.read(), "left"));
    let visibility = use_memo(move || if is_positioned() { "visible" } else { "hidden" });

    // Floating coords merged LAST so they win over forwarded styles; `data-floating`
    // keeps the native `:not([data-floating])` CSS fallback inert on web.
    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        "data-floating": floating_active.then_some("true"),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
        onmounted: move |evt| floating_ref.set(Some(evt.data())),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        if render() {
            div {
                id: listbox.id,
                role: "listbox",
                "data-state": if open() { "open" } else { "closed" },
                onpointerdown: move |event| {
                    event.prevent_default();
                },
                ..attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}

/// Compatibility props alias for [`ComboboxOptions`].
pub type ComboboxListProps = ComboboxOptionsProps;

/// Compatibility alias for [`ComboboxOptions`].
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    rsx! {
        ComboboxOptions {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
