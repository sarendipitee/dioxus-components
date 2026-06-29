//! A unified, root-level overlay manager.
//!
//! Every overlay surface (Dialog, Sheet, Popover, Menu, Select, Tooltip,
//! HoverCard, Toast) can register a single [`OverlayEntry`] with a root-scope
//! [`OverlayCtx`] and render its subtree through one shared [`OverlayOutlet`] at
//! the top of the document. The manager centralizes:
//!
//! - **z-index assignment** by open order, banded per [`OverlayKind`]
//!   (modal/sheet/floating/hint share one band, toasts ride above).
//! - **A single dismiss stack** (one Escape listener + one outside-pointerdown
//!   listener) that targets the topmost `dismissable` entry, using the §4.1
//!   union-of-subtrees "inside" predicate so a portaled panel and its descendant
//!   panels are all treated as "inside".
//! - **Ref-counted scroll-lock** that locks `document.body` overflow on the
//!   first `modal` entry and unlocks on the last.
//! - **A topmost-only focus-trap coordinator** plus explicit manager-tracked
//!   return-focus restoration (the vendored focus-trap has no pause/unpause).
//! - **Depth / stack metadata** for push-aside animations, excluding entries in
//!   their exit (`closing`) phase.
//!
//! ## Context does NOT inherit through the portal
//!
//! Dioxus resolves context up the *render* tree (where [`PortalOut`] sits), not
//! the *definition* tree. A context provided above a registering component is
//! therefore invisible to that component's portaled content. Each overlay must
//! **re-provide** every context its portaled descendants consume, inside the
//! portaled subtree. See [`OverlayOutlet`] and the smoke test in `preview/` for
//! the load-bearing pattern, mirroring `toast.rs`'s `ToastRenderCtx`.

use crate::dioxus_core::provide_root_context;
use crate::portal::{PortalId, PortalOut};
use crate::{use_effect_cleanup, FOCUS_TRAP_JS};
use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A unique, monotonic overlay identifier. Never reused for the lifetime of the
/// program (mirrors the toast `NEXT_ID` pattern) so a stale dismiss / timer can
/// never match a re-opened overlay, and outlet keys stay stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OverlayId(usize);

impl OverlayId {
    fn next() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        OverlayId(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// The raw numeric id. Useful as a stable rsx `key`.
    pub fn raw(&self) -> usize {
        self.0
    }
}

/// The behavioral class of an overlay. Drives the z-band, whether the entry
/// participates in the dismiss stack / focus trap / scroll lock, and the
/// push-aside depth math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayKind {
    /// Dialog / AlertDialog: backdrop + centered content, focus trap, scroll lock.
    Modal,
    /// Sheet / Drawer: backdrop + edge-docked content, focus trap, scroll lock.
    Sheet,
    /// Popover / Menu / Select / ContextMenu / Combobox: anchored, dismiss-on-outside.
    Floating,
    /// Tooltip / HoverCard: anchored, non-focus-trapping, never in the dismiss stack.
    Hint,
    /// Notifications: dedicated top band, pointer-events scoped.
    Toast,
}

impl OverlayKind {
    /// Whether this kind participates in modal/sheet depth (push-aside) math.
    fn is_layered(&self) -> bool {
        matches!(self, OverlayKind::Modal | OverlayKind::Sheet)
    }

    /// The z-index band base CSS variable for this kind.
    fn band_var(&self) -> &'static str {
        match self {
            OverlayKind::Toast => "--z-toast-base",
            _ => "--z-overlay-base",
        }
    }

    /// A coarse band rank used to sort entries across kinds in the outlet:
    /// non-toast overlays first (lower paint), toasts last (above).
    fn band_rank(&self) -> u8 {
        match self {
            OverlayKind::Toast => 1,
            _ => 0,
        }
    }
}

/// A single live overlay registered with the manager.
///
/// Cloned freely: every field is `Copy`/`'static` (signals, callbacks) or cheap.
#[derive(Clone)]
pub struct OverlayEntry {
    /// Stable, monotonic id. Also the outlet `key`.
    pub id: OverlayId,
    /// Behavioral class. Drives band / dismiss / focus / depth.
    pub kind: OverlayKind,
    /// The portal carrying this entry's content subtree (its `PortalIn`).
    pub portal: PortalId,
    /// Participates in scroll-lock + focus-trap + backdrop dimming.
    pub modal: bool,
    /// Participates in Escape / outside-click top-of-stack dismissal.
    pub dismissable: bool,
    /// Called when the manager selects this entry as the dismiss target.
    pub on_dismiss: Callback<()>,
    /// The overlay this one was opened from, for depth + the "inside" predicate.
    pub parent: Option<OverlayId>,
    /// Assigned at registration; defines z-order within the band.
    pub order: u64,
    /// True while the entry is animating out. Excluded from depth/stack math so
    /// siblings reflow immediately (mirrors toast `!removing`). Keeps its z slot.
    pub closing: bool,
    /// DOM id of the trigger element subtree (for the union "inside" predicate).
    pub trigger_id: Option<String>,
    /// DOM id of the portaled content root (for the union "inside" predicate).
    pub content_root_id: Option<String>,
    /// Optional grouping key for kind-specific stack effects.
    pub stack_key: Option<String>,
}

