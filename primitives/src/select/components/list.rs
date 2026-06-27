//! SelectList component implementation.

use crate::{
    dioxus_attributes::attributes, floating::{style_prop, use_position},
    listbox::use_listbox_container, merge_attributes, use_effect, ContentAlign, ContentSide,
};
use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectList`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the list
    pub children: Element,
}

/// # SelectList
///
/// The dropdown list container for the [`Select`](super::select::Select) component that contains the
/// [`SelectOption`](super::option::SelectOption)s. The list will only be rendered when the select is open.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue { placeholder: "Select a fruit..." }
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let mut ctx = use_context::<SelectContext>();

    let open = ctx.selectable.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.selectable.focus_state.any_focused();

    // Floating-element positioning. The list drops below the trigger (side=Bottom,
    // align=Start); flip handles upward placement near the bottom viewport edge and
    // shift slides it back into view. `use_position` is called unconditionally; the
    // listbox only mounts while open so both refs settle on first open.
    let pos = use_position(
        ctx.trigger_ref,
        listbox_ref,
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

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();

        // Learn from keyboard events for adaptive matching
        if let Key::Character(actual_char) = &key {
            if let Some(actual_char) = actual_char.chars().next() {
                ctx.learn_from_keyboard_event(&code.to_string(), actual_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.selectable.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                ctx.set_open(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    let listbox = use_listbox_container(props.id, ctx.selectable);
    let render = listbox.render;

    // Floating coordinates are split into individual `style:` props (position/top/left)
    // and merged LAST so the computed coordinates win over any user-forwarded style,
    // while leaving every other forwarded style intact (see popover for the rationale).
    // The `data-floating` marker keeps the native `:not([data-floating])` CSS fallback
    // inert on web.
    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        "data-floating": floating_active.then_some("true"),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        if render() {
            div {
                id: listbox.id,
                role: "listbox",
                tabindex: if focused() { "0" } else { "-1" },
                aria_multiselectable: ctx.multi(),

                // Data attributes
                "data-state": if open() { "open" } else { "closed" },

                onmounted: move |evt| listbox_ref.set(Some(evt.data())),
                onkeydown,
                onblur: move |_| {
                    if focused() {
                        ctx.set_open(false);
                    }
                },

                ..attributes,
                {props.children}
            }
        } else {
            // If not rendering, return children directly so we can populate the selected list, but they should choose to not render themselves
            {props.children}
        }
    }
}

