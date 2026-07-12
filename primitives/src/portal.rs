use crate::dioxus_core::provide_root_context;
use dioxus::prelude::*;
use std::collections::HashMap;

use crate::use_effect_cleanup;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortalId {
    id: usize,
    content: Signal<Element>,
    visible: Signal<bool>,
}

#[derive(Clone, Copy, PartialEq)]
struct PortalCtx {
    portals: Signal<HashMap<usize, Signal<Element>>>,
}

/// Create a portal.
pub fn use_portal() -> PortalId {
    static NEXT_ID: GlobalSignal<usize> = Signal::global(|| 0);

    let (sig, id) = use_hook(|| {
        let mut next_id = NEXT_ID.write();
        let id = *next_id;
        *next_id += 1;

        let mut ctx = match try_consume_context::<PortalCtx>() {
            Some(ctx) => ctx,
            None => {
                let portals = Signal::new_in_scope(HashMap::new(), ScopeId::ROOT);
                let ctx = PortalCtx { portals };
                provide_root_context(ctx)
            }
        };

        let sig = Signal::new_in_scope(VNode::empty(), ScopeId::ROOT);
        let visible = Signal::new_in_scope(false, ScopeId::ROOT);
        ctx.portals.write().insert(id, sig);

        (
            sig,
            PortalId {
                id,
                content: sig,
                visible,
            },
        )
    });

    // Cleanup the portal.
    use_effect_cleanup(move || {
        let mut ctx = consume_context::<PortalCtx>();
        ctx.portals.write().remove(&id.id);
        sig.manually_drop();
    });

    id
}

#[component]
pub fn PortalIn(portal: PortalId, children: Element) -> Element {
    if let Some(mut ctx) = try_use_context::<PortalCtx>() {
        let mut portals = ctx.portals.write();
        let _ = portals.get_mut(&portal.id);
        let mut content = portal.content;
        content.set(children);
        let mut visible = portal.visible;
        visible.set(true);
    }

    rsx! {}
}

#[component]
pub fn PortalOut(portal: PortalId) -> Element {
    if !(portal.visible)() {
        return rsx! {};
    }
    if let Some(ctx) = try_use_context::<PortalCtx>() {
        let portals = (ctx.portals)();
        if let Some(children) = portals.get(&portal.id) {
            return rsx! {
                {children()}
            };
        }
    }

    rsx! {}
}