/// Arguments to [`OverlayCtx::register`]. Bundles the immutable registration
/// inputs; mutable per-frame state (`closing`, ids) is updated via dedicated
/// callbacks so registration stays a single atomic call.
#[derive(Clone)]
pub struct RegisterArgs {
    /// Behavioral class.
    pub kind: OverlayKind,
    /// The portal carrying this entry's content subtree.
    pub portal: PortalId,
    /// Participates in scroll-lock + focus-trap + backdrop dimming.
    pub modal: bool,
    /// Participates in the dismiss stack.
    pub dismissable: bool,
    /// Dismiss callback.
    pub on_dismiss: Callback<()>,
    /// Parent overlay, if opened from inside another overlay.
    pub parent: Option<OverlayId>,
    /// Trigger element DOM id, if known at registration.
    pub trigger_id: Option<String>,
    /// Portaled content root DOM id, if known at registration.
    pub content_root_id: Option<String>,
    /// Optional grouping key for kind-specific stack effects.
    pub stack_key: Option<String>,
}

/// Root-scope overlay manager state. Provided once by [`OverlayProvider`] (or
/// lazily auto-provisioned, mirroring `portal.rs`). Cheap `Copy`.
#[derive(Clone, Copy, PartialEq)]
pub struct OverlayCtx {
    /// The ordered set of live overlay entries.
    entries: Signal<Vec<OverlayEntry>>,
    /// Monotonic order counter used to assign `OverlayEntry::order`. Retained on
    /// the ctx so future helpers (e.g. re-ordering on re-open) can reach it.
    #[allow(dead_code)]
    next_order: Signal<u64>,
    register: Callback<RegisterArgs, OverlayId>,
    unregister: Callback<OverlayId>,
    set_closing: Callback<(OverlayId, bool)>,
    set_dom_ids: Callback<(OverlayId, Option<String>, Option<String>)>,
    set_stack_key: Callback<(OverlayId, Option<String>)>,
}

impl OverlayCtx {
    /// Register a new overlay entry and return its assigned id.
    pub fn register(&self, args: RegisterArgs) -> OverlayId {
        self.register.call(args)
    }

    /// Remove an overlay entry (after its exit animation completes).
    pub fn unregister(&self, id: OverlayId) {
        self.unregister.call(id);
    }

    /// Mark / clear the entry's `closing` (exit) phase. Excludes it from depth
    /// math immediately while keeping its z slot so it animates out above lower
    /// layers (§4.5 exit-phase exclusion).
    pub fn set_closing(&self, id: OverlayId, closing: bool) {
        self.set_closing.call((id, closing));
    }

    /// Update the trigger / content-root DOM ids used by the "inside" predicate
    /// once the elements have mounted.
    pub fn set_dom_ids(
        &self,
        id: OverlayId,
        trigger_id: Option<String>,
        content_root_id: Option<String>,
    ) {
        self.set_dom_ids.call((id, trigger_id, content_root_id));
    }

    /// Update the optional grouping key used by kind-specific stack effects.
    pub fn set_stack_key(&self, id: OverlayId, stack_key: Option<String>) {
        self.set_stack_key.call((id, stack_key));
    }

    /// A reactive read of all entries (subscribes the caller).
    pub fn entries(&self) -> Signal<Vec<OverlayEntry>> {
        self.entries
    }

    /// The computed z-index expression for an entry, as a CSS `calc(...)` using
    /// the band base + step CSS variables. Read this reactively to set
    /// `--overlay-z` on the portaled content.
    pub fn z_for(&self, id: OverlayId) -> Option<String> {
        let entries = self.entries.read();
        let entry = entries.iter().find(|e| e.id == id)?;
        // Position within the band, by order, including closing entries so an
        // exiting overlay keeps its paint slot above lower layers.
        let mut band: Vec<&OverlayEntry> = entries
            .iter()
            .filter(|e| e.kind.band_rank() == entry.kind.band_rank())
            .collect();
        band.sort_by_key(|e| e.order);
        let slot = band.iter().position(|e| e.id == id)? as i64;
        Some(format!(
            "calc(var({base}) + {slot} * var(--z-overlay-step))",
            base = entry.kind.band_var(),
            slot = slot,
        ))
    }

