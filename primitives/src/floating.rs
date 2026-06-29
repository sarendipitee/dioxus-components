//! Shared floating-element positioning for overlay primitives.
//!
//! This module exposes a single internal abstraction, [`use_position`], reused by
//! every overlay primitive (popover, tooltip, hover card, menus, select, combobox)
//! so the wasm-gating, middleware configuration, leak-safe reposition wiring, and the
//! [`Placement`](floating_ui_dioxus::Placement) ↔ [`ContentSide`]/[`ContentAlign`]
//! mapping live in exactly one place.
//!
//! On the web (`cfg(target_family = "wasm")`) it delegates to
//! [`floating_ui_dioxus::use_floating`] for real collision handling (offset + flip +
//! shift). On native/desktop builds the floating-ui crates are absent, so the hook is
//! inert and preserves the existing CSS-only `data-side`/`data-align` behavior. Both
//! implementations share the same signature and return the same dioxus-core-only
//! [`PositionState`], so call sites are unconditional within a target.
//!
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
    /// The side the content resolved to *after* flip — feed back into `data-side`.
    pub side: Memo<ContentSide>,
    /// The alignment the content resolved to after flip — feed back into `data-align`.
    pub align: Memo<ContentAlign>,
    /// Whether the floating element has been positioned at least once. Callers use
    /// this to keep the element `visibility:hidden` until first compute, avoiding a
    /// flash at the origin. Reliable on web: it OR-s upstream's `is_positioned` with a
    /// monotonic latch the hook sets once both refs are mounted and a compute has run,
    /// so it flips true on the first open even when upstream's `reset` zeroes its own
    /// flag in the same flush; always `true` on native. Callers may therefore guard
    /// visibility on this flag alone — no coordinate-presence fallback is needed.
    pub is_positioned: Memo<bool>,
    /// Whether floating-ui is the active positioning engine for this overlay.
    ///
    /// A plain compile-time `bool` (not a signal): `true` in the wasm implementation
    /// where floating-ui emits inline coordinates, `false` in the inert native
    /// implementation where positioning stays CSS-driven. Callers emit it as a
    /// `data-floating` marker attribute so a single stylesheet can serve both
    /// targets: native fallback positional rules are scoped under
    /// `:not([data-floating])` and stay inert on web. Keeping it a core `bool` lets
    /// both impls return the same struct without the native build naming any
    /// floating-ui symbol.
    pub floating_active: bool,
}

