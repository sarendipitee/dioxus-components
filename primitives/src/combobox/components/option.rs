//! Combobox option components.

use dioxus::prelude::*;

use super::super::{
    context::{ComboboxContext, ComboboxPortalContext},
    hook::ComboboxDropdownEventSource,
};
use crate::{
    listbox::{ListboxContext, ListboxItemIndicator},
    selectable::{
        pointer_select_cancel, pointer_select_commit, pointer_select_start, use_selectable_option,
        use_selectable_option_registration, RcPartialEqValue, SelectableOptionConfig,
    },
    selection::option_text_value,
    use_effect_with_cleanup,
};

/// Props for [`ComboboxOption`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionProps<T: Clone + PartialEq + 'static> {
    /// The value carried by this option.
    #[props(into)]
    pub value: T,

    /// Display/searchable text. Required for non-string types.
    #[props(default)]
    pub text_value: Option<String>,

    /// Whether the option is disabled.
    #[props(default)]
    pub disabled: bool,

    /// Optional id for the option element.
    #[props(default)]
    pub id: Option<String>,

    /// Registration order used for keyboard navigation.
    pub index: usize,

    /// Optional aria-label.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional aria-roledescription.
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the option.
    pub children: Element,
}

/// A selectable option inside a [`ComboboxList`](super::list::ComboboxList).
#[component]
pub fn ComboboxOption<T: PartialEq + Clone + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let index_value = props.index;
    let index = ReadSignal::new(use_memo(move || index_value));
    let id_value = props.id.clone();
    let id_signal = ReadSignal::new(use_memo(move || id_value.clone()));
    let text_value = props.text_value.clone();
    let text_value_signal = ReadSignal::new(use_memo(move || text_value.clone()));
    let disabled_value = props.disabled;
    let disabled_signal = ReadSignal::new(use_memo(move || disabled_value));
    let portal_ctx = try_use_context::<ComboboxPortalContext>();

    if let Some(portal_ctx) = portal_ctx {
        let render = use_context::<ListboxContext>().render;
        let visible = portal_ctx.is_visible(index_value);
        let selected_value = RcPartialEqValue::new(props.value.clone());
        let selected = portal_ctx.is_selected(&selected_value);
        let highlighted = portal_ctx.focused_index == Some(index_value)
            || portal_ctx.highlighted_index == Some(index_value);
        let option_state = portal_ctx.option_state(index_value);
        let disabled_snapshot = option_state
            .map(|option| option.disabled)
            .unwrap_or(portal_ctx.root_disabled || props.disabled);
        let mut selected_signal = use_signal(|| selected);

        use_effect(use_reactive(&selected, move |selected| {
            selected_signal.set(selected);
        }));

        use_context_provider(|| crate::listbox::ListboxOptionContext {
            selected: selected_signal.into(),
        });

        if portal_ctx.register_options {
            let option = use_selectable_option_registration(
                portal_ctx.selectable,
                SelectableOptionConfig {
                    id: id_signal,
                    index,
                    value: props.value.clone(),
                    text_value: text_value_signal,
                    option_disabled: disabled_signal,
                    component_name: "ComboboxOption",
                },
                portal_ctx.root_disabled,
            );
            let disabled = (option.disabled)();
            let id = option.id;
            let down_pos = option.down_pos;

            use_effect_with_cleanup({
                let store = portal_ctx.store;
                let id = option.id;
                let disabled = option.disabled;
                move || {
                    let id_value = id.cloned();
                    store.register_option(id_value.clone(), index(), disabled(), visible, selected);
                    move || store.unregister_option(&id_value)
                }
            });

            return rsx! {
                if render() && visible {
                    div {
                        role: "option",
                        id,

                        aria_selected: selected,
                        aria_disabled: disabled,
                        aria_label: props.aria_label.clone(),
                        aria_roledescription: props.aria_roledescription.clone(),

                        "data-highlighted": highlighted,
                        "data-disabled": disabled,
                        "data-selected": selected,

                        onmouseenter: move |_| {
                            if !disabled {
                                let mut focus_state = portal_ctx.selectable.focus_state;
                                focus_state.set_focus(Some(index_value));
                                portal_ctx.store.select_option(index_value);
                            }
                        },
                        onpointerdown: move |event| {
                            pointer_select_start(&event, disabled, down_pos);
                        },
                        onpointerup: move |event| {
                            if pointer_select_commit(&event, disabled, down_pos) {
                                portal_ctx.submit_index_from_mouse.call(index_value);
                            }
                        },
                        onpointercancel: move |_| {
                            pointer_select_cancel(down_pos);
                        },

                        ..props.attributes,
                        {props.children}
                    }
                }
            };
        }

        let generated_id = crate::use_unique_id();
        let fallback_id = crate::use_id_or(generated_id, id_signal);
        let id = option_state
            .map(|option| option.id.clone())
            .unwrap_or_else(|| fallback_id.cloned());
        let disabled = disabled_snapshot;
        let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);

        return rsx! {
            if render() && visible {
                div {
                    role: "option",
                    id,

                    aria_selected: selected,
                    aria_disabled: disabled,
                    aria_label: props.aria_label.clone(),
                    aria_roledescription: props.aria_roledescription.clone(),

                    "data-highlighted": highlighted,
                    "data-disabled": disabled,
                    "data-selected": selected,

                    onmouseenter: move |_| {
                        if !disabled {
                            let mut focus_state = portal_ctx.selectable.focus_state;
                            focus_state.set_focus(Some(index_value));
                            portal_ctx.store.select_option(index_value);
                        }
                    },
                    onpointerdown: move |event| {
                        pointer_select_start(&event, disabled, down_pos);
                    },
                    onpointerup: move |event| {
                        if pointer_select_commit(&event, disabled, down_pos) {
                            portal_ctx.submit_index_from_mouse.call(index_value);
                        }
                    },
                    onpointercancel: move |_| {
                        pointer_select_cancel(down_pos);
                    },

                    ..props.attributes,
                    {props.children}
                }
            }
        };
    }

    let mut ctx: ComboboxContext = use_context();
    let text_option_value = props.value.clone();
    let text_value_option = props.text_value.clone();
    let text_value = use_memo(move || {
        option_text_value(
            &text_option_value,
            text_value_option.clone(),
            "ComboboxOption",
        )
    });
    let visible = move || ctx.is_visible_text(index(), text_value.cloned());
    let option = use_selectable_option(
        ctx.selectable,
        SelectableOptionConfig {
            id: id_signal,
            index,
            value: props.value.clone(),
            text_value: text_value_signal,
            option_disabled: disabled_signal,
            component_name: "ComboboxOption",
        },
    );
    use_effect_with_cleanup({
        let store = ctx.store;
        let id = option.id;
        let disabled = option.disabled;
        let selected = option.selected;
        move || {
            let id_value = id.cloned();
            store.register_option(id_value.clone(), index(), disabled(), visible(), selected());
            move || store.unregister_option(&id_value)
        }
    });

    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render() && visible() {
            div {
                role: "option",
                id: option.id,

                aria_selected: (option.selected)(),
                aria_disabled: (option.disabled)(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),

                "data-highlighted": (option.focused)(),
                "data-disabled": (option.disabled)(),
                "data-selected": (option.selected)(),

                onmouseenter: move |_| {
                    if !(option.disabled)() {
                        ctx.selectable.focus_state.set_focus(Some((option.index)()));
                        ctx.store.select_option((option.index)());
                    }
                },
                onpointerdown: move |event| {
                    pointer_select_start(&event, (option.disabled)(), option.down_pos);
                },
                onpointerup: move |event| {
                    if pointer_select_commit(&event, (option.disabled)(), option.down_pos) {
                        ctx.submit_index((option.index)(), ComboboxDropdownEventSource::Mouse);
                    }
                },
                onpointercancel: move |_| {
                    pointer_select_cancel(option.down_pos);
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Props for [`ComboboxItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemIndicatorProps {
    /// Children rendered only when the parent option is selected.
    pub children: Element,
}

/// Renders its children when the parent option is selected.
#[component]
pub fn ComboboxItemIndicator(props: ComboboxItemIndicatorProps) -> Element {
    rsx! {
        ListboxItemIndicator {
            {props.children}
        }
    }
}
