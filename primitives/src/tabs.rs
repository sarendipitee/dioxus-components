//! Defines the [`Tabs`] component and its sub-components.

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    use_id_or, use_unique_id,
};
use dioxus::prelude::*;

/// Supported tab-list orientations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
    /// Render the tab list horizontally.
    #[default]
    Horizontal,
    /// Render the tab list vertically.
    Vertical,
}

impl TabsOrientation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Controls whether focused tabs activate automatically or manually.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsActivationMode {
    /// Activate the tab as soon as it receives focus.
    #[default]
    Automatic,
    /// Keep focus movement separate from activation until explicit confirmation.
    Manual,
}

/// Horizontal justification for the tab list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsJustify {
    /// Align tabs to the start.
    #[default]
    Start,
    /// Center tabs in the available space.
    Center,
    /// Align tabs to the end.
    End,
    /// Distribute tabs with space between them.
    SpaceBetween,
}

impl TabsJustify {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::SpaceBetween => "space-between",
        }
    }
}

/// Placement for vertical tab lists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsPlacement {
    /// Render the list before the panels.
    #[default]
    Start,
    /// Render the list after the panels.
    End,
}

impl TabsPlacement {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

impl TabsActivationMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

fn use_controllable_tab_value(
    prop: Option<ReadSignal<Option<String>>>,
    default: Option<String>,
    on_change: Callback<Option<String>>,
) -> (Memo<Option<String>>, Callback<Option<String>>) {
    let mut internal_value = use_signal(|| {
        prop.as_ref()
            .and_then(|value| value.cloned())
            .or(default.clone())
    });
    let value = use_memo(move || match prop {
        Some(value) => value.cloned(),
        None => internal_value(),
    });

    let set_value = use_callback(move |next: Option<String>| {
        internal_value.set(next.clone());
        on_change.call(next);
    });

    (value, set_value)
}

#[derive(Clone, Copy)]
struct TabsContext {
    value: Memo<Option<String>>,
    set_value: Callback<Option<String>>,
    disabled: ReadSignal<bool>,
    focus: FocusState,
    orientation: Memo<TabsOrientation>,
    activation_mode: TabsActivationMode,
    allow_tab_deactivation: bool,
    keep_mounted: bool,
    roving_loop: ReadSignal<bool>,
    tab_values: Signal<Vec<String>>,
    tab_trigger_ids: Signal<Vec<String>>,
    tab_content_ids: Signal<Vec<String>>,
}

impl TabsContext {
    fn is_selected(&self, value: &str) -> bool {
        (self.value)().as_deref() == Some(value)
    }

    fn set_focus(&mut self, index: Option<usize>) {
        self.focus.set_focus(index);
    }

    fn activate(&self, value: String) {
        if self.is_selected(&value) {
            if self.allow_tab_deactivation {
                self.set_value.call(None);
            }
        } else {
            self.set_value.call(Some(value));
        }
    }

    fn activate_if_automatic(&self, value: String) {
        if self.activation_mode == TabsActivationMode::Automatic && !self.is_selected(&value) {
            self.set_value.call(Some(value));
        }
    }

    fn activate_focused_tab_if_automatic(&self) {
        let Some(index) = self.focus.current_focus() else {
            return;
        };
        let Some(value) = (self.tab_values)().get(index).cloned() else {
            return;
        };
        self.activate_if_automatic(value);
    }
}

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The controlled value of the active tab. If supplied, `None` means no tab is active.
    #[props(default)]
    pub value: Option<ReadSignal<Option<String>>>,

    /// The default active tab value when uncontrolled.
    #[props(default, into)]
    pub default_value: Option<String>,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<Option<String>>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Explicit orientation for the tabs.
    #[props(default)]
    pub orientation: Option<TabsOrientation>,

    /// Backwards-compatible orientation fallback.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub horizontal: ReadSignal<bool>,

    /// Whether arrow-key focus should automatically activate the focused tab.
    #[props(default)]
    pub activation_mode: TabsActivationMode,

    /// Whether pressing an already-active tab can clear the active selection.
    #[props(default = false)]
    pub allow_tab_deactivation: bool,