/// Position a floating overlay element relative to a reference (trigger) element.
///
/// Returns a [`PositionState`] whose memos drive the floating element's inline
/// `style`, resolved `data-side`/`data-align`, and an until-positioned visibility
/// guard.
///
/// On the web this wires [`floating_ui_dioxus::use_floating`] with
/// `offset`/`flip`/`shift` middleware (using [`DEFAULT_OFFSET_GAP`]), an absolute
/// positioning strategy, and `transform: false` (it writes `top`/`left`, never
/// `transform`, so CSS animation transforms are not clobbered). Repositioning on
/// scroll/resize is wired manually with teardown (no `while_elements_mounted` /
/// `auto_update`, to avoid the known unmount leak). On native the hook is inert and
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
    floating: Signal<Option<Rc<MountedData>>>,
    requested_side: ContentSide,
    requested_align: ContentAlign,
) -> PositionState {
    use floating_ui_dioxus::{use_floating, UseFloatingOptions};
    use floating_ui_dom::{
        Flip, FlipOptions, Offset, OffsetOptions, Shift, ShiftOptions, Strategy,
    };

    let placement = placement_from(requested_side, requested_align);

    let middleware: Vec<Box<dyn floating_ui_dom::Middleware<web_sys::Element, web_sys::Window>>> = vec![
        Box::new(Offset::new(OffsetOptions::Value(DEFAULT_OFFSET_GAP))),
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];

    let floating_return = use_floating(
        reference,
        floating,
        UseFloatingOptions::default()
            .placement(placement)
            .strategy(Strategy::Absolute)
            .transform(false)
            // `open(true)` makes upstream set `is_positioned.set(open)` = true after each
            // compute (and after scroll/resize repositions), so the flag is meaningful
            // for the always-mounted overlays. It is NOT sufficient on its own for the
            // first open of mount-on-open overlays — upstream's `reset` effect can zero
            // it again in the same flush — which is why we also keep our own `positioned`
            // latch (see below). The overlay is only mounted while open, so true is always
            // the correct value here.
            .open(true)
            .middleware(middleware),
    );

    let floating_styles = floating_return.floating_styles;
    let returned_placement = floating_return.placement;
    let returned_is_positioned = floating_return.is_positioned;
    let update = floating_return.update;

    // Our own monotonic "positioned" latch, OR-ed into the exposed `is_positioned`
    // below. Upstream's `is_positioned` is unreliable on the first open of an overlay
    // whose floating element mounts in response to opening (Popover/Tooltip/HoverCard):
    // `use_floating` registers three effects — compute, attach(→compute), and `reset`
    // (which calls `is_positioned.set(false)`). When the floating ref settles *before*
    // the first effect flush, all three run in one flush; the compute effects set
    // `is_positioned = true` and then `reset` clobbers it back to `false` in the same
    // flush, with nothing re-setting it until a scroll/resize fires `update` again.
    // (Always-mounted menus/Select/Combobox settle their ref a render later, so `reset`
    // does not re-run after the compute and they are unaffected.) Rather than depend on
    // effect-ordering to out-race `reset`, we latch our own flag the first time both
    // refs are mounted and a compute has run — see the effect below.
    let mut positioned = use_signal(|| false);

    // Manual reposition on scroll (capture phase) + resize, with teardown. We
    // deliberately avoid `while_elements_mounted`/`auto_update` (known unmount leak);
    // instead a JS listener pings back and we call `update` on the dioxus side.
    //
    // The listener is only established while BOTH refs are mounted (i.e. the overlay
    // is actually open and both DOM nodes exist). Overlays that mount this hook while
    // closed (DropdownMenuContent/MenubarContent) leave the floating ref None, so no
    // idle listener fires `update` on every scroll. The effect reacts to the refs
    // becoming Some/None and tears the listener down when they go back to None.
    //
    // Stop protocol: JS sends `1` for a reposition event and the recv loop calls
    // `update` only for `1`. Teardown sends the `0` sentinel, which breaks the loop
    // without calling `update` (no spurious reposition on close) and lets the JS side
    // remove its listeners. This avoids the previous protocol where `true` meant both
    // "event happened" and "stop", which fired a spurious `update.call(())` on unmount.
    //
    // Initial forced update + positioned latch: when both refs first become Some, we
    // call `update` once immediately so the floating coordinates are computed for the
    // current geometry, then latch `positioned = true` so the exposed `is_positioned`
    // becomes (and stays) true on the first open even if upstream's `reset` clobbers
    // its own `is_positioned` in the same flush (see the `positioned` declaration). The
    // latch and forced call live inside the `mounted` branch, so they fire only when
    // both refs are present (open), not on close (when the branch returns early). This
    // adds no listener and does not regress the leak-free listener lifecycle.
    crate::use_effect_with_cleanup(move || {
        let mounted = reference().is_some() && floating().is_some();
        if !mounted {
            // No listener while either ref is None; nothing to tear down.
            return Box::new(|| {}) as Box<dyn FnOnce()>;
        }

        // Force a compute for the current geometry, then latch positioned. The forced
        // `update` runs before the latch, so by the time `positioned` flips true the
        // coordinates already reflect the trigger (no flash at the origin). For
        // always-mounted overlays this is a harmless redundant compute.
        update.call(());
        positioned.set(true);

        let mut eval = document::eval(
            "function listener() { dioxus.send(1); }
            window.addEventListener('scroll', listener, true);
            window.addEventListener('resize', listener);
            // `0` is the stop sentinel; any other value is a reposition ping.
            while ((await dioxus.recv()) !== 0) {}
            window.removeEventListener('scroll', listener, true);
            window.removeEventListener('resize', listener);",
        );
        // `Eval` is `Copy`, so the teardown closure holds its own handle to send the
        // stop sentinel even though `recv` (in the spawned task) needs `&mut`.
        let stop = eval;
        let update = update;
        spawn(async move {
            // Only an explicit reposition ping (`1`) triggers `update`; the `0`
            // teardown sentinel ends the loop without repositioning.
            while let Ok(1) = eval.recv::<i32>().await {
                update.call(());
            }
        });
        Box::new(move || {
            let _ = stop.send(0);
        }) as Box<dyn FnOnce()>
    });

    let style = use_memo(move || floating_styles().to_string());
    let side = use_memo(move || placement_to(returned_placement()).0);
    let align = use_memo(move || placement_to(returned_placement()).1);
    // Expose upstream's flag OR our latch: either becoming true means "positioned".
    // The latch guarantees the first open flips visible without a scroll/resize even
    // when upstream's `reset` zeroes its own `is_positioned` in the first flush.
    let is_positioned = use_memo(move || returned_is_positioned() || positioned());

    PositionState {
        style,
        side,
        align,
        is_positioned,
        floating_active: true,
    }
}

/// Inert native/desktop implementation of [`use_position`]. The floating-ui crates
/// are gated to wasm, so this build does no coordinate math and names no floating-ui
/// symbol — positioning stays CSS-driven via `data-side`/`data-align`. See the wasm
/// implementation for full documentation.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn use_position(
    _reference: Signal<Option<Rc<MountedData>>>,
    _floating: Signal<Option<Rc<MountedData>>>,
    requested_side: ContentSide,
    requested_align: ContentAlign,
) -> PositionState {
    let style = use_memo(String::new);
    let side = use_memo(move || requested_side);
    let align = use_memo(move || requested_align);
    let is_positioned = use_memo(|| true);

    PositionState {
        style,
        side,
        align,
        is_positioned,
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

/// Map a requested ([`ContentSide`], [`ContentAlign`]) to a floating-ui
/// [`Placement`](floating_ui_dioxus::Placement).
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

/// Map a resolved floating-ui [`Placement`](floating_ui_dioxus::Placement) back to a
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