    /// Number of layered (modal/sheet) `!closing` entries stacked *above* the
    /// given entry. `0` == topmost. Used for `data-overlay-depth` push-aside.
    ///
    /// Parent-mid-exit rule: depth math is order-based, not parent-based, so a
    /// dangling/closing parent does not affect this count. Entries are counted
    /// purely by `order` and `!closing`, which is always well-defined.
    pub fn depth_above(&self, id: OverlayId) -> usize {
        let entries = self.entries.read();
        let Some(entry) = entries.iter().find(|e| e.id == id) else {
            return 0;
        };
        if !entry.kind.is_layered() {
            return 0;
        }
        entries
            .iter()
            .filter(|e| e.kind.is_layered() && !e.closing && e.order > entry.order)
            .count()
    }

    /// Number of live sheets stacked above this sheet with the same stack key.
    /// Used by direction-aware sheet depth styling.
    pub fn same_stack_sheet_depth_above(&self, id: OverlayId) -> usize {
        let entries = self.entries.read();
        let Some(entry) = entries.iter().find(|e| e.id == id) else {
            return 0;
        };
        if entry.kind != OverlayKind::Sheet {
            return 0;
        }
        let Some(stack_key) = entry.stack_key.as_deref() else {
            return 0;
        };
        entries
            .iter()
            .filter(|e| {
                e.kind == OverlayKind::Sheet
                    && !e.closing
                    && e.order > entry.order
                    && e.stack_key.as_deref() == Some(stack_key)
            })
            .count()
    }

    /// Total count of layered (modal/sheet) `!closing` entries. Used for
    /// `data-overlay-stack-size`.
    ///
    /// Parent-mid-exit rule: closing entries are excluded from this count
    /// immediately so siblings reflow push-aside transforms without waiting for
    /// the exit animation to complete. Orphaned children (whose parent is
    /// closing or already unregistered) are counted at face value — they are
    /// still live, non-closing entries and form the new top of the visual stack.
    pub fn layered_stack_size(&self) -> usize {
        self.entries
            .read()
            .iter()
            .filter(|e| e.kind.is_layered() && !e.closing)
            .count()
    }
}

/// Read the root [`OverlayCtx`], lazily auto-provisioning it at the root scope if
/// no [`OverlayProvider`] has provided one yet (mirrors `portal.rs`). Note: the
/// [`OverlayOutlet`] still has to be mounted once via [`OverlayProvider`] for any
/// content to render.
pub fn use_overlay() -> OverlayCtx {
    use_hook(consume_overlay)
}

fn consume_overlay() -> OverlayCtx {
    if let Some(ctx) = try_consume_context::<OverlayCtx>() {
        return ctx;
    }
    provide_root_overlay()
}

fn provide_root_overlay() -> OverlayCtx {
    let mut entries: Signal<Vec<OverlayEntry>> = Signal::new_in_scope(Vec::new(), ScopeId::ROOT);
    let mut next_order: Signal<u64> = Signal::new_in_scope(0, ScopeId::ROOT);

    let register = Callback::new(move |args: RegisterArgs| {
        let id = OverlayId::next();
        let order = {
            let mut n = next_order.write();
            let order = *n;
            *n += 1;
            order
        };
        entries.write().push(OverlayEntry {
            id,
            kind: args.kind,
            portal: args.portal,
            modal: args.modal,
            dismissable: args.dismissable,
            on_dismiss: args.on_dismiss,
            parent: args.parent,
            order,
            closing: false,
            trigger_id: args.trigger_id,
            content_root_id: args.content_root_id,
            stack_key: args.stack_key,
        });
        id
    });

    let unregister = Callback::new(move |id: OverlayId| {
        entries.write().retain(|e| e.id != id);
    });

    let set_closing = Callback::new(move |(id, closing): (OverlayId, bool)| {
        if let Some(e) = entries.write().iter_mut().find(|e| e.id == id) {
            e.closing = closing;
        }
    });

    let set_dom_ids = Callback::new(
        move |(id, trigger_id, content_root_id): (OverlayId, Option<String>, Option<String>)| {
            if let Some(e) = entries.write().iter_mut().find(|e| e.id == id) {
                e.trigger_id = trigger_id;
                e.content_root_id = content_root_id;
            }
        },
    );

    let set_stack_key = Callback::new(move |(id, stack_key): (OverlayId, Option<String>)| {
        if let Some(e) = entries.write().iter_mut().find(|e| e.id == id) {
            e.stack_key = stack_key;
        }
    });

    provide_root_context(OverlayCtx {
        entries,
        next_order,
        register,
        unregister,
        set_closing,
        set_dom_ids,
        set_stack_key,
    })
}

