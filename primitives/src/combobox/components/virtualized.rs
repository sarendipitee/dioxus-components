//! Virtualized combobox listbox component.

use std::rc::Rc;

use crate::dioxus_core::use_hook_with_cleanup;
use dioxus::prelude::*;

use super::super::{
    context::{
        ComboboxContext, ComboboxPortalContext, ComboboxPortalOptionRegistration,
    },
    hook::{ComboboxDropdownEventSource, VirtualizedComboboxNavigation},
};
use crate::{
    dioxus_attributes::attributes,
    floating::{style_prop, use_position},
    listbox::{use_listbox_container_with_open, use_listbox_id, ListboxContext},
    merge_attributes,
    overlay::{use_overlay_registration, OverlayKind, OverlayRegistration, RegisterArgs},
    portal::{use_portal, PortalIn},
    ContentAlign, ContentSide,
    selection::{remove_option, sync_option},
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
    let store = ctx.store;
    let _navigation = use_hook_with_cleanup(
        move || {
            let navigation = VirtualizedComboboxNavigation::new(props.count, props.visible_indices);
            store.register_virtualized_navigation(navigation);
            navigation
        },
        move |navigation| store.unregister_virtualized_navigation(navigation),
    );
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
        stack_key: None,
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
    let content_id = id.cloned();
    let overlay_z = reg.z();
    let should_render = (props.listbox_ctx.render)();
    let highlighted_index = props.ctx.store.highlighted_option_index();
    let root_disabled = props.ctx.selectable.disabled.cloned();
    let selected_values = props.ctx.selectable.values.read().clone();
    let focused_index = (props.ctx.selectable.focus_state.current_focus)();
    let options = props.ctx.selectable.options.read().clone();
    let visible_indices = props
        .props
        .visible_indices
        .as_ref()
        .map(|indices| indices.read().clone());
    let count = (props.props.count)();
    let visible_count = visible_indices.as_ref().map(Vec::len).unwrap_or(count);
    let initial_selection_index = props.ctx.store.pending_virtual_initial_selection();
    let buffer = (props.props.buffer)();
    let estimated_size = props
        .props
        .estimate_size
        .as_ref()
        .map(|cb| {
            let index = visible_indices
                .as_ref()
                .and_then(|indices| indices.first().copied())
                .unwrap_or(0);
            cb(index).max(1)
        })
        .unwrap_or(36);

    // Snapshot every floating-layout value HERE so the portaled body never reads
    // Root-owned position memos or the combobox store's target ref.
    let mut floating_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let on_floating_mounted = use_callback(move |mounted: Rc<MountedData>| {
        floating_ref.set(Some(mounted));
    });
    let pos = use_position(
        props.ctx.store.target_mount_ref(),
        floating_ref,
        ContentSide::Bottom,
        ContentAlign::Start,
    );
    let floating_style = pos.style.read().clone();
    let floating_position = style_prop(&floating_style, "position");
    let floating_top = style_prop(&floating_style, "top");
    let floating_left = style_prop(&floating_style, "left");
    let floating_visibility = if (pos.is_positioned)() {
        "visible".to_string()
    } else {
        "hidden".to_string()
    };
    let floating_side = *pos.side.read();
    let floating_align = *pos.align.read();
    let floating_active = pos.floating_active;
    let mut submit_ctx = props.ctx;
    let submit_index_from_mouse = use_callback(move |index: usize| {
        submit_ctx.submit_index(index, ComboboxDropdownEventSource::Mouse);
    });
    let mut hover_focus_state = props.ctx.selectable.focus_state;
    let hover_store = props.ctx.store;
    let hover_option = use_callback(move |index: usize| {
        hover_focus_state.set_focus(Some(index));
        hover_store.select_option(index);
    });
    let mut registration_selectable = props.ctx.selectable;
    let registration_store = props.ctx.store;
    let register_option = use_callback(move |registration: ComboboxPortalOptionRegistration| {
        let option = registration.option;
        let index = option.tab_index;
        let disabled = option.disabled;
        sync_option(registration_selectable.options, option.clone());
        registration_selectable
            .focus_state
            .add_update_item(index, disabled);
        registration_store.register_option(
            option.id,
            index,
            disabled,
            registration.visible,
            registration.selected,
        );
        if let Some(resolved) = registration_store.resolve_pending_initial_selection_at(index) {
            registration_selectable
                .focus_state
                .set_focus(Some(resolved.index));
        }
    });
    let mut unregister_selectable = props.ctx.selectable;
    let unregister_store = props.ctx.store;
    let unregister_option = use_callback(move |registration: ComboboxPortalOptionRegistration| {
        let option = registration.option;
        remove_option(unregister_selectable.options, &option.id);
        unregister_selectable.focus_state.remove_item(option.tab_index);
        unregister_store.unregister_option(&option.id);
    });

    rsx! {
        PortalIn { portal,
            VirtualizedComboboxOptionsRendered {
                is_open,
                id: content_id,
                overlay_z,
                should_render,
                portal_ctx: ComboboxPortalContext {
                    hover_option,
                    root_disabled,
                    selected_values,
                    focused_index,
                    highlighted_index,
                    options,
                    visible_indices: visible_indices.clone(),
                    has_visible_options: visible_count > 0,
                    register_options: true,
                    register_option,
                    unregister_option,
                    submit_index_from_mouse,
                },
                highlighted_index,
                initial_selection_index,
                visible_indices,
                visible_count,
                count,
                buffer,
                estimated_size,
                render_option: props.props.render_option,
                floating_position,
                floating_top,
                floating_left,
                floating_visibility,
                floating_side,
                floating_align,
                floating_active,
                on_floating_mounted,
                attributes: props.props.attributes.clone(),
            }
        }
    }
}

