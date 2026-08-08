use crate::component_styles;
use crate::components::button::{Button, ButtonVariant};
use crate::components::separator::Separator;
use crate::components::sheet::{Sheet, SheetSide};
use crate::components::skeleton::Skeleton;
use crate::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};
use dioxus::core::use_drop;
use dioxus::prelude::*;
use dioxus_icons::lucide::PanelLeft;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::use_controlled;
use std::sync::atomic::{AtomicUsize, Ordering};

#[component_styles("./style.css")]
struct Styles;

// constants
const SIDEBAR_WIDTH: &str = "16rem";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";
const SIDEBAR_DEFAULT_WIDTH: f64 = 256.0;
const SIDEBAR_DEFAULT_MIN_WIDTH: f64 = 192.0;
const SIDEBAR_DEFAULT_MAX_WIDTH: f64 = 480.0;
static SIDEBAR_RAIL_ID: AtomicUsize = AtomicUsize::new(0);
static SIDEBAR_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
struct SidebarResizeCtx {
    width: Signal<f64>,
    min_width: ReadSignal<f64>,
    max_width: ReadSignal<f64>,
}
const MOBILE_BREAKPOINT: u32 = 768;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

impl SidebarState {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarState::Expanded => "expanded",
            SidebarState::Collapsed => "collapsed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct SidebarCtx {
    pub state: Memo<SidebarState>,
    pub side: Signal<SidebarSide>,
    pub is_mobile: Signal<bool>,
    // From use_controlled:
    open: Memo<bool>,
    set_open: Callback<bool>,
    // Mobile state:
    open_mobile: Signal<bool>,
    sidebar_id: Signal<String>,
}

impl SidebarCtx {
    /// Toggle the sidebar open/closed state
    pub fn toggle(&self) {
        if (self.is_mobile)() {
            let current = (self.open_mobile)();
            let mut open_mobile = self.open_mobile;
            open_mobile.set(!current);
        } else {
            self.set_open.call(!self.open());
        }
    }

    /// Set the mobile sidebar open state
    pub fn set_open_mobile(&self, value: bool) {
        let mut open_mobile = self.open_mobile;
        open_mobile.set(value);
    }

    /// Get the current open state (desktop)
    pub fn open(&self) -> bool {
        self.open.cloned()
    }
}

pub fn use_sidebar() -> SidebarCtx {
    use_context::<SidebarCtx>()
}

pub fn use_is_mobile() -> Signal<bool> {
    let mut is_mobile = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function checkMobile() {{
                    return window.innerWidth < {MOBILE_BREAKPOINT};
                }}
                function handleResize() {{
                    dioxus.send(checkMobile());
                }}
                window.__sidebarResizeHandler = handleResize;
                window.addEventListener('resize', window.__sidebarResizeHandler);
                dioxus.send(checkMobile());
                "#
            );
            let mut eval = document::eval(&js_code);

            while let Ok(result) = eval.recv::<bool>().await {
                is_mobile.set(result);
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
            window.removeEventListener('resize', window.__sidebarResizeHandler);
            delete window.__sidebarResizeHandler;
            "#,
        );
    });

    is_mobile
}