/// Compute the topmost `dismissable`, `!closing` entry's id (highest order).
/// `Hint` and `Toast` entries never participate — hints are non-interactive;
/// toasts have their own timeout/swipe dismissal and must not block Escape from
/// reaching the modal beneath.
fn topmost_dismissable(entries: &[OverlayEntry]) -> Option<OverlayId> {
    entries
        .iter()
        .filter(|e| {
            e.dismissable && !e.closing && !matches!(e.kind, OverlayKind::Hint | OverlayKind::Toast)
        })
        .max_by_key(|e| e.order)
        .map(|e| e.id)
}

/// Build the union of "inside" element ids for the topmost dismissable entry:
/// its own trigger + content root, plus every descendant entry's trigger +
/// content root (via `parent` linkage). A pointerdown inside any of these must
/// NOT dismiss the entry (§4.1 union predicate).
///
/// Parent-mid-exit rule (§4.5): if an entry's `parent` id is not present in
/// the current entry set (dangling — already unregistered or closing), the
/// child is treated as orphaned at the top level. This keeps `included`
/// expansion terminating and prevents a stale parent from silently cutting off
/// the child's subtree from the inside check.
fn inside_ids_for(entries: &[OverlayEntry], target: OverlayId) -> Vec<String> {
    // Collect the target plus all transitive descendants.  We skip children
    // whose parent no longer exists in the list (orphan/dangling parent) so
    // depth math stays correct even when a parent finishes unmounting while
    // children are still live.
    let live_ids: std::collections::HashSet<OverlayId> = entries.iter().map(|e| e.id).collect();

    let mut included = vec![target];
    let mut changed = true;
    while changed {
        changed = false;
        for e in entries {
            if let Some(parent) = e.parent {
                // Only traverse the parent link if the parent is still live.
                if live_ids.contains(&parent)
                    && included.contains(&parent)
                    && !included.contains(&e.id)
                {
                    included.push(e.id);
                    changed = true;
                }
            }
        }
    }
    let mut ids = Vec::new();
    for e in entries.iter().filter(|e| included.contains(&e.id)) {
        if let Some(t) = &e.trigger_id {
            ids.push(t.clone());
        }
        if let Some(c) = &e.content_root_id {
            ids.push(c.clone());
        }
    }
    ids
}

/// The root overlay provider. Provides [`OverlayCtx`] and mounts the single
/// [`OverlayOutlet`]. Wrap your app once: `OverlayProvider { App {} }`.
///
/// Auto-provisioning via [`use_overlay`] removes the *context* requirement, but
/// the outlet must still be mounted somewhere in the tree — that is what this
/// component guarantees.
#[component]
pub fn OverlayProvider(children: Element) -> Element {
    // Provide (or reuse) the root ctx at this scope so descendants and the
    // outlet share it.
    let ctx = use_hook(|| match try_consume_context::<OverlayCtx>() {
        Some(ctx) => ctx,
        None => provide_root_overlay(),
    });
    // Ensure the ctx is also visible directly at this scope for children that
    // consume it via plain `use_context` rather than `use_overlay`.
    use_context_provider(|| ctx);

    // Drive the central dismiss stack + scroll lock + focus trap from one place.
    use_overlay_dismiss_stack();
    use_overlay_scroll_lock();
    use_overlay_focus_trap();

    rsx! {
        // Load the focus-trap helper WITHOUT defer so it is synchronously
        // available before any overlay effect fires. Using defer=true caused a
        // race: the effect could run before the script loaded, silently
        // producing no trap on first open.
        document::Script { src: FOCUS_TRAP_JS }
        {children}
        OverlayOutlet {}
    }
}

/// The single portal outlet. Iterates live entries sorted by (band, order) and
/// emits one keyed [`PortalOut`] per entry, so reordering never remounts a live
/// overlay (preserves animation + focus state).
///
/// Note: the portaled content itself is supplied by each overlay's `PortalIn`
/// and must **re-provide** any context its descendants consume — context does
/// not inherit through the portal (see module docs).
#[component]
pub fn OverlayOutlet() -> Element {
    let ctx = use_overlay();
    let entries = ctx.entries();

    let ordered = use_memo(move || {
        // Single read: collect both the (id, portal) list and the sort key in
        // one pass to avoid holding two simultaneous borrows of the same Signal.
        let guard = entries.read();
        let mut list: Vec<(OverlayId, PortalId, u8, u64)> = guard
            .iter()
            .map(|e| (e.id, e.portal, e.kind.band_rank(), e.order))
            .collect();
        drop(guard);
        list.sort_by_key(|(_, _, band, order)| (*band, *order));
        list.into_iter()
            .map(|(id, portal, _, _)| (id, portal))
            .collect::<Vec<_>>()
    });

    rsx! {
        for (id , portal) in ordered() {
            PortalOut { key: "{id.raw()}", portal }
        }
    }
}

