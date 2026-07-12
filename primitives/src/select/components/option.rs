//! SelectOption and SelectItemIndicator component implementations.

use std::rc::Rc;

use crate::{
    focus::use_focus_control_disabled,
    listbox::{ListboxContext, ListboxItemIndicator},
    selectable::{
        pointer_select_cancel, pointer_select_commit, pointer_select_start, use_selectable_option,
        RcPartialEqValue, SelectableOptionConfig,
    },
};
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectPortalContext};

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps<T: Clone + PartialEq + 'static> {
    /// The value of the option
    #[props(into)]
    pub value: T,

    /// The text value of the option used for typeahead search
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether the option is disabled
    #[props(default)]
    pub disabled: bool,

    /// Optional ID for the option
    #[props(default)]
    pub id: Option<String>,

    /// The index of the option in the list. This is used to define the focus order for keyboard navigation.
    pub index: usize,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes for the option element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the option
    pub children: Element,
}

/// # SelectOption
///
/// An individual selectable option within a [`SelectList`](super::list::SelectList) component. Each option represents
/// a value that can be selected.
///
/// ## Value vs Text Value
///
/// - **`value`**: The programmatic value (e.g., `"apple"`, `"user_123"`) used internally
/// - **`text_value`**: The text value (e.g., `"Apple"`, `"John Doe"`) used for typeahead search and displayed in the [`SelectValue`](super::value::SelectValue)
///
/// This must be used inside a [`SelectList`](super::list::SelectList) component.
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
pub fn SelectOption<T: PartialEq + Clone + 'static>(props: SelectOptionProps<T>) -> Element {
    let index_value = props.index;
    let index = ReadSignal::new(use_memo(move || index_value));
    let id_value = props.id.clone();
    let id_signal = ReadSignal::new(use_memo(move || id_value.clone()));
    let text_value = props.text_value.clone();
    let text_value_signal = ReadSignal::new(use_memo(move || text_value.clone()));
    let disabled_value = props.disabled;
    let disabled_signal = ReadSignal::new(use_memo(move || disabled_value));
    let portal_ctx = try_use_context::<Signal<SelectPortalContext>>();

    if let Some(portal_ctx) = portal_ctx {
        let portal_ctx = portal_ctx();
        let render = use_context::<ListboxContext>().render;
        let generated_id = crate::use_unique_id();
        let fallback_id = crate::use_id_or(generated_id, id_signal);
        let option_value = props.value.clone();
        let selected_value = RcPartialEqValue::new(option_value.clone());
        let option_state = portal_ctx.option_state(index_value);
        let id = option_state
            .map(|option| option.id.clone())
            .unwrap_or_else(|| fallback_id.cloned());
        let disabled = option_state
            .map(|option| option.disabled)
            .unwrap_or(portal_ctx.root_disabled || props.disabled);
        let selected = portal_ctx.is_selected(&selected_value);
        let focused = portal_ctx.focused_index == Some(index_value);
        let mut option_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
        let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);
        let mut selected_signal = use_signal(|| selected);

        use_effect(use_reactive(&selected, move |selected| {
            selected_signal.set(selected);
        }));

        use_context_provider(|| crate::listbox::ListboxOptionContext {
            selected: selected_signal.into(),
        });

        use_effect(use_reactive(&focused, move |focused| {
            if disabled {
                return;
            }
            let Some(option_ref) = option_ref() else {
                return;
            };
            if focused {
                spawn(async move {
                    _ = option_ref.set_focus(true).await;
                });
            }
        }));

        return rsx! {
            if render() {
                div {
                    role: "option",
                    id,
                    tabindex: if focused { "0" } else { "-1" },
                    onmounted: move |event| {
                        option_ref.set(Some(event.data()));
                    },

                    aria_selected: selected,
                    aria_disabled: disabled,
                    aria_label: props.aria_label.clone(),
                    aria_roledescription: props.aria_roledescription.clone(),
                    "data-disabled": disabled,
                    "data-highlighted": focused,

                    onpointerdown: move |event| {
                        pointer_select_start(&event, disabled, down_pos);
                    },
                    onpointerup: move |event| {
                        if pointer_select_commit(&event, disabled, down_pos) {
                            portal_ctx.select_value.call(RcPartialEqValue::new(option_value.clone()));
                        }
                    },
                    onpointercancel: move |_| {
                        pointer_select_cancel(down_pos);
                    },
                    onblur: move |_| {
                        if focused {
                            portal_ctx.blur_focus.call(());
                            portal_ctx.set_open.call(false);
                        }
                    },

                    ..props.attributes,
                    {props.children}
                }
            }
        };
    }

    let mut ctx: SelectContext = use_context();
    let option = use_selectable_option(
        ctx.selectable,
        SelectableOptionConfig {
            id: id_signal,
            index,
            value: props.value.clone(),
            text_value: text_value_signal,
            option_disabled: disabled_signal,
            component_name: "SelectOption",
        },
    );

    let onmounted = use_focus_control_disabled(ctx.selectable.focus_state, index, move || {
        option.disabled.cloned()
    });

    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render() {
            div {
                role: "option",
                id: option.id,
                tabindex: if (option.focused)() { "0" } else { "-1" },
                onmounted,

                aria_selected: (option.selected)(),
                aria_disabled: (option.disabled)(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),
                "data-disabled": (option.disabled)(),
                "data-highlighted": (option.focused)(),

                onpointerdown: move |event| {
                    pointer_select_start(&event, (option.disabled)(), option.down_pos);
                },
                onpointerup: move |event| {
                    if pointer_select_commit(&event, (option.disabled)(), option.down_pos) {
                        ctx.selectable.select_value(RcPartialEqValue::new(option.value.clone()));
                    }
                },
                onpointercancel: move |_| {
                    pointer_select_cancel(option.down_pos);
                },
                onblur: move |_| {
                    if (option.focused)() {
                        ctx.selectable.focus_state.blur();
                        ctx.set_open(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`SelectItemIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    /// The children to render inside the indicator
    pub children: Element,
}

/// # SelectItemIndicator
///
/// The `SelectItemIndicator` component is used to render an indicator for a selected item within a [`SelectList`](super::list::SelectList). The
/// children will only be rendered if the option is selected.
///
/// This must be used inside a [`SelectOption`](SelectOption) component.
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
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    rsx! {
        ListboxItemIndicator {
            {props.children}
        }
    }
}
