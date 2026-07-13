//! Combobox option components.

use dioxus::prelude::*;

use super::super::{
    context::{
        ComboboxContext, ComboboxPortalContext, ComboboxPortalOptionRegistration,
    },
    hook::ComboboxDropdownEventSource,
};
use crate::{
    listbox::{ListboxContext, ListboxItemIndicator},
    selectable::{
        pointer_select_cancel, pointer_select_commit, pointer_select_start, use_selectable_option,
        RcPartialEqValue, SelectableOptionConfig,
    },
    selection::{option_text_value, OptionState},
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
    let portal_ctx = try_use_context::<Signal<ComboboxPortalContext>>();

    if let Some(portal_ctx) = portal_ctx {
        // Read from portal-local signal so navigation/filter snapshots rerender
        // options after their root-side state changes.
        let portal_ctx = portal_ctx();
        let listbox = use_context::<ListboxContext>();
        let render = listbox.render;
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
            let listbox_id = listbox.id.clone();
            let id = use_memo(move || {
                id_signal()
                    .unwrap_or_else(|| format!("{listbox_id}-option-{}", index()))
            });
            let disabled = disabled_snapshot;
            let registration_value = props.value.clone();
            let option_value = props.value.clone();
            let text_value_override = props.text_value.clone();
            let text_value = use_memo(move || {
                option_text_value(
                    &option_value,
                    text_value_override.clone(),
                    "ComboboxOption",
                )
            });
            let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);

            use_effect_with_cleanup({
                let register_option = portal_ctx.register_option;
                let unregister_option = portal_ctx.unregister_option;
                move || {
                    let registration = ComboboxPortalOptionRegistration {
                        option: OptionState {
                            tab_index: index(),
                            value: RcPartialEqValue::new(registration_value.clone()),
                            text_value: text_value.cloned(),
                            id: id.cloned(),
                            disabled,
                        },
                        visible,
                        selected,
                    };
                    register_option.call(registration.clone());
                    move || unregister_option.call(registration)
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

                        onmouseenter: move |_: Event<MouseData>| {
                            if !disabled {
                                portal_ctx.hover_option.call(index_value);
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

                    onmouseenter: move |_: Event<MouseData>| {
                        if !disabled {
                            portal_ctx.hover_option.call(index_value);
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
        let selectable = ctx.selectable;
        move || {
            let id_value = id.cloned();
            store.register_option(id_value.clone(), index(), disabled(), visible(), selected());
            if let Some(initial) = store
                .resolve_pending_initial_selection_at(index())
                .or_else(|| store.retry_pending_initial_selection())
            {
                let mut focus_state = selectable.focus_state;
                focus_state.set_focus(Some(initial.index));
            }
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