/// Owns the single Escape listener and the single outside-pointerdown listener.
/// On either trigger it finds the topmost `dismissable` entry and calls its
/// `on_dismiss`, honoring the union "inside" predicate for pointer events.
///
/// Gated like the existing per-component listeners: runs via `document::eval`,
/// which is a runtime no-op off-client (SSR) and works on web + desktop.
fn use_overlay_dismiss_stack() {
    let ctx = use_overlay();
    let entries = ctx.entries();

    // Escape: dismiss topmost dismissable.
    // IMPORTANT: resolve the callback and drop the read guard BEFORE calling it.
    // If on_dismiss calls back into unregister() -> entries.write(), holding the
    // read guard alive causes a RefCell borrow-conflict panic.
    crate::use_global_keydown_listener("Escape", move || {
        let cb = {
            let list = entries.read();
            topmost_dismissable(&list)
                .and_then(|id| list.iter().find(|e| e.id == id).map(|e| e.on_dismiss))
            // read guard drops here
        };
        if let Some(cb) = cb {
            cb.call(());
        }
    });

    // Outside pointerdown: dismiss topmost dismissable if the press lands
    // outside the union of its (and descendants') trigger + content subtrees.
    use_overlay_outside_dismiss(ctx);
}

/// Outside-pointerdown dismissal for the topmost dismissable entry, using the
/// union-of-subtrees predicate. A single long-lived JS listener sends the click
/// target id back; the Rust side resolves the current topmost entry + inside set
/// and decides. The JS reports the closest element id under the pointer so the
/// Rust side can run the union containment test against the live entry set
/// (entries change between presses, so the test must be re-evaluated per event).
///
/// Teardown protocol (mirrors floating.rs sentinel pattern): the JS side `await
/// dioxus.recv()` blocks until the Rust cleanup closure sends a value (here
/// `true`). On receipt the JS removes the pointerdown listener and exits, which
/// causes the async Recv to resolve as Err, ending the Rust `while` loop. This
/// guarantees the capture-phase listener is removed when the effect re-runs or
/// the component unmounts, with no lingering listener leak.
fn use_overlay_outside_dismiss(ctx: OverlayCtx) {
    let entries = ctx.entries();
    crate::use_effect_with_cleanup(move || {
        let mut eval = document::eval(
            r#"
            // Report, for each pointerdown (capture phase), the list of element
            // ids on the path from the target up to <body>. The Rust side tests
            // its union "inside" set against this path.
            const onPointer = e => {
                const ids = [];
                let node = e.target;
                while (node && node !== document.body && node !== document.documentElement) {
                    if (node.id) ids.push(node.id);
                    node = node.parentElement;
                }
                dioxus.send(ids);
            };
            document.addEventListener('pointerdown', onPointer, true);
            await dioxus.recv();
            document.removeEventListener('pointerdown', onPointer, true);
            "#,
        );
        spawn(async move {
            while let Ok(path_ids) = eval.recv::<Vec<String>>().await {
                // IMPORTANT: resolve the callback and drop the read guard BEFORE
                // calling it. If on_dismiss calls back into unregister() ->
                // entries.write(), holding the read guard alive causes a RefCell
                // borrow-conflict panic.
                let cb = {
                    let list = entries.read();
                    let Some(target) = topmost_dismissable(&list) else {
                        continue;
                    };
                    let inside = inside_ids_for(&list, target);
                    let is_inside = path_ids
                        .iter()
                        .any(|pid| inside.iter().any(|iid| iid == pid));
                    if is_inside {
                        None
                    } else {
                        list.iter().find(|e| e.id == target).map(|e| e.on_dismiss)
                    }
                    // read guard drops here
                };
                if let Some(cb) = cb {
                    cb.call(());
                }
            }
        });
        move || {
            let _ = eval.send(true);
        }
    });
}