    /// Whether inactive tab panels should remain mounted in the DOM.
    #[props(default = true)]
    pub keep_mounted: bool,

    /// Whether the tab list should stretch its triggers across the available width.
    #[props(default = false)]
    pub grow: bool,

    /// Horizontal justification for the tab list.
    #[props(default)]
    pub justify: TabsJustify,

    /// Placement for vertical tab lists.
    #[props(default)]
    pub placement: TabsPlacement,

    /// Whether the tab list should render after the panels.
    #[props(default = false)]
    pub inverted: bool,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// # Tabs
///
/// The `Tabs` component creates a tabbed interface that allows users to switch between different panels
/// of content. The [`TabTrigger`] component is used to switch between the different [`TabContent`]s.
///
/// ## Styling
///
/// The [`Tabs`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the tabs. Values are `horizontal` or `vertical`.
/// - `data-disabled`: Indicates if the tabs are disabled. Values are `true` or `false`.
/// - `data-activation-mode`: Indicates whether focused tabs activate automatically or manually.
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let (value, set_value) =
        use_controllable_tab_value(props.value, props.default_value, props.on_value_change);
    let orientation = use_memo(use_reactive(
        (&props.orientation, &props.horizontal),
        move |(orientation, horizontal)| {
            orientation.unwrap_or_else(|| {
                if horizontal() {
                    TabsOrientation::Horizontal
                } else {
                    TabsOrientation::Vertical
                }
            })
        },
    ));

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| TabsContext {
        value,
        set_value,
        disabled: props.disabled,
        focus,
        orientation,
        activation_mode: props.activation_mode,
        allow_tab_deactivation: props.allow_tab_deactivation,
        keep_mounted: props.keep_mounted,
        roving_loop: props.roving_loop,
        tab_values: Signal::new(Vec::new()),
        tab_trigger_ids: Signal::new(Vec::new()),
        tab_content_ids: Signal::new(Vec::new()),
    });

    rsx! {
        div {
            "data-orientation": orientation().as_str(),
            "data-disabled": (props.disabled)(),
            "data-activation-mode": props.activation_mode.as_str(),
            "data-has-value": value().is_some().to_string(),
            "data-grow": props.grow.to_string(),
            "data-justify": props.justify.as_str(),
            "data-placement": props.placement.as_str(),
            "data-inverted": props.inverted.to_string(),

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::ScopeId;
    use std::cell::RefCell;

    thread_local! {
        static CONTROLLED_TABS_INITIAL_NONE_VALUE: RefCell<Option<String>> = const { RefCell::new(None) };
        static CONTROLLED_TABS_RESET_VALUE: RefCell<Option<String>> = const { RefCell::new(None) };
        static CONTROLLED_TABS_ORIENTATION_VALUE: RefCell<TabsOrientation> = const { RefCell::new(TabsOrientation::Horizontal) };
        static CONTROLLED_TABS_HORIZONTAL_VALUE: RefCell<bool> = const { RefCell::new(true) };
    }

    #[component]
    fn ControlledTabsInitialNoneHarness() -> Element {
        let controlled = CONTROLLED_TABS_INITIAL_NONE_VALUE.with(|value| value.borrow().clone());

        rsx! {
            Tabs {
                value: Some(ReadSignal::new(Signal::new(controlled))),
                default_value: Some("overview".to_string()),
                TabList {
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                }
                TabContent { value: "overview", index: 0usize, "Overview content" }
                TabContent { value: "metrics", index: 1usize, "Metrics content" }
            }
        }
    }

    #[test]
    fn controlled_none_stays_unselected_on_initial_render() {
        CONTROLLED_TABS_INITIAL_NONE_VALUE.with(|value| {
            *value.borrow_mut() = None;
        });

        let mut dom = VirtualDom::new(ControlledTabsInitialNoneHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("data-has-value=\"false\""));
        assert!(!html.contains("data-state=\"active\""));
    }

    #[component]
    fn UncontrolledTabsLiteralDefaultHarness() -> Element {
        rsx! {
            Tabs {
                default_value: "demo",
                TabList {
                    TabTrigger { value: "demo", index: 0usize, "Demo" }
                    TabTrigger { value: "code", index: 1usize, "Code" }
                }
                TabContent { value: "demo", index: 0usize, "Demo content" }
                TabContent { value: "code", index: 1usize, "Code content" }
            }
        }
    }

    #[test]
    fn uncontrolled_default_value_accepts_string_literal() {
        let mut dom = VirtualDom::new(UncontrolledTabsLiteralDefaultHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Demo content"));
        assert!(html.contains("data-state=\"active\""));
        assert!(html.contains("data-has-value=\"true\""));
    }

    #[component]
    fn UncontrolledTabsOwnedDefaultHarness() -> Element {
        let owned_default_value = String::from("demo");

        rsx! {
            Tabs {
                default_value: owned_default_value,
                TabList {
                    TabTrigger { value: "demo", index: 0usize, "Demo" }
                    TabTrigger { value: "code", index: 1usize, "Code" }
                }
                TabContent { value: "demo", index: 0usize, "Demo content" }
                TabContent { value: "code", index: 1usize, "Code content" }
            }
        }
    }

    #[test]
    fn uncontrolled_default_value_accepts_owned_string() {
        let mut dom = VirtualDom::new(UncontrolledTabsOwnedDefaultHarness);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Demo content"));
        assert!(html.contains("data-state=\"active\""));
        assert!(html.contains("data-has-value=\"true\""));
    }

    #[component]
    fn ControlledTabsResetHarness() -> Element {
        let controlled = CONTROLLED_TABS_RESET_VALUE.with(|value| value.borrow().clone());

        rsx! {
            Tabs {
                value: Some(ReadSignal::new(Signal::new(controlled))),
                default_value: Some("overview".to_string()),
                TabList {
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                }
                TabContent { value: "overview", index: 0usize, "Overview content" }
                TabContent { value: "metrics", index: 1usize, "Metrics content" }
            }
        }
    }

    #[test]
    fn controlled_parent_reset_to_none_clears_previous_internal_selection() {
        CONTROLLED_TABS_RESET_VALUE.with(|value| {
            *value.borrow_mut() = Some("overview".to_string());
        });

        let mut dom = VirtualDom::new(ControlledTabsResetHarness);
        dom.rebuild_in_place();
        let selected_html = dioxus_ssr::render(&dom);
        assert!(selected_html.contains("data-has-value=\"true\""));
        assert!(selected_html.contains("data-state=\"active\""));

        CONTROLLED_TABS_RESET_VALUE.with(|value| {
            *value.borrow_mut() = None;
        });
        dom.mark_dirty(ScopeId::ROOT);
        dom.rebuild_in_place();
        let reset_html = dioxus_ssr::render(&dom);

        assert!(reset_html.contains("data-has-value=\"false\""));
        assert!(!reset_html.contains("data-state=\"active\""));
    }

    #[component]
    fn ControlledTabsOrientationHarness() -> Element {
        let orientation = CONTROLLED_TABS_ORIENTATION_VALUE.with(|value| *value.borrow());

        rsx! {
            Tabs {
                orientation: Some(orientation),
                TabList {
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                }
                TabContent { value: "overview", index: 0usize, "Overview content" }
                TabContent { value: "metrics", index: 1usize, "Metrics content" }
            }
        }
    }

    #[test]
    fn controlled_orientation_updates_after_parent_rerender() {
        CONTROLLED_TABS_ORIENTATION_VALUE.with(|value| {
            *value.borrow_mut() = TabsOrientation::Horizontal;
        });

        let mut dom = VirtualDom::new(ControlledTabsOrientationHarness);
        dom.rebuild_in_place();
        let initial_html = dioxus_ssr::render(&dom);
        assert!(initial_html.contains("data-orientation=\"horizontal\""));
        assert!(initial_html.contains("aria-orientation=\"horizontal\""));

        CONTROLLED_TABS_ORIENTATION_VALUE.with(|value| {
            *value.borrow_mut() = TabsOrientation::Vertical;
        });
        dom.mark_dirty(ScopeId::ROOT);
        dom.rebuild_in_place();
        let updated_html = dioxus_ssr::render(&dom);

        assert!(updated_html.contains("data-orientation=\"vertical\""));
        assert!(updated_html.contains("aria-orientation=\"vertical\""));
    }

    #[component]
    fn ControlledTabsHorizontalHarness() -> Element {
        let horizontal = CONTROLLED_TABS_HORIZONTAL_VALUE.with(|value| *value.borrow());

        rsx! {
            Tabs {
                horizontal: ReadSignal::new(Signal::new(horizontal)),
                TabList {
                    TabTrigger { value: "overview", index: 0usize, "Overview" }
                    TabTrigger { value: "metrics", index: 1usize, "Metrics" }
                }
                TabContent { value: "overview", index: 0usize, "Overview content" }
                TabContent { value: "metrics", index: 1usize, "Metrics content" }
            }
        }
    }

    #[test]
    fn legacy_horizontal_prop_updates_orientation_after_parent_rerender() {
        CONTROLLED_TABS_HORIZONTAL_VALUE.with(|value| {
            *value.borrow_mut() = true;
        });

        let mut dom = VirtualDom::new(ControlledTabsHorizontalHarness);
        dom.rebuild_in_place();
        let initial_html = dioxus_ssr::render(&dom);
        assert!(initial_html.contains("data-orientation=\"horizontal\""));
        assert!(initial_html.contains("aria-orientation=\"horizontal\""));

        CONTROLLED_TABS_HORIZONTAL_VALUE.with(|value| {
            *value.borrow_mut() = false;
        });
        dom.mark_dirty(ScopeId::ROOT);
        dom.rebuild_in_place();
        let updated_html = dioxus_ssr::render(&dom);

        assert!(updated_html.contains("data-orientation=\"vertical\""));
        assert!(updated_html.contains("aria-orientation=\"vertical\""));
    }
}

/// The props for the [`TabList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabListProps {
    /// Whether the list should support overflow scrolling.
    #[props(default = false)]
    pub scrollable: bool,

    /// Additional attributes to apply to the tab list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab list component.
    pub children: Element,
}

/// # TabList
///
/// The `TabList` component contains a list of [`TabTrigger`] components that allow users to switch between different tabs.
#[component]
pub fn TabList(props: TabListProps) -> Element {
    let ctx: TabsContext = use_context();

    rsx! {
        div {
            role: "tablist",
            aria_orientation: (ctx.orientation)().as_str(),
            "data-scrollable": props.scrollable.to_string(),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`TabTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct TabTriggerProps {
    /// The value of the tab trigger, which is used to identify the corresponding tab content.
    #[props(into)]
    pub value: String,

    /// The index of the tab trigger. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the tab trigger is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The ID of the tab trigger element.
    pub id: Option<String>,

    /// Additional attributes to apply to the tab trigger element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab trigger component.
    pub children: Element,
}

/// # TabTrigger
///
/// The `TabTrigger` component is a button that switches to the [`TabContent`] with the same `value` when clicked.
///
/// ## Styling
///
/// The [`TabTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the tab trigger. Values are `active` or `inactive`.
/// - `data-disabled`: Indicates if the tab trigger is disabled. Values are `true` or `false`.
#[component]
#[allow(clippy::redundant_closure)]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let mut ctx: TabsContext = use_context();
    let generated_id = use_unique_id();
    let trigger_id = use_memo({
        let provided_id = props.id.clone();
        move || provided_id.clone().unwrap_or_else(|| generated_id())
    });

    use_effect({
        let index = props.index;
        let value = props.value.clone();
        move || {
            let idx = index();
            let mut values = ctx.tab_values.write();
            while values.len() <= idx {
                values.push(String::new());
            }
            values[idx] = value.clone();

            let mut ids = ctx.tab_trigger_ids.write();
            while ids.len() <= idx {
                ids.push(String::new());
            }
            ids[idx] = trigger_id();
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let value = props.value.clone();
    let selected = use_memo(move || ctx.is_selected(&value));
    let pointerdown_value = props.value.clone();
    let click_value = props.value.clone();
    let keydown_value = props.value.clone();
    let mut was_selected_on_pointer_down = use_signal(|| false);

    let tab_index = use_memo(move || {
        if !(ctx.roving_loop)() {
            return "0";
        }

        if selected() || ctx.focus.is_focused((props.index)()) {
            return "0";
        }

        if (ctx.value)().is_none()
            && ctx.focus.current_focus().is_none()
            && ctx.focus.first_enabled_index() == Some((props.index)())
        {
            return "0";
        }

        "-1"
    });

    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    rsx! {
        button {
            role: "tab",
            id: trigger_id,
            tabindex: tab_index,
            type: "button",

            aria_selected: selected,
            aria_controls: (ctx.tab_content_ids)().get((props.index)()).cloned(),
            "data-state": if selected() { "active" } else { "inactive" },
            "data-disabled": disabled(),
            disabled: disabled(),

            onmounted,
            onpointerdown: move |_| {
                was_selected_on_pointer_down.set(ctx.is_selected(&pointerdown_value));
            },
            onclick: move |_| {
                if !disabled() {
                    if !was_selected_on_pointer_down() {
                        ctx.set_value.call(Some(click_value.clone()));
                    } else {
                        ctx.activate(click_value.clone());
                    }
                }
                was_selected_on_pointer_down.set(false);
            },

            onfocus: move |_| {
                if disabled() {
                    return;
                }
                ctx.set_focus(Some((props.index)()));
            },

            onkeydown: move |event: Event<KeyboardData>| {
                let mut prevent_default = true;
                match event.key() {
                    Key::ArrowUp if (ctx.orientation)() == TabsOrientation::Vertical => {
                        ctx.focus.focus_prev();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::ArrowDown if (ctx.orientation)() == TabsOrientation::Vertical => {
                        ctx.focus.focus_next();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::ArrowLeft if (ctx.orientation)() == TabsOrientation::Horizontal => {
                        ctx.focus.focus_prev();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::ArrowRight if (ctx.orientation)() == TabsOrientation::Horizontal => {
                        ctx.focus.focus_next();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::Home => {
                        ctx.focus.focus_first();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::End => {
                        ctx.focus.focus_last();
                        ctx.activate_focused_tab_if_automatic();
                    }
                    Key::Enter => {
                        if !disabled() {
                            ctx.activate(keydown_value.clone());
                        }
                    }
                    Key::Character(ref ch) if ch == " " => {
                        if !disabled() {
                            ctx.activate(keydown_value.clone());
                        }
                    }
                    _ => prevent_default = false,
                }

                if prevent_default {
                    event.prevent_default();
                }
            },

            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`TabContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct TabContentProps {
    /// The value of the tab content, which must match the `value` prop of the corresponding [`TabTrigger`].
    #[props(into)]
    pub value: String,

    /// The ID of the tab content element.
    pub id: ReadSignal<Option<String>>,

    /// The index of the tab content.
    pub index: ReadSignal<usize>,

    /// Additional attributes to apply to the tab content element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab content element.
    pub children: Element,
}

/// # TabContent
///
/// The content of a tab panel.
///
/// ## Styling
///
/// The [`TabContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the tab panel. Values are `active` or `inactive`.
#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let mut ctx: TabsContext = use_context();
    let selected = use_memo(move || (ctx.value)().as_deref() == Some(props.value.as_str()));
    let generated_id = use_unique_id();
    let id = use_id_or(generated_id, props.id);

    use_effect(move || {
        let mut ids = ctx.tab_content_ids.write();
        let index = (props.index)();
        while ids.len() <= index {
            ids.push(String::new());
        }
        ids[index] = id();
    });

    let labelled_by = use_memo(move || (ctx.tab_trigger_ids)().get((props.index)()).cloned());

    rsx! {
        div {
            role: "tabpanel",
            id,
            tabindex: "0",
            aria_labelledby: labelled_by,
            "data-state": if selected() { "active" } else { "inactive" },
            hidden: !selected(),
            ..props.attributes,

            if ctx.keep_mounted || selected() {
                {props.children}
            }
        }
    }
}
