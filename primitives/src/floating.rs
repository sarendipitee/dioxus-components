//! Shared floating-element positioning for overlay primitives.
//!
//! This module exposes a single internal abstraction, [`use_position`], reused by
//! every overlay primitive (popover, tooltip, hover card, menus, select, combobox)
//! so the wasm-gating, middleware configuration, leak-safe reposition wiring, and
//! the `floating_ui_dioxus::Placement` ↔ [`ContentSide`]/[`ContentAlign`]
//! mapping live in exactly one place.
//!
//! On the web (`cfg(target_family = "wasm")`) it delegates to
//! `floating_ui_dioxus::use_floating` for real collision handling (offset + flip +
//! shift). On native/desktop builds the floating-ui crates are absent, so the hook is
//! inert and preserves the existing CSS-only `data-side`/`data-align` behavior. Both
//! implementations share the same signature and return the same dioxus-core-only
//! [`PositionState`], so call sites are unconditional within a target.
//!
#[cfg(target_family = "wasm")]
use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::{ContentAlign, ContentSide};

/// The default gap, in pixels, between the trigger (reference) element and the
/// floating element. Matches the `--floating-offset` CSS token, which
/// resolves to roughly `8px` with the default `--space` of `4px`.
///
/// Only referenced by the wasm `use_position` (the native impl is CSS-driven), so it
/// is `dead_code` on native targets.
#[cfg_attr(not(target_family = "wasm"), allow(dead_code))]
pub(crate) const DEFAULT_OFFSET_GAP: f64 = 8.0;

/// Computed positioning state for a floating overlay element.
///
/// Every field is a dioxus-core type so the wasm and native implementations of
/// [`use_position`] return the same type and the native build never names a
/// floating-ui symbol.
pub(crate) struct PositionState {
    /// Inline `style` string for the floating element (e.g. `position`/`left`/`top`).
    /// Empty on native, where positioning stays CSS-driven.
    pub style: Memo<String>,
    /// Handles the floating element's mount and computes its initial position.
    pub on_mounted: Callback<Rc<MountedData>>,
    /// Resolved side after collision middleware.
    pub side: Memo<ContentSide>,
    /// Resolved alignment after collision middleware.
    pub align: Memo<ContentAlign>,
    /// Whether valid coordinates have been computed.
    pub is_positioned: Memo<bool>,
    /// Whether floating-ui is the active positioning engine.
    pub floating_active: bool,
    /// Width of the active positioning reference in CSS pixels.
    pub reference_width: Memo<Option<f64>>,
}

/// Position a floating overlay element relative to a reference (trigger) element.
///
/// Returns a [`PositionState`] whose memos drive the floating element's inline
/// `style`, resolved `data-side`/`data-align`, and an until-positioned visibility
/// guard.
///
/// On the web this computes fixed coordinates with `offset`/`flip`/`shift`
/// middleware and keeps them current with `auto_update`, including reference
/// layout shifts and animation-frame movement. The returned cleanup is retained
/// and called on remount or component drop. On native the hook is inert and
/// preserves the requested side/align unchanged.
///
/// The `requested_side`/`requested_align` are treated as mount-time constants: the
/// floating-ui middleware is captured once at the hook call, so runtime changes to
/// the requested placement do not re-run middleware (flip still resolves the side at
/// runtime). This hook must be called unconditionally by callers; the internal hook
/// calls run every render.
#[cfg(target_family = "wasm")]
pub(crate) fn use_position(
    reference: Signal<Option<Rc<MountedData>>>,
    mut floating: Signal<Option<Rc<MountedData>>>,
    requested_side: ContentSide,
    requested_align: ContentAlign,
    target_selector: Option<String>,
) -> PositionState {
    use dioxus::web::WebEventExt;
    use floating_ui_dom::{
        auto_update, compute_position, AutoUpdateOptions, Flip, FlipOptions, Middleware, Offset,
        OffsetOptions, Shift, ShiftOptions, Strategy,
    };

    let mut style = use_signal(|| "position: fixed; top: 0px; left: 0px;".to_string());
    let mut side = use_signal(|| requested_side);
    let mut align = use_signal(|| requested_align);
    let mut is_positioned = use_signal(|| false);
    let mut measured_reference_width = use_signal(|| None);
    let cleanup = use_hook(|| Rc::new(RefCell::new(None::<Box<dyn Fn()>>)));
    let position_target_selector = target_selector.clone();

    let position = use_callback(move |_| {
        let Some(reference) = reference() else {
            return;
        };
        let Some(floating) = floating() else {
            return;
        };
        let reference_root = reference.as_web_event();
        let reference = if let Some(selector) = position_target_selector.as_deref() {
            let Ok(Some(target)) = reference_root.query_selector(selector) else {
                return;
            };
            target
        } else {
            reference_root
        };
        let floating = floating.as_web_event();
        let middleware: Vec<Box<dyn Middleware<web_sys::Element, web_sys::Window>>> = vec![
            Box::new(Offset::new(OffsetOptions::Value(DEFAULT_OFFSET_GAP))),
            Box::new(Flip::new(FlipOptions::default())),
            Box::new(Shift::new(ShiftOptions::default())),
        ];
        let computed_position = compute_position(
            (&reference).into(),
            &floating,
            floating_ui_dom::ComputePositionConfig {
                placement: Some(placement_from(requested_side, requested_align)),
                middleware: Some(middleware),
                strategy: Some(Strategy::Fixed),
            },
        );
        let measured_width = reference.get_bounding_client_rect().width();
        measured_reference_width.set(Some(measured_width));
        let position_style = format!(
            "position: fixed; top: {}px; left: {}px;",
            computed_position.y, computed_position.x
        );
        style.set(position_style);
        let (resolved_side, resolved_align) = placement_to(computed_position.placement);
        side.set(resolved_side);
        align.set(resolved_align);
        is_positioned.set(true);
    });
    let mounted_cleanup = cleanup.clone();
    let on_mounted = use_callback(move |mounted: Rc<MountedData>| {
        floating.set(Some(mounted));
        if let Some(cleanup) = mounted_cleanup.take() {
            cleanup();
        }
        let Some(reference) = reference() else {
            return;
        };
        let Some(floating) = floating() else {
            return;
        };
        let reference_root = reference.as_web_event();
        let reference = if let Some(selector) = target_selector.as_deref() {
            let Ok(Some(target)) = reference_root.query_selector(selector) else {
                return;
            };
            target
        } else {
            reference_root
        };
        let floating = floating.as_web_event();
        mounted_cleanup.replace(Some(auto_update(
            (&reference).into(),
            &floating,
            Rc::new(move || position.call(())),
            AutoUpdateOptions::default().animation_frame(true),
        )));
        position.call(());
    });
    PositionState {
        style: use_memo(move || style()),
        on_mounted,
        side: use_memo(move || side()),
        align: use_memo(move || align()),
        is_positioned: use_memo(move || is_positioned()),
        floating_active: true,
        reference_width: use_memo(move || measured_reference_width()),
    }
}
/// are gated to wasm, so this build does no coordinate math and names no floating-ui
/// symbol — positioning stays CSS-driven via `data-side`/`data-align`. See the wasm
/// implementation for full documentation.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn use_position(
    _reference: Signal<Option<Rc<MountedData>>>,
    _floating: Signal<Option<Rc<MountedData>>>,
    requested_side: ContentSide,
    requested_align: ContentAlign,
    _target_selector: Option<String>,
) -> PositionState {
    let style = use_memo(String::new);
    let side = use_memo(move || requested_side);
    let align = use_memo(move || requested_align);
    let is_positioned = use_memo(|| true);
    PositionState {
        style,
        on_mounted: use_callback(|_: Rc<MountedData>| {}),
        side,
        align,
        is_positioned,
        reference_width: use_memo(|| None),
        floating_active: false,
    }
}