/// Ref-counted scroll lock: lock `document.body` overflow when the count of
/// live (`!closing`) `modal` entries transitions 0->1, unlock on 1->0.
/// Generalizes the per-Dialog lock so nested modals don't double-lock or
/// early-unlock.
fn use_overlay_scroll_lock() {
    let ctx = use_overlay();
    let entries = ctx.entries();

    let modal_count = use_memo(move || {
        entries
            .read()
            .iter()
            .filter(|e| e.modal && !e.closing)
            .count()
    });

    crate::use_effect_with_cleanup(move || {
        let lock = modal_count() > 0;
        let eval = document::eval(
            "const lock = await dioxus.recv();
            document.body.style.overflow = lock ? 'hidden' : '';",
        );
        let _ = eval.send(lock);
        // Only clear overflow in cleanup when there are no more modal entries.
        // Without this guard, opening a second modal (count 1→2) triggers a
        // re-run of this effect: the previous run's cleanup fires before the new
        // run's eval, momentarily setting overflow='' then immediately back to
        // 'hidden' — a visible unlock flash. Gating on count==0 makes the
        // scroll-lock idempotent: the unlock only happens when the last modal
        // closes, never mid-stack.
        move || {
            if modal_count() == 0 {
                let _ = document::eval("document.body.style.overflow = '';");
            }
        }
    });
}

/// A per-overlay registration handle returned by [`use_overlay_registration`].
/// Cheap `Copy`. Exposes the assigned id, the manager ctx, and reactive helpers
/// the portaled content reads to set `--overlay-z` / depth data attributes.
#[derive(Clone, Copy, PartialEq)]
pub struct OverlayRegistration {
    /// The manager.
    pub ctx: OverlayCtx,
    /// This entry's assigned id (available after the first commit).
    id: Signal<Option<OverlayId>>,
}

impl OverlayRegistration {
    /// This entry's id, once registered.
    ///
    /// Uses a fallible peek: the backing signal is component-scoped, and the
    /// portaled content (which lives in the outlet's scope, not the registering
    /// component's) can render/read this handle once more *after* the registering
    /// component has unmounted and freed its signal. A plain read would then hit
    /// `ValueDroppedError` and panic the runtime. Degrading to `None` post-drop
    /// is correct: z/depth simply stop being applied to a node that is itself
    /// being torn down.
    pub fn id(&self) -> Option<OverlayId> {
        self.id.try_peek().ok().and_then(|v| *v)
    }

    /// The computed `--overlay-z` value for this entry, reactive.
    pub fn z(&self) -> Option<String> {
        self.id().and_then(|id| self.ctx.z_for(id))
    }

    /// `data-overlay-depth`: layered entries stacked above this one (0 = top).
    pub fn depth(&self) -> usize {
        self.id().map(|id| self.ctx.depth_above(id)).unwrap_or(0)
    }

    /// `data-overlay-stack-size`: total live layered entries.
    pub fn stack_size(&self) -> usize {
        self.ctx.layered_stack_size()
    }

    /// `data-overlay-sheet-depth`: same-direction sheets stacked above this one.
    pub fn sheet_depth(&self) -> usize {
        self.id()
            .map(|id| self.ctx.same_stack_sheet_depth_above(id))
            .unwrap_or(0)
    }

    /// Mark / clear the exit (`closing`) phase.
    pub fn set_closing(&self, closing: bool) {
        if let Some(id) = self.id() {
            self.ctx.set_closing(id, closing);
        }
    }

    /// Update trigger / content-root DOM ids for the dismiss predicate.
    pub fn set_dom_ids(&self, trigger_id: Option<String>, content_root_id: Option<String>) {
        if let Some(id) = self.id() {
            self.ctx.set_dom_ids(id, trigger_id, content_root_id);
        }
    }

    /// Update the kind-specific stack grouping key.
    pub fn set_stack_key(&self, stack_key: Option<String>) {
        if let Some(id) = self.id() {
            self.ctx.set_stack_key(id, stack_key);
        }
    }
}