/// Props for [`VirtualizedComboboxOptionsRendered`], the portaled DOM body.
#[derive(Props, Clone, PartialEq)]
struct VirtualizedComboboxOptionsRenderedProps {
    /// Open snapshot threaded from the non-portaled parent — see the matching
    /// note on `DialogPortalBodyProps::is_open`.
    is_open: bool,
    /// Snapshotted in the non-portaled parent so the portaled body never reads
    /// the Root-owned listbox id memo across the portal boundary.
    id: String,
    /// Overlay z metadata, snapshotted in the non-portaled parent where the
    /// backing `OverlayRegistration` id signal is owned.
    overlay_z: Option<String>,
    /// `ListboxContext::render`, snapped before `PortalIn` so the body never
    /// reads the Root-owned render signal.
    should_render: bool,
    portal_ctx: ComboboxPortalContext,
    /// Virtual window inputs snapped before `PortalIn` so the body never reads
    /// Root-owned count/filter/highlight signals across the portal boundary.
    highlighted_index: Option<usize>,
    initial_selection_index: Option<usize>,
    visible_indices: Option<Vec<usize>>,
    visible_count: usize,
    count: usize,
    buffer: usize,
    estimated_size: u32,
    render_option: Callback<usize, Element>,
    /// Floating layout fields, snapshotted in the non-portaled parent so the
    /// portaled body never reads Root-owned positioning memos.
    floating_position: String,
    floating_top: String,
    floating_left: String,
    floating_visibility: String,
    floating_side: ContentSide,
    floating_align: ContentAlign,
    floating_active: bool,
    on_floating_mounted: Callback<Rc<MountedData>>,
    attributes: Vec<Attribute>,
}