/// Extract a single CSS property value from a `prop: value; prop: value;` style string.
///
/// Returns an empty string if the property is absent, so the resulting `style:` prop is
/// a no-op (dioxus skips empty style values). Used to split the floating-ui inline style
/// string into individual `style:` props so user-forwarded styles are preserved while
/// only the same-named floating coordinate props are overridden.
pub(crate) fn style_prop(style: &str, prop: &str) -> String {
    style
        .split(';')
        .filter_map(|decl| decl.split_once(':'))
        .find(|(name, _)| name.trim() == prop)
        .map(|(_, value)| value.trim().to_string())
        .unwrap_or_default()
}

/// Map a requested ([`ContentSide`], [`ContentAlign`]) to a floating-ui placement.
///
/// [`ContentAlign::Center`] maps to the un-suffixed placement (e.g. `Top`); `Start`
/// and `End` map to the `*Start`/`*End` variants.
#[cfg(target_family = "wasm")]
fn placement_from(side: ContentSide, align: ContentAlign) -> floating_ui_dioxus::Placement {
    use floating_ui_dioxus::Placement;
    match (side, align) {
        (ContentSide::Top, ContentAlign::Center) => Placement::Top,
        (ContentSide::Top, ContentAlign::Start) => Placement::TopStart,
        (ContentSide::Top, ContentAlign::End) => Placement::TopEnd,
        (ContentSide::Right, ContentAlign::Center) => Placement::Right,
        (ContentSide::Right, ContentAlign::Start) => Placement::RightStart,
        (ContentSide::Right, ContentAlign::End) => Placement::RightEnd,
        (ContentSide::Bottom, ContentAlign::Center) => Placement::Bottom,
        (ContentSide::Bottom, ContentAlign::Start) => Placement::BottomStart,
        (ContentSide::Bottom, ContentAlign::End) => Placement::BottomEnd,
        (ContentSide::Left, ContentAlign::Center) => Placement::Left,
        (ContentSide::Left, ContentAlign::Start) => Placement::LeftStart,
        (ContentSide::Left, ContentAlign::End) => Placement::LeftEnd,
    }
}

/// Map a resolved floating-ui placement back to a
/// ([`ContentSide`], [`ContentAlign`]) pair for `data-side`/`data-align`.
#[cfg(target_family = "wasm")]
fn placement_to(placement: floating_ui_dioxus::Placement) -> (ContentSide, ContentAlign) {
    use floating_ui_dioxus::Placement;
    match placement {
        Placement::Top => (ContentSide::Top, ContentAlign::Center),
        Placement::TopStart => (ContentSide::Top, ContentAlign::Start),
        Placement::TopEnd => (ContentSide::Top, ContentAlign::End),
        Placement::Right => (ContentSide::Right, ContentAlign::Center),
        Placement::RightStart => (ContentSide::Right, ContentAlign::Start),
        Placement::RightEnd => (ContentSide::Right, ContentAlign::End),
        Placement::Bottom => (ContentSide::Bottom, ContentAlign::Center),
        Placement::BottomStart => (ContentSide::Bottom, ContentAlign::Start),
        Placement::BottomEnd => (ContentSide::Bottom, ContentAlign::End),
        Placement::Left => (ContentSide::Left, ContentAlign::Center),
        Placement::LeftStart => (ContentSide::Left, ContentAlign::Start),
        Placement::LeftEnd => (ContentSide::Left, ContentAlign::End),
    }
}