/// Register an overlay entry for the lifetime of the calling component and
/// return an [`OverlayRegistration`] handle. The entry is registered on first
/// mount and unregistered automatically on unmount.
///
/// `args` is built once (in a `use_hook`-style closure) so registration is a
/// single atomic call; per-frame mutations (`closing`, dom ids) go through the
/// returned handle.
///
/// Wave 1 components pair this with `use_portal()` + `PortalIn { portal, ... }`
/// and **re-provide their own ctx inside the `PortalIn` children** (see module
/// docs / the R-CTX smoke demo).
pub fn use_overlay_registration(make_args: impl FnOnce() -> RegisterArgs) -> OverlayRegistration {
    let ctx = use_overlay();
    // Component-scoped signal so it frees on unmount (the previous
    // `Signal::new_in_scope(None, ScopeId::ROOT)` lived for the whole program,
    // leaking one signal per call). The id is *also* mirrored into a plain
    // `Rc<Cell<..>>` below so the unmount cleanup never has to read this signal:
    // by the time `use_drop` fires the signal may already be freed.
    let mut id_sig: Signal<Option<OverlayId>> = use_signal(|| None);

    // Non-reactive, signal-runtime-independent mirror of the id. Cloned into the
    // cleanup closure so unregister is guaranteed even after the component's
    // signals are dropped — preventing a stale entry leak in `entries`.
    let id_cell: std::rc::Rc<std::cell::Cell<Option<OverlayId>>> =
        use_hook(|| std::rc::Rc::new(std::cell::Cell::new(None)));

    use_hook(|| {
        let id = ctx.register(make_args());
        id_sig.set(Some(id));
        id_cell.set(Some(id));
    });

    {
        let id_cell = id_cell.clone();
        use_effect_cleanup(move || {
            // Read the plain Cell, never the (possibly-freed) signal.
            if let Some(id) = id_cell.get() {
                ctx.unregister(id);
            }
        });
    }

    OverlayRegistration { ctx, id: id_sig }
}

/// Topmost-only focus-trap coordinator (§4.1 M3).
///
/// The vendored `focus-trap.js` `FocusTrap` has no `pause()`/`unpause()`, so a
/// trap is *suspended* by `remove()` and *restored* by recreating it via
/// `createFocusTrap`. Because `remove()` refocuses the element captured at
/// construction, the manager tracks each modal's return-focus origin itself and
/// restores focus explicitly when a layer closes (rather than relying on the
/// trap's own restore).
///
/// Return-focus is maintained as a **stack** (`window.__dxOverlayFocusStack`):
/// each time a new topmost modal opens, the current `document.activeElement` is
/// pushed onto the stack; when a modal closes (topmost id changes or becomes
/// null), the stack top is popped and focused. This correctly handles nested
/// modals: closing the inner modal returns focus to the inner layer's origin,
/// not to wherever focus was before the outermost modal opened.
///
/// This effect uses a stable long-lived eval per run that receives setup/teardown
/// commands through `dioxus.recv()`. Proper teardown (sending `null`) ensures
/// each previous eval is cleaned up before a new one takes over, preventing
/// multiple eval contexts from accumulating across `top_modal_id` changes.
fn use_overlay_focus_trap() {
    let ctx = use_overlay();
    let entries = ctx.entries();

    // The content-root id of the topmost modal, if any.
    let top_modal_id = use_memo(move || {
        entries
            .read()
            .iter()
            .filter(|e| e.modal && !e.closing)
            .max_by_key(|e| e.order)
            .and_then(|e| e.content_root_id.clone())
    });

    // Stable long-lived eval channel: the effect sends the new topId each run
    // and the JS side updates traps accordingly. We use use_effect_with_cleanup
    // so the channel is explicitly torn down (by sending null) when the effect
    // re-runs or the component unmounts, preventing stale eval accumulation.
    crate::use_effect_with_cleanup(move || {
        let top = top_modal_id();
        let eval = document::eval(
            r#"
            // Focus-stack: push current activeElement on new topmost modal, pop
            // and restore when modal closes. This handles nested modals correctly
            // — each layer restores focus to where it was before THAT layer opened,
            // not back to the pre-outermost-modal element.
            if (!window.__dxOverlayFocusStack) window.__dxOverlayFocusStack = [];

            const topId = await dioxus.recv();

            if (topId) {
                // Poll briefly for createFocusTrap in case the script is still
                // parsing (avoids the deferred-load race on fast opens).
                let attempts = 0;
                while (!window.createFocusTrap && attempts < 10) {
                    await new Promise(r => setTimeout(r, 20));
                    attempts++;
                }

                // Push the current focused element so we can restore it later.
                window.__dxOverlayFocusStack.push(document.activeElement);

                // Remove any existing traps that are not the top one.
                document.querySelectorAll('[data-overlay-trap="1"]').forEach(el => {
                    if (el.id !== topId && el.trap) {
                        el.trap.remove();
                        el.trap = null;
                        el.removeAttribute('data-overlay-trap');
                    }
                });
                const el = document.getElementById(topId);
                if (el && !el.trap && window.createFocusTrap) {
                    el.trap = window.createFocusTrap(el);
                    el.setAttribute('data-overlay-trap', '1');
                }
            } else {
                // No modal: tear down all traps and restore focus to the element
                // that was focused before this layer was opened.
                document.querySelectorAll('[data-overlay-trap="1"]').forEach(el => {
                    if (el.trap) { el.trap.remove(); el.trap = null; }
                    el.removeAttribute('data-overlay-trap');
                });
                const ret = (window.__dxOverlayFocusStack || []).pop();
                if (ret && typeof ret.focus === 'function') { ret.focus(); }
            }

            // Wait for teardown signal (sent by cleanup closure). When received,
            // this eval context exits cleanly, preventing accumulation.
            await dioxus.recv();
            "#,
        );
        let _ = eval.send(top);
        move || {
            // Signal teardown: unblocks the second `await dioxus.recv()` so the
            // eval exits, removing this eval context from the active set.
            let _ = eval.send(true);
        }
    });
}