#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let is_mobile = use_is_mobile();
    let side = use_signal(|| SidebarSide::Left);
    let open_mobile = use_signal(|| false);
    let sidebar_id =
        use_signal(|| format!("dx-sidebar-{}", SIDEBAR_ID.fetch_add(1, Ordering::Relaxed)));

    let (open, set_open) = use_controlled(open, default_open, on_open_change);

    let state = use_memo(move || {
        if open() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    let ctx = SidebarCtx {
        state,
        side,
        is_mobile,
        open,
        set_open,
        open_mobile,
        sidebar_id,
    };

    use_context_provider(|| ctx);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function sidebarKeyHandler(event) {{
                    if (event.key === '{SIDEBAR_KEYBOARD_SHORTCUT}' && (event.metaKey || event.ctrlKey)) {{
                        event.preventDefault();
                        dioxus.send(true);
                    }}
                }}
                window.__sidebarKeyHandler = sidebarKeyHandler;
                window.addEventListener('keydown', window.__sidebarKeyHandler);
                "#
            );
            let mut eval = document::eval(&js_code);

            loop {
                if eval.recv::<bool>().await.is_ok() {
                    ctx.toggle();
                }
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
            window.removeEventListener('keydown', window.__sidebarKeyHandler);
            delete window.__sidebarKeyHandler;
            "#,
        );
    });

    let sidebar_style = format!(
        r#"
        --dx-sidebar-width: {SIDEBAR_WIDTH};
        --dx-sidebar-width-mobile: {SIDEBAR_WIDTH_MOBILE};
        --dx-sidebar-width-icon: {SIDEBAR_WIDTH_ICON}
        "#
    );

    let base = attributes!(div {
        class: Styles::dx_sidebar_wrapper,
        "data-slot": "sidebar-wrapper",
        style: sidebar_style,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    /// Minimum desktop width in CSS pixels while resizing.
    #[props(default = SIDEBAR_DEFAULT_MIN_WIDTH)]
    min_width: ReadSignal<f64>,
    /// Maximum desktop width in CSS pixels while resizing.
    #[props(default = SIDEBAR_DEFAULT_MAX_WIDTH)]
    max_width: ReadSignal<f64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let width = use_signal(|| SIDEBAR_DEFAULT_WIDTH);
    let ctx = use_sidebar();
    let mut ctx_side = ctx.side;
    if *ctx_side.peek() != side {
        ctx_side.set(side);
    }

    let is_mobile = ctx.is_mobile;
    let state = ctx.state;
    let open_mobile = ctx.open_mobile;

    if collapsible == SidebarCollapsible::None {
        let base = attributes!(div {
            id: (ctx.sidebar_id)(),
            class: Styles::dx_sidebar_static,
            "data-slot": "sidebar",
        });
        let merged = merge_attributes(vec![base, attributes]);

        return rsx! {
            div { ..merged, {children} }
        };
    }

    if is_mobile() {
        let sheet_side = match side {
            SidebarSide::Left => SheetSide::Left,
            SidebarSide::Right => SheetSide::Right,
        };

        return rsx! {
            Sheet {
                open: open_mobile(),
                on_open_change: move |v| ctx.set_open_mobile(v),
                side: sheet_side,
                id: (ctx.sidebar_id)(),
                class: Styles::dx_sidebar_sheet,
                "data-sidebar": "sidebar",
                "data-slot": "sidebar",
                "data-mobile": "true",
                title: "Sidebar",
                description: "Displays the mobile sidebar.",
                with_close_button: false,
                div { class: Styles::dx_sidebar_mobile_inner, {children} }
            }
        };
    }

    let collapsible_str = if state() == SidebarState::Collapsed {
        collapsible.as_str()
    } else {
        ""
    };

    let resize_ctx = SidebarResizeCtx {
        width,
        min_width,
        max_width,
    };
    use_context_provider(|| resize_ctx);
    let desktop_style = format!("--dx-sidebar-width: {}px", width());

    let container_base = attributes!(div {
        class: Styles::dx_sidebar_container,
        "data-slot": "sidebar-container",
    });
    let container_attrs = merge_attributes(vec![container_base, attributes]);

    rsx! {
        div {
            id: (ctx.sidebar_id)(),
            class: Styles::dx_sidebar_desktop,
            "data-state": state().as_str(),
            "data-collapsible": collapsible_str,
            "data-variant": variant.as_str(),
            "data-side": side.as_str(),
            "data-slot": "sidebar",
            style: desktop_style,
            div { class: Styles::dx_sidebar_gap, "data-slot": "sidebar-gap" }
            div {
                class: Styles::dx_sidebar_hotzone,
                "data-slot": "sidebar-hotzone",
                "aria-hidden": "true",
            }
            div {
                ..container_attrs,
                div {
                    class: Styles::dx_sidebar_inner,
                    "data-sidebar": "sidebar",
                    "data-slot": "sidebar-inner",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn SidebarTrigger(
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_sidebar();

    let base = attributes!(button {
        class: Styles::dx_sidebar_trigger,
        "data-sidebar": "trigger",
        "data-slot": "sidebar-trigger",
        aria_controls: (ctx.sidebar_id)(),
        aria_expanded: if (ctx.is_mobile)() {
            (ctx.open_mobile)()
        } else {
            (ctx.open)()
        },
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
                ctx.toggle();
            },
            attributes: merged,
            PanelLeft {
                class: Styles::dx_sidebar_trigger_icon,
                size: "1rem",
            }
            span { class: Styles::dx_sr_only, "Toggle Sidebar" }
        }
    }
}

#[component]
pub fn SidebarRail(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let ctx = use_sidebar();
    let resize = use_context::<SidebarResizeCtx>();
    let rail_id = use_hook(|| SIDEBAR_RAIL_ID.fetch_add(1, Ordering::Relaxed));
    // Keep the click decision on the Rust event path. The evaluator reports its
    // result asynchronously, after the browser may already have dispatched click.
    let mut pointer_active = use_signal(|| false);
    let mut pointer_dragged = use_signal(|| false);
    let mut pointer_start_x = use_signal(|| 0.0_f64);
    let mut dragged = use_signal(|| false);
    let base = attributes!(button {
        class: Styles::dx_sidebar_rail,
        "data-sidebar": "rail",
        "data-slot": "sidebar-rail",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        button {
            aria_label: "Resize Sidebar",
            tabindex: -1,
            title: "Drag to resize sidebar",
            "data-sidebar-rail-id": rail_id,
            onpointerdown: move |event| {
                if event.trigger_button() != Some(dioxus::html::input_data::MouseButton::Primary) {
                    return;
                }
                let was_collapsed = (ctx.state)() == SidebarState::Collapsed;
                pointer_active.set(true);
                pointer_dragged.set(false);
                pointer_start_x.set(event.client_coordinates().x);
                if !was_collapsed {
                    event.prevent_default();
                }
                let start_width = (resize.width)();
                let min = (resize.min_width)().max(0.0);
                let max = (resize.max_width)().max(min);
                dragged.set(false);
                let start_x = event.client_coordinates().x;

                spawn(async move {
                    let mut eval = document::eval(&format!(r#"
                        const rail = document.querySelector('[data-sidebar-rail-id="{rail_id}"]');
                        const sidebar = rail.closest('[data-slot="sidebar"]');
                        const startX = {start_x};
                        const startWidth = {start_width};
                        const minWidth = {min};
                        const maxWidth = {max};
                        const side = sidebar.dataset.side;
                        const canExpandFromDrag = sidebar.dataset.variant !== 'floating';
                        const direction = side === 'left' ? 1 : -1;
                        const collapseDistance = minWidth * 0.2;
                        const dragThreshold = 30;
                        let width = startWidth;
                        let moved = false;
                        let frame = 0;
                        let finished = false;

                        sidebar.dataset.resizing = 'true';
                        const apply = clientX => {{
                            width = Math.min(maxWidth, Math.max(minWidth,
                                startWidth + (clientX - startX) * direction));
                            sidebar.style.setProperty('--dx-sidebar-width', `${{width}}px`);
                        }};
                        const shouldCollapse = clientX => side === 'left'
                            ? clientX <= collapseDistance
                            : window.innerWidth - clientX <= collapseDistance;
                        const cleanup = () => {{
                            cancelAnimationFrame(frame);
                            delete sidebar.dataset.resizing;
                            window.removeEventListener('pointermove', move);
                            window.removeEventListener('pointerup', end);
                            window.removeEventListener('pointercancel', cancel);
                        }};
                        const finish = (event, collapse) => {{
                            if (finished) return;
                            finished = true;
                            if (moved && !{was_collapsed}) {{
                                event.preventDefault();
                                apply(event.clientX);
                            }}
                            const openFromClick = false;
                            if (moved) {{
                                rail.addEventListener('click', event => {{
                                    event.preventDefault();
                                    event.stopImmediatePropagation();
                                }}, {{ once: true, capture: true }});
                            }}
                            const mustStayCollapsed = {was_collapsed} && !canExpandFromDrag;
                            const result = [width, moved, moved && (collapse || mustStayCollapsed), openFromClick, true];
                            dioxus.send(result);
                        }};
                        const move = event => {{
                            event.preventDefault();
                            moved ||= Math.abs(event.clientX - startX) >= dragThreshold;
                            if ({was_collapsed}) {{
                                if (canExpandFromDrag && moved) {{
                                    finished = true;
                                    width = minWidth;
                                    sidebar.style.setProperty('--dx-sidebar-width', `${{width}}px`);
                                    cleanup();
                                    rail.addEventListener('click', event => {{
                                        event.preventDefault();
                                        event.stopImmediatePropagation();
                                    }}, {{ once: true, capture: true }});
                                    dioxus.send([width, true, false, false, true]);
                                }}
                                return;
                            }}
                            if (shouldCollapse(event.clientX)) {{
                                finish(event, true);
                                return;
                            }}
                            cancelAnimationFrame(frame);
                            frame = requestAnimationFrame(() => apply(event.clientX));
                        }};
                        const end = event => finish(event, shouldCollapse(event.clientX));
                        const cancel = event => finish(event, false);
                        window.addEventListener('pointermove', move, {{ passive: false }});
                        window.addEventListener('pointerup', end, {{ once: true }});
                        window.addEventListener('pointercancel', cancel, {{ once: true }});
                    "#));
                    while let Ok((final_width, was_dragged, collapse, open_from_click, finished)) =
                        eval.recv::<(f64, bool, bool, bool, bool)>().await
                    {
                        if !finished {
                            ctx.set_open.call(true);
                            continue;
                        }
                        if open_from_click {
                            // Clicks are handled synchronously by the Rust `onclick` handler.
                        }
                        if was_dragged {
                            dragged.set(true);
                            let mut width = resize.width;
                            let minimum = (resize.min_width)().max(0.0);
                            let maximum = (resize.max_width)().max(minimum);
                            width.set(final_width.clamp(minimum, maximum));
                            ctx.set_open.call(!collapse);
                        }
                        pointer_active.set(false);
                        pointer_dragged.set(false);
                        break;
                    }
                });
            },
            onpointermove: move |event| {
                if pointer_active()
                    && (event.client_coordinates().x - pointer_start_x()).abs() >= 30.0
                {
                    pointer_dragged.set(true);
                }
            },
            onpointerup: move |event| {
                if pointer_active()
                    && (event.client_coordinates().x - pointer_start_x()).abs() >= 30.0
                {
                    pointer_dragged.set(true);
                }
            },
            onpointercancel: move |_| {
                pointer_active.set(false);
                pointer_dragged.set(false);
            },
            onclick: move |event| {
                let was_dragged = pointer_dragged()
                    || (pointer_active()
                        && (event.client_coordinates().x - pointer_start_x()).abs() >= 30.0);
                pointer_active.set(false);
                pointer_dragged.set(false);
                if !was_dragged {
                    ctx.toggle();
                }
            },
            ..merged,
        }
    }
}

#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(main {
        class: Styles::dx_sidebar_inset,
        "data-slot": "sidebar-inset",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        main { ..merged, {children} }
    }
}

#[component]
pub fn SidebarHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_header,
        "data-slot": "sidebar-header",
        "data-sidebar": "header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_content,
        "data-slot": "sidebar-content",
        "data-sidebar": "content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_footer,
        "data-slot": "sidebar-footer",
        "data-sidebar": "footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarSeparator(
    #[props(default = true)] horizontal: bool,
    #[props(default = true)] decorative: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_separator,
        "data-slot": "sidebar-separator",
        "data-sidebar": "separator",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        Separator { horizontal, decorative, attributes: merged }
    }
}

#[component]
pub fn SidebarGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_group,
        "data-slot": "sidebar-group",
        "data-sidebar": "group",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarGroupLabel(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_group_label,
        "data-slot": "sidebar-group-label",
        "data-sidebar": "group-label",
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            div { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarGroupAction(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_sidebar_group_action,
        "data-slot": "sidebar-group-action",
        "data-sidebar": "group-action",
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarGroupContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_group_content,
        "data-slot": "sidebar-group-content",
        "data-sidebar": "group-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarMenu(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul {
        class: Styles::dx_sidebar_menu,
        "data-slot": "sidebar-menu",
        "data-sidebar": "menu",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        ul { ..merged, {children} }
    }
}

#[component]
pub fn SidebarMenuItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li {
        class: Styles::dx_sidebar_menu_item,
        "data-slot": "sidebar-menu-item",
        "data-sidebar": "menu-item",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        li { ..merged, {children} }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

impl SidebarMenuButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonVariant::Default => "default",
            SidebarMenuButtonVariant::Outline => "outline",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SidebarMenuButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuButtonSize::Default => "default",
            SidebarMenuButtonSize::Sm => "sm",
            SidebarMenuButtonSize::Lg => "lg",
        }
    }
}

#[component]
pub fn SidebarMenuButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] variant: SidebarMenuButtonVariant,
    #[props(default)] size: SidebarMenuButtonSize,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] tooltip: Option<Element>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx = use_sidebar();
    let is_mobile = ctx.is_mobile;
    let state = ctx.state;

    let base = attributes!(button {
        class: Styles::dx_sidebar_menu_button,
        "data-slot": "sidebar-menu-button",
        "data-sidebar": "menu-button",
        "data-size": size.as_str(),
        "data-variant": variant.as_str(),
        "data-active": if is_active { "true" } else { "false" },
    });
    let merged = merge_attributes(vec![base, attributes]);

    let Some(tooltip_content) = tooltip else {
        return if let Some(dynamic) = r#as {
            dynamic.call(merged)
        } else {
            rsx! { button { ..merged, {children} } }
        };
    };

    let hidden = state() != SidebarState::Collapsed || is_mobile();
    let sidebar_side = ctx.side;

    rsx! {
        Tooltip {
            class: Styles::dx_sidebar_tooltip,
            disabled: hidden,
            TooltipTrigger {
                as: move |tooltip_attrs: Vec<Attribute>| {
                    let final_attrs = merge_attributes(vec![tooltip_attrs, merged.clone()]);
                    let children = children.clone();
                    if let Some(dynamic) = &r#as {
                        dynamic.call(final_attrs)
                    } else {
                        rsx! { button { ..final_attrs, {children} } }
                    }
                },
            }
            TooltipContent {
                side: match sidebar_side() {
                    SidebarSide::Left => dioxus_primitives::ContentSide::Right,
                    SidebarSide::Right => dioxus_primitives::ContentSide::Left,
                },
                {tooltip_content}
            }
        }
    }
}

