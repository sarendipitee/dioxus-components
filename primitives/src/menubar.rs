//! Defines the [`Menubar`] component and its sub-components.

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_attributes::attributes;

use crate::{
    floating::{style_prop, use_position},
    focus::{
        use_deferred_focus, use_focus_control, use_focus_entry_disabled, use_focus_provider,
        FocusPlacement, FocusState,
    },
    menu::{self, MenuContext},
    merge_attributes, use_unique_id, ContentAlign, ContentSide,
};

#[derive(Clone, Copy)]
struct MenubarContext {
    // Currently open menu index
    open_menu: Signal<Option<usize>>,
    set_open_menu: Callback<Option<usize>>,
    disabled: ReadSignal<bool>,

    // Focus state
    focus: FocusState,
}

/// The props for the [`Menubar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    /// Whether the menubar is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the menubar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menubar component.
    pub children: Element,
}

/// # Menubar
///
/// The `Menubar` component creates a menu bar that allows users to define multiple grouped dropdowns.
/// Each dropdown menu is represented by a [`MenubarMenu`] component with an associated trigger and content.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Menubar {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "new".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "New"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "open".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Open"
///                     }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "cut".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Cut"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "copy".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Copy"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Menubar`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the menubar is disabled. Values are `true` or `false`.
#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let mut open_menu = use_signal(|| None);
    let set_open_menu = use_callback(move |idx| open_menu.set(idx));

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| MenubarContext {
        open_menu,
        set_open_menu,
        disabled: props.disabled,
        focus,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_menu.peek().is_some() {
            ctx.set_open_menu.call(index);
        }
    });

    rsx! {
        div {
            role: "menubar",
            "data-disabled": (props.disabled)(),
            tabindex: (!ctx.focus.any_focused()).then_some("0"),
            // If the menu receives focus, focus the most recently focused menu item
            onfocus: move |_| {
                ctx.focus.set_focus(Some(ctx.focus.recent_focus_or_default()));
            },

            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct MenubarMenuContext {
    index: ReadSignal<usize>,
    focus: FocusState,
    is_open: Memo<bool>,
    disabled: ReadSignal<bool>,
    initial_focus: Signal<Option<FocusPlacement>>,
}

impl MenubarMenuContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

/// The props for the [`MenubarMenu`] component.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    /// The index of this menu in the menubar. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether this menu is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to apply to the menu element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the menu component.
    pub children: Element,
}

/// # MenubarMenu
///
/// The `MenubarMenu` component represents a single menu within a menubar. It contains a [`MenubarTrigger`]
/// to open the menu and a [`MenubarContent`] that holds the menu items. Each menu must define an index
/// to establish its position within the menubar.
///
/// This must be used inside a [`Menubar`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Menubar {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "new".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "New"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "open".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Open"
///                     }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "cut".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Cut"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "copy".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Copy"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`MenubarMenu`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the menu is open or closed. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the menu is disabled. Values are `true` or `false`.
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let is_open = use_memo(move || (ctx.open_menu)() == Some(props.index.cloned()));
    let focus = use_focus_provider(ctx.focus.roving_loop);
    let initial_focus = use_signal(|| None);
    let trigger_id = use_unique_id();
    let trigger_ref = use_signal(|| None);
    let mut menu_ctx = use_context_provider(|| MenubarMenuContext {
        index: props.index,
        focus,
        is_open,
        disabled: props.disabled,
        initial_focus,
    });
    let set_menu_open = use_callback(move |open: bool| {
        ctx.set_open_menu.call(open.then_some(props.index.cloned()));
    });
    let shared_disabled = use_memo(move || (ctx.disabled)() || (props.disabled)());
    use_context_provider(|| MenuContext {
        open: is_open,
        set_open: set_menu_open,
        disabled: shared_disabled,
        focus,
        trigger_id,
        trigger_ref,
    });

    use_effect(move || {
        if !is_open() {
            menu_ctx.focus.blur();
            menu_ctx.initial_focus.set(None);
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    use_focus_entry_disabled(ctx.focus, menu_ctx.index, disabled);

    rsx! {
        div {
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_menu.call((!is_open()).then(&*props.index));
                    }
                    Key::Escape => ctx.set_open_menu.call(None),
                    Key::ArrowLeft => ctx.focus.focus_prev(),
                    Key::ArrowRight => ctx.focus.focus_next(),
                    Key::ArrowDown if !disabled() => {
                        if !is_open() {
                            menu_ctx.initial_focus.set(Some(FocusPlacement::First));
                            ctx.set_open_menu.call(Some(props.index.cloned()));
                        } else {
                            menu_ctx.focus_next();
                        }
                    },
                    Key::ArrowUp if !disabled() => {
                        if is_open() {
                            menu_ctx.focus_prev();
                        } else {
                            menu_ctx.initial_focus.set(Some(FocusPlacement::Last));
                            ctx.set_open_menu.call(Some(props.index.cloned()));
                        }
                    },
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
                    _ => return,
                }
                event.prevent_default();
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger component.
    pub children: Element,
}

/// # MenubarTrigger
///
/// The `MenubarTrigger` component is a button that opens and closes a [`MenubarMenu`] when clicked.
///
/// This must be used inside a [`MenubarMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Menubar {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "new".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "New"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "open".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Open"
///                     }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "cut".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Cut"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "copy".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Copy"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let menu_ctx: MenubarMenuContext = use_context();
    let shared_menu_ctx: MenuContext = use_context();
    let mut trigger_ref = shared_menu_ctx.trigger_ref;
    let mut focus_onmounted = use_focus_control(ctx.focus, menu_ctx.index);
    // Drive both the roving-focus controller and the floating-ui reference ref from
    // the single element ref (an element can only carry one `onmounted`).
    let onmounted = move |evt: MountedEvent| {
        trigger_ref.set(Some(evt.data()));
        focus_onmounted(evt);
    };
    let disabled = move || (ctx.disabled)() || (menu_ctx.disabled)();
    let is_open = menu_ctx.is_open;
    let index = menu_ctx.index;
    let is_focused = move || {
        ctx.focus.current_focus() == Some(menu_ctx.index.cloned()) && !menu_ctx.focus.any_focused()
    };

    rsx! {
        button {
            id: shared_menu_ctx.trigger_id,
            onmounted,
            onpointerup: move |_| {
                if !disabled() {
                    let new_open = if is_open() { None } else { Some(index.cloned()) };
                    ctx.set_open_menu.call(new_open);
                    ctx.focus.set_focus(Some(index.cloned()));
                }
            },
            onmouseenter: move |_| {
                if !disabled() && (ctx.open_menu)().is_some() {
                    ctx.focus.set_focus(Some(index.cloned()));
                }
            },
            onblur: move |_| {
                if is_focused() && !is_open() {
                    ctx.focus.set_focus(None);
                }
            },
            role: "menuitem",
            type: "button",
            tabindex: if is_focused() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    /// The id of the content element.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to apply to the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the content component.
    pub children: Element,
}

/// # MenubarContent
///
/// The `MenubarContent` component defines the content of a [`MenubarMenu`]. It will only be rendered if the menu is open.
///
/// This must be used inside a [`MenubarMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Menubar {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "new".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "New"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "open".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Open"
///                     }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "cut".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Cut"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "copy".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Copy"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`MenubarContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the menu is open or closed. Values are `open` or `closed`.
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let menu_ctx: MenubarMenuContext = use_context();
    let shared_menu_ctx: MenuContext = use_context();
    use_deferred_focus(menu_ctx.focus, menu_ctx.initial_focus, move || {
        (shared_menu_ctx.open)()
    });

    // Floating-element positioning. Menubar content opens below its trigger, aligned
    // to the trigger's left edge (matching the legacy `top:100% left:0` CSS);
    // flip()/shift() handle viewport edges. On native the hook is inert and the
    // `:not([data-floating])` CSS fallback provides the static placement.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(
        shared_menu_ctx.trigger_ref,
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

    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
        "data-floating": floating_active.then_some("true"),
        onmounted: move |evt: MountedEvent| floating_ref.set(Some(evt.data())),
    });
    // Floating props must win over user-forwarded coords → place them last.
    let attributes = merge_attributes(vec![props.attributes, floating_attrs]);

    rsx! {
        menu::MenuContent {
            id: props.id,
            attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// The index of this item within the [`MenubarContent`]. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// The value associated with this menu item. This value will be passed to the [`Self::on_select`] callback when the item is selected.
    #[props(into)]
    pub value: String,

    /// Whether this menu item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Callback fired when the item is selected. The [`Self::value`] will be passed as an argument.
    #[props(default)]
    pub on_select: Callback<String>,

    /// Whether the menu should close after the item is selected.
    #[props(default = true)]
    pub close_on_select: bool,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the item component.
    pub children: Element,
}

/// # MenubarItem
///
/// The `MenubarItem` component represents a selectable item within a menu. In addition to calling the
/// [`MenubarItemProps::on_select`] callback, the menu will close when the item is selected.
///
/// This must be used inside a [`MenubarContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::menubar::{
///     Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Menubar {
///             MenubarMenu { index: 0usize,
///                 MenubarTrigger { "File" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "new".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "New"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "open".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Open"
///                     }
///                 }
///             }
///             MenubarMenu { index: 1usize,
///                 MenubarTrigger { "Edit" }
///                 MenubarContent {
///                     MenubarItem {
///                         index: 0usize,
///                         value: "cut".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Cut"
///                     }
///                     MenubarItem {
///                         index: 1usize,
///                         value: "copy".to_string(),
///                         on_select: move |value| {
///                             tracing::info!("Selected value: {}", value);
///                         },
///                         "Copy"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`MenubarItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the item is disabled. Values are `true` or `false`.
#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        menu::MenuItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarLabel`] component.
pub type MenubarLabelProps = menu::MenuLabelProps;

/// A non-interactive label within a [`MenubarContent`].
#[component]
pub fn MenubarLabel(props: MenubarLabelProps) -> Element {
    rsx! {
        menu::MenuLabel {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarSeparator`] component.
pub type MenubarSeparatorProps = menu::MenuSeparatorProps;

/// A separator between groups of menubar menu items.
#[component]
pub fn MenubarSeparator(props: MenubarSeparatorProps) -> Element {
    rsx! {
        menu::MenuSeparator {
            attributes: props.attributes,
        }
    }
}

/// The props for the [`MenubarGroup`] component.
pub type MenubarGroupProps = menu::MenuGroupProps;

/// A semantic group of related menubar menu items.
#[component]
pub fn MenubarGroup(props: MenubarGroupProps) -> Element {
    rsx! {
        menu::MenuGroup {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarItemIndicator`] component.
pub type MenubarItemIndicatorProps = menu::MenuItemIndicatorProps;

/// A presentational indicator for checked menubar menu items.
#[component]
pub fn MenubarItemIndicator(props: MenubarItemIndicatorProps) -> Element {
    rsx! {
        menu::MenuItemIndicator {
            visible: props.visible,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarItemSection`] component.
pub type MenubarItemSectionProps = menu::MenuItemSectionProps;

/// A presentational section inside a menubar menu item.
#[component]
pub fn MenubarItemSection(props: MenubarItemSectionProps) -> Element {
    rsx! {
        menu::MenuItemSection {
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarCheckboxItem`] component.
pub type MenubarCheckboxItemProps = menu::MenuCheckboxItemProps<String>;

/// A checkbox-style menubar menu item.
#[component]
pub fn MenubarCheckboxItem(props: MenubarCheckboxItemProps) -> Element {
    rsx! {
        menu::MenuCheckboxItem {
            value: props.value,
            index: props.index,
            checked: props.checked,
            disabled: props.disabled,
            on_checked_change: props.on_checked_change,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarRadioGroup`] component.
pub type MenubarRadioGroupProps = menu::MenuRadioGroupProps<String>;

/// A group that coordinates related menubar radio items.
#[component]
pub fn MenubarRadioGroup(props: MenubarRadioGroupProps) -> Element {
    rsx! {
        menu::MenuRadioGroup {
            value: props.value,
            on_value_change: props.on_value_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarRadioItem`] component.
pub type MenubarRadioItemProps = menu::MenuRadioItemProps<String>;

/// A radio-style menubar menu item.
#[component]
pub fn MenubarRadioItem(props: MenubarRadioItemProps) -> Element {
    rsx! {
        menu::MenuRadioItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            close_on_select: props.close_on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarSub`] component.
pub type MenubarSubProps = menu::MenuSubProps;

/// A nested menubar submenu root.
#[component]
pub fn MenubarSub(props: MenubarSubProps) -> Element {
    rsx! {
        menu::MenuSub {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarSubTrigger`] component.
pub type MenubarSubTriggerProps = menu::MenuSubTriggerProps<String>;

/// A menubar menu item that opens a nested submenu.
#[component]
pub fn MenubarSubTrigger(props: MenubarSubTriggerProps) -> Element {
    rsx! {
        menu::MenuSubTrigger {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`MenubarSubContent`] component.
pub type MenubarSubContentProps = menu::MenuSubContentProps;

/// The popup content for a nested menubar submenu.
#[component]
pub fn MenubarSubContent(props: MenubarSubContentProps) -> Element {
    rsx! {
        menu::MenuSubContent {
            id: props.id,
            role: props.role,
            attributes: props.attributes,
            {props.children}
        }
    }
}