/// The portaled DOM body: re-provides the portal combobox read model plus
/// `ListboxContext` and owns local scroll/viewport virtualization state. All
/// Root-owned listbox id, overlay z, floating layout, and filter/highlight
/// snapshots are forwarded as plain values from the non-portaled parent.
#[component]
fn VirtualizedComboboxOptionsRendered(props: VirtualizedComboboxOptionsRenderedProps) -> Element {
    let is_open = props.is_open;
    let should_render = props.should_render;
    let mut render_signal = use_signal(|| should_render);

    use_effect(move || {
        render_signal.set(should_render);
    });
    // Context providers initialize once. Keep a portal-local signal so every
    // root-side snapshot update reaches descendants after portal mounting.
    let mut portal_ctx = use_context_provider(|| Signal::new(props.portal_ctx.clone()));
    use_effect(use_reactive(&props.portal_ctx, move |next| {
        portal_ctx.set(next);
    }));
    use_context_provider(|| ListboxContext {
        render: render_signal.into(),
        id: props.id.clone(),
    });

    let mut scroll_offset = use_signal(|| 0u32);
    let mut viewport_size = use_signal(|| 0u32);
    let content_id = props.id.clone();
    let visible_indices = props.visible_indices.clone();
    let visible_count = props.visible_count;
    let count = visible_count;
    let estimated_size = props.estimated_size.max(1);
    let highlighted_index = props.highlighted_index;
    let initial_selection_index = props.initial_selection_index;
    let render_option = props.render_option;

    // Reset scroll position whenever the filter changes.
    use_effect(use_reactive(
        (&props.visible_indices, &props.count),
        move |_| {
            scroll_offset.set(0);
            let content_id = content_id.clone();
            spawn(async move {
                sync_scroll(content_id, 0).await;
            });
        },
    ));

    // Read scroll position directly from the native scroll event — no JS eval loop needed.
    // ScrollData carries scrollTop and clientHeight from the browser event itself.
    let on_scroll = move |evt: Event<ScrollData>| {
        let data = evt.data();
        scroll_offset.set(data.scroll_top().round() as u32);
        viewport_size.set(data.client_height() as u32);
    };

    // On mount, capture the initial viewport height so the window calculation is correct
    // before the first scroll event fires.
    let on_floating_mounted = props.on_floating_mounted;
    let on_mounted = move |e: Event<MountedData>| {
        let data = e.data();
        on_floating_mounted.call(data.clone());
        spawn(async move {
            if let Ok(rect) = data.get_client_rect().await {
                viewport_size.set(rect.size.height.round() as u32);
            }
        });
        // Ensure the signal state is clean for each fresh open.
        scroll_offset.set(0);
    };

    // Scroll-to highlighted option using pure estimate positions.
    let effect_visible_indices = visible_indices.clone();
    let effect_content_id = props.id.clone();
    use_effect(use_reactive(
        (
            &props.should_render,
            &props.highlighted_index,
            &props.visible_indices,
            &props.visible_count,
            &props.estimated_size,
        ),
        move |_| {
            if !should_render {
                return;
            }
            let Some(highlighted_index) = highlighted_index else {
                return;
            };
            let visible_index = if let Some(indices) = effect_visible_indices.as_ref() {
                let Some(pos) = indices.iter().position(|&i| i == highlighted_index) else {
                    return;
                };
                pos
            } else {
                highlighted_index
            };
            if visible_index >= visible_count {
                return;
            }
            let item_start = visible_index as u32 * estimated_size;
            let item_end = item_start + estimated_size;
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
                let content_id = effect_content_id.clone();
                spawn(async move {
                    sync_scroll(content_id, next).await;
                });
            }
        },
    ));

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
    let buf = props.buffer;
    let e1 = estimated_size;

    // How many rows can the viewport hold? Use 240px as a stand-in before the
    // first scroll event so the initial render is already fully populated.
    let viewport_rows = if vp == 0 { 240 } else { vp };

    // Fixed pool size — constant as long as viewport and buffer don't change.
    let window_size = ((viewport_rows / e1) as usize + 2 * buf + 1).min(count);

    // Desired first visible row.
    let scroll_start = (off / e1).saturating_sub(buf as u32) as usize;
    // Resolving a pending virtual keyboard selection clears its pending index
    // before the highlight-driven scroll effect can update `scroll_offset`.
    // Keep the highlighted row in the first post-resolution window until that
    // local scroll state catches up; otherwise the target unmounts and loses
    // its selection registration.
    let window_anchor =
        initial_selection_index.or_else(|| (off == 0).then_some(highlighted_index).flatten());
    let initial_start = window_anchor
        .and_then(|index| {
            visible_indices
                .as_ref()
                .and_then(|indices| indices.iter().position(|&candidate| candidate == index))
                .or((index < count).then_some(index))
        })
        .map(|index| index.saturating_sub(window_size.saturating_sub(1)));
    let desired_start = initial_start.unwrap_or(scroll_start);

    // Clamp so we always emit exactly `window_size` items. At the bottom of
    // the list this shifts the window backward instead of shrinking it.
    let start = desired_start.min(count.saturating_sub(window_size));

    // canvas_height = count × est. Fixed. Never changes during scroll.
    let canvas_height = (count as u32 * e1).max(vp);
    let set_size = count.to_string();

    let z_style = props
        .overlay_z
        .as_ref()
        .map(|z| format!("--overlay-z: {z};"));

    let floating_attrs = attributes!(div {
        position: props.floating_position.clone(),
        top: props.floating_top.clone(),
        left: props.floating_left.clone(),
        visibility: props.floating_visibility.clone(),
        style: z_style,
        "data-floating": props.floating_active.then_some("true"),
        "data-side": props.floating_side.as_str(),
        "data-align": props.floating_align.as_str(),
    });
    let attributes = merge_attributes(vec![props.attributes.clone(), floating_attrs]);

    rsx! {
        if should_render {
            div {
                id: props.id.clone(),
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
                                let index = visible_indices
                                    .as_ref()
                                    .and_then(|indices| indices.get(visible_index).copied())
                                    .unwrap_or(visible_index);
                                let item_top = visible_index as u32 * e1;
                                rsx! {
                                    div {
                                        key: "{visible_index}",
                                        role: "presentation",
                                        style: "position: absolute; top: 0; left: 0; width: 100%; transform: translateY({item_top}px);",
                                        "data-virtual-index": "{visible_index}",
                                        "aria-setsize": "{set_size}",
                                        "aria-posinset": "{visible_index + 1}",
                                        {render_option.call(index)}
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

#[cfg(test)]
mod tests {
    use super::super::{
        Combobox, ComboboxInput, ComboboxOption, VirtualizedComboboxOptions,
    };
    use crate::overlay::OverlayProvider;
    use dioxus::prelude::*;


    #[component]
    fn OpenVirtualizedHoverComboboxApp() -> Element {
        let count = ReadSignal::new(Signal::new(1));
        let render_option = use_callback(move |index: usize| {
            rsx! {
                ComboboxOption::<String> {
                    index,
                    value: "apple",
                    "Apple"
                }
            }
        });

        rsx! {
            OverlayProvider {
                Combobox::<String> {
                    default_open: ReadSignal::new(Signal::new(true)),
                    ComboboxInput {}
                    VirtualizedComboboxOptions {
                        count,
                        render_option,
                    }
                }
            }
        }
    }

    #[test]
    fn portaled_virtualized_option_renders_mouseenter_hover_handler() {
        let mut dom = VirtualDom::new(OpenVirtualizedHoverComboboxApp);
        let mut mutations = dom.rebuild_to_vec();
        for _ in 0..8 {
            mutations.edits.extend(dom.render_immediate_to_vec().edits);
        }

        assert!(
            mutations.edits.iter().any(|mutation| matches!(
                mutation,
                dioxus::core::Mutation::NewEventListener { name, .. } if name == "mouseenter"
            )),
            "virtualized portaled option did not register its mouseenter hover handler"
        );
    }
}
