use dioxus::prelude::*;
use dioxus_primitives::tabs::{
    self, TabContentProps, TabListProps, TabTriggerProps, TabsActivationMode, TabsJustify,
    TabsOrientation, TabsPlacement,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/tabs/style.css")]
struct Styles;

/// Visual variants for the styled Tabs wrapper.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    /// Classic tab bar styling with a border rail and active indicator.
    #[default]
    Default,
    /// Segmented control styling with a muted container.
    Outline,
    /// Rounded pill-style triggers.
    Pills,
    /// Minimal ghost styling with subtle active fill.
    Ghost,
}

impl TabsVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Pills => "pills",
            Self::Ghost => "ghost",
        }
    }
}

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The controlled value of the active tab. `None` means no tab is active.
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

    /// Explicit orientation override.
    #[props(default)]
    pub orientation: Option<TabsOrientation>,

    /// Backwards-compatible orientation fallback.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub horizontal: ReadSignal<bool>,

    /// Whether arrow-key focus should automatically activate the focused tab.
    #[props(default)]
    pub activation_mode: TabsActivationMode,

    /// Whether clicking the active tab can clear the active selection.
    #[props(default = false)]
    pub allow_tab_deactivation: bool,

    /// Whether inactive tab panels should remain mounted.
    #[props(default = true)]
    pub keep_mounted: bool,

    /// Whether the tab list should stretch its triggers across the available width.
    #[props(default = false)]
    pub grow: bool,

    /// Horizontal alignment for the tab list.
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

    /// The visual variant.
    #[props(default)]
    pub variant: TabsVariant,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tabs,
        "data-variant": props.variant.as_str(),
        "data-slot": "tabs-root",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tabs::Tabs {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            orientation: props.orientation,
            horizontal: props.horizontal,
            activation_mode: props.activation_mode,
            allow_tab_deactivation: props.allow_tab_deactivation,
            keep_mounted: props.keep_mounted,
            grow: props.grow,
            justify: props.justify,
            placement: props.placement,
            inverted: props.inverted,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tabs_list,
        "data-slot": "tabs-list",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tabs::TabList { scrollable: props.scrollable, attributes: merged, {props.children} }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_tabs_trigger,
        "data-slot": "tabs-tab",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tabs::TabTrigger {
            id: props.id,
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tabs_content,
        "data-slot": "tabs-panel",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tabs::TabContent {
            value: props.value,
            id: props.id,
            index: props.index,
            attributes: merged,
            {props.children}
        }
    }
}