#[component]
pub fn SidebarMenuAction(
    #[props(default = false)] show_on_hover: bool,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_sidebar_menu_action,
        "data-slot": "sidebar-menu-action",
        "data-sidebar": "menu-action",
        "data-show-on-hover": if show_on_hover { "true" } else { "false" },
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged,{children} }
        }
    }
}

#[component]
pub fn SidebarMenuBadge(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_menu_badge,
        "data-slot": "sidebar-menu-badge",
        "data-sidebar": "menu-badge",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn SidebarMenuSkeleton(
    #[props(default = false)] show_icon: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_menu_skeleton,
        "data-slot": "sidebar-menu-skeleton",
        "data-sidebar": "menu-skeleton",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            if show_icon {
                Skeleton { class: Styles::dx_sidebar_menu_skeleton_icon }
            }
            Skeleton { class: Styles::dx_sidebar_menu_skeleton_text, width: "70%" }
        }
    }
}

#[component]
pub fn SidebarMenuSub(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul {
        class: Styles::dx_sidebar_menu_sub,
        "data-slot": "sidebar-menu-sub",
        "data-sidebar": "menu-sub",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        ul { ..merged, {children} }
    }
}

#[component]
pub fn SidebarMenuSubItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li {
        class: Styles::dx_sidebar_menu_sub_item,
        "data-slot": "sidebar-menu-sub-item",
        "data-sidebar": "menu-sub-item",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        li { ..merged, {children} }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

impl SidebarMenuSubButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarMenuSubButtonSize::Sm => "sm",
            SidebarMenuSubButtonSize::Md => "md",
        }
    }
}

#[component]
pub fn SidebarMenuSubButton(
    #[props(default = false)] is_active: bool,
    #[props(default)] size: SidebarMenuSubButtonSize,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(a {
        class: Styles::dx_sidebar_menu_sub_button,
        "data-slot": "sidebar-menu-sub-button",
        "data-sidebar": "menu-sub-button",
        "data-size": size.as_str(),
        "data-active": if is_active { "true" } else { "false" },
    });
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            a { ..merged, {children} }
        }
    }
}