#[cfg(test)]
mod tests {
    //! R-CTX smoke proof (plan §4.2 + Wave-0 acceptance).
    //!
    //! Proves end-to-end at run time (via SSR, no DOM required): an overlay
    //! registers with the manager, renders its content through the shared portal
    //! outlet at the top of the document, **re-provides a context inside the
    //! portaled subtree**, and a child consumes it through the *portaled* chain
    //! (not the definition chain). Also proves the manager assigns `--overlay-z`.
    use super::*;
    use crate::portal::{use_portal, PortalIn};

    /// A trivial context re-provided inside the portaled subtree.
    #[derive(Clone, Copy, PartialEq)]
    struct SmokeCtx {
        marker: Signal<&'static str>,
    }

    /// Consumes `SmokeCtx` up the *portaled* render chain and renders its marker.
    #[component]
    fn SmokeChild() -> Element {
        let ctx = use_context::<SmokeCtx>();
        rsx! {
            span { class: "smoke-consumed", "{(ctx.marker)()}" }
        }
    }

    /// A test overlay: registers a `Modal`, portals content to the outlet, and
    /// re-provides `SmokeCtx` at the top of the portaled subtree.
    #[component]
    fn SmokeOverlay() -> Element {
        let portal = use_portal();
        let on_dismiss = use_callback(|_| {});
        let reg = use_overlay_registration(move || RegisterArgs {
            kind: OverlayKind::Modal,
            portal,
            modal: true,
            dismissable: true,
            on_dismiss,
            parent: None,
            trigger_id: None,
            content_root_id: Some("smoke-content".to_string()),
            stack_key: None,
        });

        rsx! {
            PortalIn { portal,
                SmokePortaledRoot { reg }
            }
        }
    }

    /// The top of the portaled subtree: re-provides `SmokeCtx` so a portaled
    /// consumer resolves it, and sets `--overlay-z` from the manager.
    #[component]
    fn SmokePortaledRoot(reg: OverlayRegistration) -> Element {
        // Re-provide the ctx INSIDE the portal (the load-bearing rule).
        use_context_provider(|| SmokeCtx {
            marker: Signal::new("inside-portal"),
        });
        let z = reg.z().unwrap_or_default();
        rsx! {
            div {
                id: "smoke-content",
                style: "--overlay-z: {z}",
                SmokeChild {}
            }
        }
    }

    /// Provides a *different* `SmokeCtx` ABOVE the overlay (outside the portal)
    /// to prove it is NOT what the portaled consumer resolves.
    #[component]
    fn SmokeApp() -> Element {
        rsx! {
            OverlayProvider {
                // This ctx sits above the registering overlay in the definition
                // tree. Because context does NOT inherit through the portal, the
                // portaled `SmokeChild` must NOT see this value.
                SmokeAboveProvider {
                    SmokeOverlay {}
                }
            }
        }
    }

    #[component]
    fn SmokeAboveProvider(children: Element) -> Element {
        use_context_provider(|| SmokeCtx {
            marker: Signal::new("above-portal-should-not-leak"),
        });
        rsx! { {children} }
    }

    #[test]
    fn overlay_portals_and_re_provides_context() {
        let mut dom = VirtualDom::new(SmokeApp);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        // The portaled content rendered through the outlet.
        assert!(
            html.contains("smoke-content"),
            "portaled content not rendered via outlet: {html}"
        );
        // The consumer resolved the RE-PROVIDED (inside-portal) ctx.
        assert!(
            html.contains("inside-portal"),
            "portaled consumer did not resolve re-provided ctx: {html}"
        );
        // It did NOT resolve the ctx provided ABOVE the overlay (proving context
        // does not inherit through the portal — the §4.2 load-bearing fact).
        assert!(
            !html.contains("above-portal-should-not-leak"),
            "portaled consumer leaked the definition-tree ctx: {html}"
        );
        // The manager assigned an `--overlay-z` built from the z-scale vars.
        assert!(
            html.contains("--overlay-z: calc(var(--z-overlay-base)"),
            "manager did not assign --overlay-z: {html}"
        );
    }
}
