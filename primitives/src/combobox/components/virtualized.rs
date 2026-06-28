//! Virtualized combobox listbox component.

use std::rc::Rc;

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{
    dioxus_attributes::attributes,
    floating::{style_prop, use_position},
    listbox::{use_listbox_container_with_open, use_listbox_id, ListboxContext},
    merge_attributes,
    overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs},
    portal::{use_portal, PortalIn},
    ContentAlign, ContentSide,
};

/// Props for [`VirtualizedComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
pub struct VirtualizedComboboxOptionsProps {
    /// The total number of options.
    pub count: ReadSignal<usize>,

    /// Optional visible-row to absolute-option index mapping.
    ///
    /// When provided, the virtualizer only materializes the mapped rows and passes the underlying
    /// absolute option index into [`Self::render_option`] and [`Self::estimate_size`].
    #[props(default)]
    pub visible_indices: Option<ReadSignal<Vec<usize>>>,

    /// The amount of render buffer in estimated row counts.
    #[props(default = ReadSignal::new(Signal::new(8)))]
    pub buffer: ReadSignal<usize>,

    /// Estimates the height of an option by absolute index.
    pub estimate_size: Option<Callback<usize, u32>>,

    /// Renders one option by absolute index.
    pub render_option: Callback<usize, Element>,

    /// Optional id for the listbox.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the listbox scroll container.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A virtualized combobox listbox that preserves listbox/option semantics.
///
/// Root-tree half: provides `ListboxContext` and portals the open listbox through
/// the overlay manager. The virtualization state + DOM live in
/// [`VirtualizedComboboxOptionsRendered`], the portaled body.
#[component]
pub fn VirtualizedComboboxOptions(props: VirtualizedComboboxOptionsProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());
    let id = use_listbox_id(props.id, ctx.selectable.list_id);
    let listbox = use_listbox_container_with_open(id, ctx.selectable, open);
    let render = listbox.render;
    let listbox_ctx: ListboxContext = use_context();

    let portal = use_portal();
    let mut ctx_dismiss = ctx;
    let on_dismiss = use_callback(move |_| {
        ctx_dismiss.set_open(false);
    });

    if !render() {
        return rsx! {};
    }

    rsx! {
        VirtualizedComboboxOptionsPortaled {
            ctx,
            listbox_ctx,
            id: listbox.id,
            open,
            portal,
            on_dismiss,
            props,
        }
    }
}

/// Props for [`VirtualizedComboboxOptionsPortaled`], the in-portal registration
/// + portal-mount half.
#[derive(Props, Clone, PartialEq)]
struct VirtualizedComboboxOptionsPortaledProps {
    ctx: ComboboxContext,
    listbox_ctx: ListboxContext,
    id: Memo<String>,
    open: Memo<bool>,
    portal: crate::portal::PortalId,
    on_dismiss: Callback<()>,
    props: VirtualizedComboboxOptionsProps,
}

/// Registers the virtualized listbox as a Floating overlay entry and renders it
/// through the shared outlet.
#[component]
fn VirtualizedComboboxOptionsPortaled(props: VirtualizedComboboxOptionsPortaledProps) -> Element {
    let id = props.id;
    let open = props.open;
    let portal = props.portal;
    let on_dismiss = props.on_dismiss;

    let reg: OverlayRegistration = use_overlay_registration(move || RegisterArgs {
        kind: OverlayKind::Floating,
        portal,
        modal: false,
        dismissable: true,
        on_dismiss,
        parent: None,
        trigger_id: None,
        content_root_id: Some(id.peek().clone()),
    });

    use_effect(move || {
        reg.set_dom_ids(None, Some(id()));
    });
    use_effect(move || {
        reg.set_closing(!open());
    });

    // Subscribe to `open` HERE, in the non-portaled (Root-descendant) scope, and
    // forward the snapshot into the portaled body as a plain bool so the body
    // never reads the Root-owned store across the portal boundary.
    let is_open = open();

    rsx! {
        PortalIn { portal,
            VirtualizedComboboxOptionsRendered {
                ctx: props.ctx,
                listbox_ctx: props.listbox_ctx,
                reg,
                id,
                is_open,
                inner: props.props,
            }
        }
    }
}

/// A tiny `Copy` handle exposing the listbox id, mirroring the field access
/// (`listbox.id`) the virtualization body uses for scroll helpers.
#[derive(Clone, Copy)]
struct ListboxIdHandle {
    id: Memo<String>,
}

/// Props for [`VirtualizedComboboxOptionsRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct VirtualizedComboboxOptionsRenderedProps {
    ctx: ComboboxContext,
    listbox_ctx: ListboxContext,
    reg: OverlayRegistration,
    id: Memo<String>,
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    inner: VirtualizedComboboxOptionsProps,
}

/// The portaled DOM body: re-provides `ComboboxContext` + `ListboxContext` and
/// owns all virtualization state. Emits `--overlay-z`. The listbox container hook
/// stays in the Root-tree wrapper — here we read the re-provided `ListboxContext`
/// and the shared listbox id directly so option-state registration is not
/// duplicated.
#[component]
fn VirtualizedComboboxOptionsRendered(props: VirtualizedComboboxOptionsRenderedProps) -> Element {
    let ctx = props.ctx;
    let reg = props.reg;
    let id = props.id;
    let is_open = props.is_open;
    let listbox_ctx = props.listbox_ctx;
    // Re-provide both contexts INSIDE the portal (the load-bearing rule).
    use_context_provider(|| ctx);
    use_context_provider(|| listbox_ctx);

    // Reconstruct the original prop bindings used throughout the body.
    let props = props.inner;
    let render = listbox_ctx.render;
    // Local handle mirroring the old `listbox.id` usage in scroll helpers.
    let listbox = ListboxIdHandle { id };

    let mut scroll_offset = use_signal(|| 0u32);
    let mut viewport_size = use_signal(|| 0u32);

    // Floating-element positioning (mirrors `ComboboxOptions`). The reference is the
    // combobox target registered in the store; the floating element is this listbox
    // scroll container.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let pos = use_position(
        ctx.store.target_mount_ref(),
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

    // The total number of visible rows (changes when the filter changes).
    let visible_count = use_memo(move || {
        props
            .visible_indices
            .as_ref()
            .map(|indices| indices.read().len())
            .unwrap_or_else(|| (props.count)())
    });

    // The single row-height estimate. We call estimate_size(0) as a representative sample.
    // For comboboxes all options are the same height, so this is exact.
    let est = use_memo(move || {
        props
            .estimate_size
            .as_ref()
            .map(|cb| {
                let idx = props
                    .visible_indices
                    .as_ref()
                    .and_then(|s| s.read().first().copied())
                    .unwrap_or(0);
                cb(idx).max(1)
            })
            .unwrap_or(36)
    });

    // Reset scroll position whenever the filter changes.
    use_effect(move || {
        let _ = visible_count.read();
        scroll_offset.set(0);
        spawn(async move {
            sync_scroll(listbox.id.peek().clone(), 0).await;
        });
    });

    // Read scroll position directly from the native scroll event — no JS eval loop needed.
    // ScrollData carries scrollTop and clientHeight from the browser event itself.
    let on_scroll = move |evt: Event<ScrollData>| {
        let data = evt.data();
        scroll_offset.set(data.scroll_top().round() as u32);
        viewport_size.set(data.client_height() as u32);
    };

    // On mount, capture the initial viewport height so the window calculation is correct
    // before the first scroll event fires.
    let on_mounted = move |e: Event<MountedData>| {
        let data = e.data();
        floating_ref.set(Some(data.clone()));
        spawn(async move {
            if let Ok(rect) = data.get_client_rect().await {
                viewport_size.set(rect.size.height.round() as u32);
            }
        });
        // Ensure the signal state is clean for each fresh open.
        scroll_offset.set(0);
    };

    // Scroll-to highlighted option using pure estimate positions.
    use_effect(move || {
        if !render() {
            return;
        }
        let Some(highlighted_index) = ctx.store.highlighted_option_index() else {
            return;
        };
        let visible_index = if let Some(indices) = props.visible_indices.as_ref() {
            let indices = indices.read();
            let Some(pos) = indices.iter().position(|&i| i == highlighted_index) else {
                return;
            };
            pos
        } else {
            highlighted_index
        };
        let count = *visible_count.peek();
        if visible_index >= count {
            return;
        }
        let e = *est.peek();
        let item_start = visible_index as u32 * e;
        let item_end = item_start + e;
        let current = *scroll_offset.peek();
        let vp = *viewport_size.peek();
        let next = if item_start < current {
            Some(item_start)
        } else if item_end > current.saturating_add(vp) {
            Some(item_end.saturating_sub(vp))
        } else {
            None
        };
        if let Some(next) = next {
            scroll_offset.set(next);
            spawn(async move {
                sync_scroll(listbox.id.peek().clone(), next).await;
            });
        }
    });

    // ── Window calculation ────────────────────────────────────────────────────
    //
    // The number of rendered DOM nodes MUST be stable during scroll. If it
    // varies, Dioxus mounts/unmounts elements, which triggers browser layout
    // recalculation and temporarily changes scrollHeight — making the thumb
    // jump in size and position.
    //
    // Strategy (same as react-window / TanStack Virtual):
    //   1. Compute `window_size` = rows_that_fit_in_viewport + 2 × buffer.
    //      This value is constant for a given viewport height.
    //   2. Clamp `start` so that `start + window_size ≤ count`. This means
    //      near the end of the list we shift the window backward rather than
    //      letting it shrink — keeping the count fixed.
    //   3. Each item is `position: absolute; transform: translateY(index * est)`.
    //      Items are NOT in normal document flow, so the canvas div's intrinsic
    //      height is zero — only the explicit `height: Xpx` CSS matters.
    //      `overflow: hidden` ensures no item can poke outside the canvas.

    let off = *scroll_offset.read();
    let vp = *viewport_size.read();
    let e = *est.read();
    let count = *visible_count.read();
    let buf = (props.buffer)();
    let e1 = e.max(1);

    // How many rows can the viewport hold? Use 240px as a stand-in before the
    // first scroll event so the initial render is already fully populated.
    let viewport_rows = if vp == 0 { 240 } else { vp };

    // Fixed pool size — constant as long as viewport and buffer don't change.
    let window_size = ((viewport_rows / e1) as usize + 2 * buf + 1).min(count);

    // Desired first visible row.
    let desired_start = (off / e1).saturating_sub(buf as u32) as usize;

    // Clamp so we always emit exactly `window_size` items. At the bottom of
    // the list this shifts the window backward instead of shrinking it.
    let start = desired_start.min(count.saturating_sub(window_size));

    // canvas_height = count × est. Fixed. Never changes during scroll.
    let canvas_height = (count as u32 * e1).max(vp);
    let set_size = count.to_string();

    // z-index assigned by the overlay manager via open order.
    let z_style = reg.z().map(|z| format!("--overlay-z: {z};"));

    let floating_attrs = attributes!(div {
        position: position(),
        top: top(),
        left: left(),
        visibility: visibility(),
        style: z_style,
        "data-floating": floating_active.then_some("true"),
        "data-side": resolved_side.read().as_str(),
        "data-align": resolved_align.read().as_str(),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        if render() {
            div {
                id: listbox.id,
                role: "listbox",
                "data-state": if is_open { "open" } else { "closed" },
                onmounted: on_mounted,
                onscroll: on_scroll,
                onpointerdown: move |event| {
                    event.prevent_default();
                },
                ..attributes,
                // Canvas: flex-shrink:0 is critical — the listbox is a flex column container,
                // and without it the browser compresses this div to fit the max-height,
                // eliminating overflow and making the list unscrollable.
                div { style: "position: relative; overflow: hidden; flex-shrink: 0; height: {canvas_height}px; width: 100%;",
                    {
                        (start..start + window_size)
                            .map(move |visible_index| {
                                let index = props
                                    .visible_indices
                                    .as_ref()
                                    .map(|indices| indices.read().get(visible_index).copied())
                                    .unwrap_or_else(|| {
                                        (visible_index < count).then_some(visible_index)
                                    });
                                let item_top = visible_index as u32 * e1;
                                rsx! {
                                    div {
                                        key: "{visible_index}",
                                        role: "presentation",
                                        style: "position: absolute; top: 0; left: 0; width: 100%; transform: translateY({item_top}px);",
                                        "data-virtual-index": "{visible_index}",
                                        "aria-setsize": "{set_size}",
                                        "aria-posinset": "{visible_index + 1}",
                                        {index.map(|i| (props.render_option)(i))}
                                    }
                                }
                            })
                    }
                }
            }
        } else {

        }
    }
}

async fn sync_scroll(container_id: String, scroll_top: u32) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const scrollTop = await dioxus.recv();
        const container = document.getElementById(id);
        if (container) container.scrollTop = scrollTop;
        "#,
    );
    let _ = eval.send(container_id);
    let _ = eval.send(scroll_top);
}
